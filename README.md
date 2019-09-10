```
████████╗██╗    ██╗██╗████████╗ ██████╗██╗  ██╗         
╚══██╔══╝██║    ██║██║╚══██╔══╝██╔════╝██║  ██║         
   ██║   ██║ █╗ ██║██║   ██║   ██║     ███████║         
   ██║   ██║███╗██║██║   ██║   ██║     ██╔══██║         
   ██║   ╚███╔███╔╝██║   ██║   ╚██████╗██║  ██║         
   ╚═╝    ╚══╝╚══╝ ╚═╝   ╚═╝    ╚═════╝╚═╝  ╚═╝         
                                                        
███████╗ ██████╗██████╗  █████╗ ██████╗ ███████╗██████╗ 
██╔════╝██╔════╝██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗
███████╗██║     ██████╔╝███████║██████╔╝█████╗  ██████╔╝
╚════██║██║     ██╔══██╗██╔══██║██╔═══╝ ██╔══╝  ██╔══██╗
███████║╚██████╗██║  ██║██║  ██║██║     ███████╗██║  ██║
╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝
```


```sh
❯ twitch-scraper -c $TWITCH_CLIENT_ID -d /videos/twitch/ -o "%(channel)s/%(title)s-%(id)s.%(ext)s" -x $PWD/stream-went-live.sh naysayer88 demolition_d studio_trigger

❯ cat example_scripts/stream-went-live.sh
#!/bin/sh
exec notify-send TWITCH "$TWITCH_CHANNEL_NAME went live at $TWITCH_STREAM_CREATED_AT!
Downloading $TWITCH_STREAM_ID"


❯ twitch-scraper --help
twitch-scraper 0.1.0
Program to poll twitch via its API and download streams from channels as they come live.

USAGE:
    twitch-scraper [OPTIONS] --client-id <client-id> --directory <directory> [--] [channel-names]...

FLAGS:
    -h, --help
            Prints help information

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

            - %(channel)s

            - %(timestamp)s

            I personally use "%(channel)s/%(title)s-%(id)s.%(ext)s" [default: %(title)s-%(id)s.%(ext)s]
    -x, --script <script>
            A script to execute when the stream comes live.

            These environment variables will be set:

            - TWITCH_CHANNEL_NAME

            - TWITCH_CHANNEL_ID

            - TWITCH_STREAM_ID

            - TWITCH_STREAM_CREATED_AT

            - YOUTUBE_DL_PID: The pid of the child process launched with youtube-dl

ARGS:
    <channel-names>...
            List of channel names to poll.
```


### Thanks

Sweet ascii art http://www.patorjk.com/software/taag/
