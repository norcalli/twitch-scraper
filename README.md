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
  Downloaded twitch-scraper v0.1.1
  ...

❯ curl -OL https://github.com/norcalli/twitch-scraper/releases/download/v0.1.1/twitch-scraper \
	&& chmod +x twitch-scraper # Or download a prebuilt static binary

❯ twitch-scraper \
	-q \
	-c $TWITCH_CLIENT_ID \
	-d /videos/twitch/ \
	-o "%(channel)s/%(title)s-%(id)s.%(ext)s" \
	-x $PWD/stream-went-live.sh \
	ashkankiani naysayer88 demolition_d studio_trigger
[2019-09-10T22:48:13Z INFO  twitch_scraper] Watching ashkankiani with id 443849438
[2019-09-10T22:48:13Z INFO  twitch_scraper] Watching naysayer88 with id 51679076
[2019-09-10T22:48:13Z INFO  twitch_scraper] Watching demolition_d with id 4666862
[2019-09-10T22:48:14Z INFO  twitch_scraper] Watching studio_trigger with id 178995638
[2019-09-10T22:50:10Z INFO  twitch_scraper] Downloading stream 35615896368 from ashkankiani

❯ cat example_scripts/stream-went-live.sh
#!/bin/sh
exec notify-send TWITCH "$TWITCH_CHANNEL_NAME went live at $TWITCH_STREAM_CREATED_AT!
Downloading $TWITCH_STREAM_ID"


❯ twitch-scraper --help
twitch-scraper 0.1.1
Program to poll twitch via its API and download streams from channels as they come live.

Use RUST_LOG to set logging level. e.g. export RUST_LOG='debug' or export RUST_LOG='twitch_scraper=info'

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
            
            - %(description)s: channel status/title
            
            - %(timestamp)s
            
            - %(title)s: for a live stream, looks like 'ashkankiani 2019-09-06 14_19'
            
            I personally use "%(uploader)s/%(title)s-%(description)s-%(id)s.%(ext)s" [default: %(title)s-%(id)s.%(ext)s]
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

### Thanks

Sweet ascii art http://www.patorjk.com/software/taag/
