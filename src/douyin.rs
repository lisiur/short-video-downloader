use crate::video::{Video, VideoDetailPage};
use crate::AppResult;
use async_trait::async_trait;
use scraper::{Html, Selector};

pub struct DouyinVideoDetailPage {
    url: String,
}

impl DouyinVideoDetailPage {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

#[async_trait]
impl VideoDetailPage for DouyinVideoDetailPage {
    async fn extract_video(&self) -> AppResult<Option<Video>> {
        log::info!("【解析短地址...】 {}", self.url);
        let resp = reqwest::get(&self.url).await?;
        let text = resp.text().await?;
        let document = Html::parse_document(&text);
        let selector = Selector::parse("#RENDER_DATA").unwrap();
        log::info!("【解析 RENDER_DATA...】");
        match document.select(&selector).next() {
            Some(ele) => {
                log::info!("【查询视频地址...】");
                let content = ele.inner_html();
                let content = percent_encoding::percent_decode(content.as_bytes())
                    .decode_utf8()
                    .unwrap();
                let data: serde_json::Value = serde_json::from_str(&content)?;
                let author_name = &data["C_14"]["aweme"]["detail"]["authorInfo"]["nickname"];
                let video_title = &data["C_14"]["aweme"]["detail"]["desc"];
                let video_src = &data["C_14"]["aweme"]["detail"]["video"]["playAddr"][0]["src"];
                if author_name.is_string() && video_title.is_string() && video_src.is_string() {
                    log::info!("【查询视频地址成功】 {}", video_src);
                    Ok(Some(Video {
                        author: author_name.as_str().unwrap().to_string(),
                        title: video_title.as_str().unwrap().to_string(),
                        src: "https:".to_string() + video_src.as_str().unwrap(),
                    }))
                } else {
                    log::info!("【查询视频地址失败】");
                    Ok(None)
                }
            }
            None => {
                log::info!("【解析 RENDER_DATA 失败】 未找到");
                Ok(None)
            }
        }
    }
}
