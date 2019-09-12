pub mod kraken {
    pub mod search {
        pub mod channels {
            #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct Root {
                #[serde(rename = "_total")]
                pub total: i64,
                pub channels: Vec<Channel>,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct Channel {
                #[serde(rename = "_id")]
                pub id: i64,
                // pub display_name: String,
                pub name: String,
                pub status: Option<String>,
                #[serde(flatten)]
                pub rest: serde_json::Value,
            }

        }

    }

    pub mod streams {
        /*
        {
            "_total": 1295,
            "streams": [
                {
                    "_id": 23937446096,
                    "average_fps": 60,
                    "channel": {
                        "_id": 121059319,
                        "broadcaster_language": "en",
                        "created_at": "2016-04-06T04:12:40Z",
                        "display_name": "MOONMOON_OW",
                        "followers": 251220,
                        "game": "Overwatch",
                        "language": "en",
                        "logo": "https://static-cdn.jtvnw.net/jtv_user_pictures/moonmoon_ow-profile_image-0fe586039bb28259-300x300.png",
                        "mature": true,
                        "name": "moonmoon_ow",
                        "partner": true,
                        "profile_banner": "https://static-cdn.jtvnw.net/jtv_user_pictures/moonmoon_ow-profile_banner-13fbfa1ba07bcd8a-480.png",
                        "profile_banner_background_color": null,
                        "status": "KKona where my Darryl subs at KKona",
                        "updated_at": "2016-12-15T20:04:53Z",
                        "url": "https://www.twitch.tv/moonmoon_ow",
                        "video_banner": "https://static-cdn.jtvnw.net/jtv_user_pictures/moonmoon_ow-channel_offline_image-2b3302e20384eee8-1920x1080.png",
                        "views": 9869754
                    },
                    "created_at": "2016-12-15T14:55:49Z",
                    "delay": 0,
                    "game": "Overwatch",
                    "is_playlist": false,
                    "preview": {
                        "large": "https://static-cdn.jtvnw.net/previews-ttv/live_user_moonmoon_ow-640x360.jpg",
                        "medium": "https://static-cdn.jtvnw.net/previews-ttv/live_user_moonmoon_ow-320x180.jpg",
                        "small": "https://static-cdn.jtvnw.net/previews-ttv/live_user_moonmoon_ow-80x45.jpg",
                        "template": "https://static-cdn.jtvnw.net/previews-ttv/live_user_moonmoon_ow-{width}x{height}.jpg"
                    },
                    "video_height": 720,
                    "viewers": 11523
                }
            ]
        }
        */
        pub mod query {
            #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct Root {
                #[serde(rename = "_total")]
                pub total: i64,
                pub streams: Vec<Stream>,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct Stream {
                #[serde(rename = "_id")]
                pub id: i64,
                pub channel: crate::models::kraken::search::channels::Channel,
                pub created_at: String,
                #[serde(flatten)]
                pub rest: serde_json::Value,
            }
        }
    }
}

pub mod helix {
    pub mod search {
        pub mod channels {
            #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct Root {
                #[serde(rename = "_total")]
                pub total: i64,
                pub channels: Vec<Channel>,
            }

            #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct Channel {
                #[serde(rename = "_id")]
                pub id: i64,
                // pub display_name: String,
                pub name: String,
                pub status: Option<String>,
                #[serde(flatten)]
                pub rest: serde_json::Value,
            }

        }

    }

    /*
    {
      "data": [
        {
          "id": "26007351216",
          "user_id": "7236692",
          "user_name": "BillyBob",
          "game_id": "29307",
          "type": "live",
          "title": "[Punday Monday] Necromancer - Dan's First Character - Maps - !build",
          "viewer_count": 5723,
          "started_at": "2017-08-14T15:45:17Z",
          "language": "en",
          "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_dansgaming-{width}x{height}.jpg"
        }
       ],
       "pagination": {
         "cursor": "eyJiIjp7Ik9mZnNldCI6MH0sImEiOnsiT2Zmc2V0Ijo0MH19"
       }
    }
    json_typegen --options '{  field_visibility: "pub", derives: "Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize" }'
    */
    pub mod streams {
        #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Root {
            pub data: Vec<Datum>,
            pub pagination: Pagination,
        }

        #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Datum {
            pub id: String,
            pub user_id: String,
            pub user_name: String,
            pub game_id: String,
            #[serde(rename = "type")]
            pub type_field: String,
            pub title: String,
            pub viewer_count: i64,
            pub started_at: String,
            pub language: String,
            pub thumbnail_url: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Pagination {
            pub cursor: Option<String>,
        }
    }
}
