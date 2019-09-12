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
