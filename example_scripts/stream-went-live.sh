#!/bin/sh
exec notify-send TWITCH "$TWITCH_CHANNEL_NAME went live at $TWITCH_STREAM_CREATED_AT!
Downloading $TWITCH_STREAM_ID"
