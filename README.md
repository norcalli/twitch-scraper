```
$$$$$$$$\ $$\      $$\ $$$$$$\ $$$$$$$$\  $$$$$$\  $$\   $$\
\__$$  __|$$ | $\  $$ |\_$$  _|\__$$  __|$$  __$$\ $$ |  $$ |
   $$ |   $$ |$$$\ $$ |  $$ |     $$ |   $$ /  \__|$$ |  $$ |
   $$ |   $$ $$ $$\$$ |  $$ |     $$ |   $$ |      $$$$$$$$ |
   $$ |   $$$$  _$$$$ |  $$ |     $$ |   $$ |      $$  __$$ |
   $$ |   $$$  / \$$$ |  $$ |     $$ |   $$ |  $$\ $$ |  $$ |
   $$ |   $$  /   \$$ |$$$$$$\    $$ |   \$$$$$$  |$$ |  $$ |
   \__|   \__/     \__|\______|   \__|    \______/ \__|  \__|
 $$$$$$\   $$$$$$\  $$$$$$$\   $$$$$$\  $$$$$$$\  $$$$$$$$\ $$$$$$$\
$$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\ $$  _____|$$  __$$\
$$ /  \__|$$ /  \__|$$ |  $$ |$$ /  $$ |$$ |  $$ |$$ |      $$ |  $$ |
\$$$$$$\  $$ |      $$$$$$$  |$$$$$$$$ |$$$$$$$  |$$$$$\    $$$$$$$  |
 \____$$\ $$ |      $$  __$$< $$  __$$ |$$  ____/ $$  __|   $$  __$$<
$$\   $$ |$$ |  $$\ $$ |  $$ |$$ |  $$ |$$ |      $$ |      $$ |  $$ |
\$$$$$$  |\$$$$$$  |$$ |  $$ |$$ |  $$ |$$ |      $$$$$$$$\ $$ |  $$ |
 \______/  \______/ \__|  \__|\__|  \__|\__|      \________|\__|  \__|
```


```sh
❯ cargo install twitch-scraper # Install via cargo
    Updating crates.io index
    Downloaded twitch-scraper v0.1.3
    ...
```

```sh
❯ curl -OL https://github.com/norcalli/twitch-scraper/releases/download/v0.1.3/twitch-scraper \
    && chmod +x twitch-scraper # Or download a prebuilt static binary
```

```sh
❯ twitch-scraper \
    -q \
    -c $TWITCH_CLIENT_ID \
    -t $TWITCH_CLIENT_TOKEN \
    -d /videos/twitch/ \
    -o "%(channel)s/%(title)s-%(id)s.%(ext)s" \
    -x $PWD/stream-went-live.sh \
    ashkankiani naysayer88 demolition_d studio_trigger
    # Example session
    [2019-09-12T16:26:47Z INFO  twitch_scraper] Watching ["demolition_d", "naysayer88", "ashkankiani", "studio_trigger"]
    [2019-09-12T16:34:45Z INFO  twitch_scraper] Downloading stream 35633084336 from ashkankiani
    [2019-09-12T16:36:21Z INFO  twitch_scraper] Finished downloading stream 35633084336
    [2019-09-13T10:01:47Z INFO  twitch_scraper] Downloading stream 35628821840 from naysayer88
    ERROR: ffmpeg exited with code 255
    [2019-09-13T10:02:19Z ERROR twitch_scraper] Downloading of stream 35628821840 from naysayer88 failed with status ExitStatus(ExitStatus(256))
    [2019-09-13T10:02:19Z INFO  twitch_scraper] Downloading stream 35628821840 from naysayer88
    [2019-09-13T10:02:49Z ERROR twitch_scraper] Downloading of stream 35628821840 from naysayer88 failed with status ExitStatus(ExitStatus(15))
    [2019-09-13T10:02:50Z INFO  twitch_scraper] Downloading stream 35628821840 from naysayer88
    [2019-09-13T12:00:47Z INFO  twitch_scraper] Finished downloading stream 35628821840 from naysayer88
```

```sh
export TWITCH_CLIENT_TOKEN=$(curl -s -X POST \
    "https://id.twitch.tv/oauth2/token?client_id=$TWITCH_CLIENT_ID&client_secret=$TWITCH_CLIENT_SECRET&grant_type=client_credentials" |
    jq -r '.access_token'
)
```


```sh
❯ cat example_scripts/stream-went-live.sh
#!/bin/sh
exec notify-send TWITCH "$TWITCH_CHANNEL_NAME went live at $TWITCH_STREAM_CREATED_AT!
Downloading $TWITCH_STREAM_ID"
```

```sh
❯ twitch-scraper --help
twitch-scraper 0.1.3
Program to poll twitch via its API and download streams from channels as they come live.

Fault tolerance:
 - It will retry requests to Twitch API with exponential jitter backoff up to 5s between retries.
 - It will handle API limit rating.
 - It will try to restart the download in case youtube-dl fails prematurely.

Sending SIGINT will kill all running downloads.
 Sending SIGTERM will keep the downloads running.

Use RUST_LOG to set logging level.
 e.g. export RUST_LOG='debug' or export RUST_LOG='twitch_scraper=info'

USAGE:
    twitch-scraper [FLAGS] [OPTIONS] --client-id <client-id> --directory <directory> [--] [channel-names]...

FLAGS:
    -h, --help
            Prints help information

    -q, --quiet
            Quiet output for youtube-dl.

            Shortcut for --additional_args=-q.
    -V, --version
            Prints version information


OPTIONS:
    -a, --additional-args <additional-args>...
            Extra args to pass to youtube-dl.

            Current arguments are: --write-info-json --hls-use-mpegts --no-part --netrc
    -c, --client-id <client-id>
            Twitch client id
    -t, --token <token>
            Twitch client token

        --delay-max <delay-max>
            Maximum milliseconds to wait before polling again [default: 3000]

        --delay-min <delay-min>
            Minimum milliseconds to wait before polling again [default: 100]

    -d, --directory <directory>
            Directory to save videos.

    -o, --filename-template <filename-template>
            Directory to save videos.

            See `man youtube-dl` under OUTPUT TEMPLATE for variables to use.

            Useful variables:
             - %(uploader)s: channel name
             - %(uploader_id)s: channel name (lowercase)
             - %(description)s: channel status/title
             - %(timestamp)s
             - %(title)s: for a live stream, looks like 'ashkankiani 2019-09-06 14_19'

            Note that in the case of a temporary download failure, when youtube-dl is restarted the filename may change,
            meaning a separate file is created. This occurs with %(title)s, which uses the time of downloading as part
            of the filename.

            MPEGTS files can be concatenated together without any complex processing (e.g. with `cat`), so these streams
            can be trivially recomposed.

            I personally use "%(uploader)s/%(uploader)s-%(id)s-%(description)s.%(ext)s"

             [default: %(title)s-%(id)s.%(ext)s]
    -x, --script <script>
            A script to execute when the stream comes live.

            These environment variables will be set:
             - TWITCH_CHANNEL_NAME
             - TWITCH_CHANNEL_ID
             - TWITCH_CHANNEL_STATUS
             - TWITCH_STREAM_ID
             - TWITCH_STREAM_CREATED_AT
             - YOUTUBE_DL_PID: The pid of the child process launched with youtube-dl

ARGS:
    <channel-names>...
            List of channel names to poll.
```

### Planned features

- [ ] Subcommand/companion tool for scraping chat logs.
- [ ] Taking a file as the input list for channels to watch so that
the list can be amended without restarting the tool. This will allow it to
become a permanent daemon. Planned to be an optional CLI flag of `-f` for
`--file` which will constitute a file to watch for channel names.

### Thanks

Sweet ascii art http://www.patorjk.com/software/taag/
