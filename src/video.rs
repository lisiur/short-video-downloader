use crate::AppResult;
use async_trait::async_trait;
use std::fs::File;
use std::time::Duration;

#[derive(Debug)]
pub struct Video {
    pub src: String,
    pub author: String,
    pub title: String,
}

#[async_trait]
pub trait VideoDetailPage: Send + Sync {
    fn get_url(&self) -> &str;
    async fn extract_video(&self) -> AppResult<Option<Video>>;
}

pub async fn download_video(page: &dyn VideoDetailPage) -> AppResult<()> {
    let video = page.extract_video().await?;
    if let Some(video) = video {
        let video_name = format!("【{}】{}.mp4", &video.author, &video.title);
        println!("【下载中...】 {}", video_name);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        let resp = client.get(&video.src).send().await?;
        println!("【下载成功】");
        let bytes = resp.bytes().await?;
        let mut slice: &[u8] = bytes.as_ref();
        let mut out = File::create(&video_name)?;
        std::io::copy(&mut slice, &mut out)?;
    }
    Ok(())
}
