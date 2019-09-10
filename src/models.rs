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
            pub channel: crate::models::search::channels::Channel,
            pub created_at: String,
            #[serde(flatten)]
            pub rest: serde_json::Value,
        }
    }
}
