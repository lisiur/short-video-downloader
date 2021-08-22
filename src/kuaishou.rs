use crate::video::{Video, VideoDetailPage};
use crate::AppResult;
use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use std::time::Duration;

pub struct KuaishouVideoDetailPage {
    url: String,
}

impl KuaishouVideoDetailPage {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

#[async_trait]
impl VideoDetailPage for KuaishouVideoDetailPage {
    async fn extract_video(&self) -> AppResult<Option<Video>> {
        let cookie_store = Arc::new(reqwest::cookie::Jar::default());
        log::info!("【解析短地址...】 {}", self.url);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .cookie_provider(cookie_store.clone())
            // .cookie_store(true)
            .build()?;
        let resp = client.get(&self.url).send().await?;
        let url = resp.url();
        log::info!("【短地址解析成功】 {}", url.as_str());
        let id = url.path().split("/").last().unwrap().to_string();
        let payload = query_template(&id);
        log::info!("【查询视频地址...】");
        let did = std::fs::read_to_string("./did.txt").map_or(None, |v| Some(v.trim().to_string()));

        log::debug!("{:?}", cookie_store);
        let mut instance = client
            .post("https://www.kuaishou.com/graphql")
            .header("Host", "www.kuaishou.com")
            .header("Origin", "https://www.kuaishou.com")
            .header("Referer", url.as_str())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36")
            .header("sec-ch-ua", r#""Chromium";v="92", " Not A;Brand";v="99", "Google Chrome";v="92""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
            .header("DNT", "1")
            .header("accept", "*/*")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7");
        if let Some(did) = did {
            instance = instance.header(
                "Cookie",
                format!("did={}; kpf=PC_WEB; kpn=KUAISHOU_VISION; clientid=3", did),
            );
        }
        let res = instance.json(&payload).send().await?;
        let data: serde_json::Value = res.json().await?;
        let author_name = &data["data"]["visionVideoDetail"]["author"]["name"];
        let video_title = &data["data"]["visionVideoDetail"]["photo"]["caption"];
        let video_src = &data["data"]["visionVideoDetail"]["photo"]["photoUrl"];

        if author_name.is_string() && video_title.is_string() && video_src.is_string() {
            log::info!("【查询视频地址成功】 {}", video_src);
            Ok(Some(Video {
                author: author_name.as_str().unwrap().to_string(),
                title: video_title.as_str().unwrap().to_string(),
                src: video_src.as_str().unwrap().to_string(),
            }))
        } else {
            log::error!("【查询视频地址失败】 {}", data);
            anyhow::bail!("查询视频地址失败")
        }
    }
}

pub fn query_template(id: &str) -> serde_json::Value {
    serde_json::json!({
        "operationName": "visionVideoDetail",
        "query": "query visionVideoDetail($photoId: String, $type: String, $page: String, $webPageArea: String) {\n visionVideoDetail(photoId: $photoId, type: $type, page: $page, webPageArea: $webPageArea) {\n author {\n name\n }\n photo {\n caption\n photoUrl\n }\n }\n }",
        "variables": {
            "photoId": id,
            "page": "detail",
        },
    })
}
