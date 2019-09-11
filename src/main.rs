#![warn(clippy::all)]

use log::*;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod models;

#[derive(derive_more::From, Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    MaximumRetries,
    CouldntFindChannelId,
    CouldntFindChannel,
    BadScript,
}

/// random(0, 2**n - 1) * base
fn exponential_jitter(base: u64) -> impl Iterator<Item = u64> {
    let mut rng = rand::thread_rng();
    (1..)
        .map(|x| (1 << x) - 1)
        .map(move |slots| base * Uniform::new_inclusive(0, slots).sample(&mut rng))
}

pub type Result<T> = std::result::Result<T, Error>;

struct Client {
    inner: reqwest::Client,
    client_id: String,
}

impl Client {
    fn new(client_id: impl Into<String>) -> Self {
        Self {
            inner: reqwest::Client::new(),
            client_id: client_id.into(),
        }
    }

    fn fetch(&mut self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let request = request
            .header(reqwest::header::ACCEPT, "application/vnd.twitchtv.v5+json")
            .header("client-id", &self.client_id);
        debug!("{:#?}", request);

        // Maximum of 5 seconds .min(n)
        // ~Maximum of 5 retries .take(n)~ Retry forever.
        // 100 ms base for the jitter
        // for delay_ms in exponential_jitter(100).map(|x| x.min(1000 * 5)).take(5) {
        for delay_ms in exponential_jitter(100).map(|x| x.min(1000 * 5)) {
            match request.try_clone().expect("Failed to clone request").send() {
                Err(ref err) if err.status().is_none() => {
                    error!("Failed to fetch: retrying in {} ms", delay_ms);
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                Err(ref err) if err.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                    panic!("Invalid authorization");
                    // unimplemented!("Reauthorization needs to be implemented.");
                }
                value => {
                    return Ok(value?);
                }
            }
        }
        Err(Error::MaximumRetries)
    }

    fn fetch_channel_id(&mut self, channel_name: &str) -> Result<i64> {
        let result: models::search::channels::Root = self
            .fetch(
                self.inner
                    .get("https://api.twitch.tv/kraken/search/channels")
                    .query(&[("query", channel_name)]),
            )?
            .json()?;
        debug!("{:#?}", result);
        for channel in result.channels {
            debug!("{}", channel.name);
            if channel.name == channel_name {
                return Ok(channel.id);
            }
        }
        return Err(Error::CouldntFindChannelId);
    }

    fn fetch_channel(&mut self, channel_name: &str) -> Result<models::search::channels::Channel> {
        let result: models::search::channels::Root = self
            .fetch(
                self.inner
                    .get("https://api.twitch.tv/kraken/search/channels")
                    .query(&[("query", channel_name)]),
            )?
            .json()?;
        debug!("{:#?}", result);
        for channel in result.channels {
            debug!("{}", channel.name);
            if channel.name == channel_name {
                return Ok(channel);
            }
        }
        return Err(Error::CouldntFindChannel);
    }

    fn fetch_live_streams(&mut self, channel_ids: &[i64]) -> Result<models::streams::query::Root> {
        Ok(self
            .fetch(
                self.inner
                    .get("https://api.twitch.tv/kraken/streams/")
                    .query(&[(
                        "channel",
                        channel_ids
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .as_slice()
                            .join(","),
                    )]),
            )?
            .json()?)
    }

    // fn fetch_videos(&mut self, channel_id: &str) -> Result<i64> {
    //     let result: models::search::channels::Root = self
    //         .fetch(
    //             self.inner
    //                 .get(&format!(
    //                     "https://api.twitch.tv/kraken/channels/{}/videos",
    //                     channel_id
    //                 ))
    //                 .query(&[("limit", 100, "offset", 100 * i)]),
    //         )?
    //         .json()?;
    //     info!("{:#?}", result);
    //     for channel in result.channels {
    //         info!("{}", channel.name);
    //         if channel.name == channel_name {
    //             return Ok(channel.id);
    //         }
    //     }
    //     return Err(Error::CouldntFindChannelId);
    // }
}

/// Program to poll twitch via its API and download streams from channels as they come live.
///
/// Use RUST_LOG to set logging level.
/// e.g. export RUST_LOG='debug' or export RUST_LOG='twitch_scraper=info'
#[derive(StructOpt)]
struct Opt {
    /// List of channel names to poll.
    channel_names: Vec<String>,

    /// Twitch client id
    #[structopt(short, long)]
    client_id: String,

    /// Maximum milliseconds to wait before polling again
    #[structopt(long, default_value = "3000")]
    delay_max: u64,

    /// Minimum milliseconds to wait before polling again
    #[structopt(long, default_value = "100")]
    delay_min: u64,

    /// Directory to save videos.
    #[structopt(short, long, parse(from_os_str))]
    directory: PathBuf,

    /// Directory to save videos.
    ///
    /// See `man youtube-dl` under OUTPUT TEMPLATE for variables to use.
    ///
    /// Useful variables:
    ///
    /// - %(uploader)s: channel name
    ///
    /// - %(description)s: channel status/title
    ///
    /// - %(timestamp)s
    ///
    /// - %(title)s: for a live stream, looks like 'ashkankiani 2019-09-06 14_19'
    ///
    /// I personally use "%(uploader)s/%(title)s-%(description)s-%(id)s.%(ext)s"
    #[structopt(short = "o", long, default_value = "%(title)s-%(id)s.%(ext)s")]
    filename_template: String,

    /// Extra args to pass to youtube-dl.
    ///
    /// Current arguments are: --write-info-json --hls-use-mpegts --no-part --netrc
    #[structopt(short, long)]
    additional_args: Vec<String>,

    /// A script to execute when the stream comes live.
    ///
    /// These environment variables will be set:
    ///
    /// - TWITCH_CHANNEL_NAME
    ///
    /// - TWITCH_CHANNEL_ID
    ///
    /// - TWITCH_CHANNEL_STATUS
    ///
    /// - TWITCH_STREAM_ID
    ///
    /// - TWITCH_STREAM_CREATED_AT
    ///
    /// - YOUTUBE_DL_PID: The pid of the child process launched with youtube-dl
    ///
    // /// TWITCH_STREAM_TITLE
    #[structopt(short = "x", long, parse(from_os_str))]
    script: Option<PathBuf>,

    /// Quiet output for youtube-dl.
    ///
    /// Shortcut for --additional_args=-q.
    #[structopt(short, long)]
    quiet: bool,
}

fn download(
    directory: &PathBuf,
    filename_template: &str,
    channel_name: &str,
    extra: &[String],
) -> Result<std::process::Child> {
    let mut cmd = std::process::Command::new("youtube-dl");
    cmd.arg("--write-info-json")
        .arg("--hls-use-mpegts")
        .arg("--no-part")
        .arg("--netrc")
        .arg("--output")
        .arg(filename_template)
        .arg(format!("https://www.twitch.tv/{}", channel_name))
        .current_dir(directory);
    for extra_arg in extra {
        cmd.arg(&extra_arg);
    }
    Ok(cmd.spawn()?)
}

fn main() -> Result<()> {
    env_logger::init();
    let mut opt = Opt::from_args();
    let mut client = Client::new(opt.client_id.clone());
    if let Some(ref script) = opt.script {
        if !script.is_file() {
            error!("Script isn't a file {:?}", script);
            return Err(Error::BadScript);
        }
    }

    if opt.quiet {
        opt.additional_args.push("-q".into());
    }

    let channel_ids: Vec<_> = opt
        .channel_names
        .iter()
        .flat_map(|name| match client.fetch_channel_id(&name) {
            Ok(id) => {
                info!("Watching {} with id {}", name, id);
                Some(id)
            }
            Err(err) => {
                error!("Failed to fetch id for {}: {:?}", name, err);
                None
            }
        })
        .collect();

    // let mut in_progress = Arc::new(Mutex::new(HashMap::new()));
    let mut in_progress = HashMap::new();
    let mut rng = rand::thread_rng();
    for delay_ms in std::iter::repeat_with(|| {
        Uniform::new_inclusive(opt.delay_min, opt.delay_max).sample(&mut rng)
    }) {
        for stream in client
            .fetch_live_streams(channel_ids.as_slice())
            // Sometimes Twitch sends malformed responses, so in case
            // JSON decoding fails, just use an empty default.
            .unwrap_or_else(|_| Default::default())
            .streams
        {
            in_progress.entry(stream.id).or_insert_with(|| {
                info!(
                    "Downloading stream {} from {}",
                    stream.id, stream.channel.name
                );
                let child = download(
                    &opt.directory,
                    &opt.filename_template,
                    &stream.channel.name,
                    opt.additional_args.as_slice(),
                )
                .expect("Failed to launch child");
                if let Some(ref script) = opt.script {
                    if let Err(err) = std::process::Command::new(script)
                        .env("TWITCH_CHANNEL_ID", stream.channel.id.to_string())
                        .env("TWITCH_CHANNEL_NAME", &stream.channel.name)
                        .env(
                            "TWITCH_CHANNEL_STATUS",
                            &stream.channel.status.unwrap_or_else(String::new),
                        )
                        .env("TWITCH_STREAM_ID", stream.id.to_string())
                        .env("TWITCH_STREAM_CREATED_AT", &stream.created_at)
                        .env("YOUTUBE_DL_PID", child.id().to_string())
                        .spawn()
                    {
                        error!("Failed to launch script {:?}: {:#?}", script, err);
                    }
                }
                child
            });
        }
        thread::sleep(Duration::from_millis(delay_ms));
    }

    Ok(())
}
