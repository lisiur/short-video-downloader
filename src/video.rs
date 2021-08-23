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
    async fn extract_video(&self) -> AppResult<Option<Video>>;
}

pub async fn download_video(page: &dyn VideoDetailPage, retry: usize) -> AppResult<()> {
    let mut retry_times = retry;
    let video: Option<Video> = loop {
        match page.extract_video().await {
            Ok(video) => {
                break video;
            }
            Err(e) => {
                if retry_times > 0 {
                    log::info!("【剩余重试次数】 {}", retry_times);
                    // tokio::time::sleep(Duration::from_secs(2)).await;
                    retry_times -= 1;
                } else {
                    anyhow::bail!("{}", e.to_string());
                }
            }
        }
    };
    if let Some(video) = video {
        let video_name = format!("【{}】{}.mp4", &video.author, &video.title);
        log::info!("【下载中...】");
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        let resp = client.get(&video.src).send().await?;
        log::info!("【下载成功】 {}", video_name);
        let bytes = resp.bytes().await?;
        let mut slice: &[u8] = bytes.as_ref();
        let mut out = File::create(&video_name)?;
        std::io::copy(&mut slice, &mut out)?;
    }
    Ok(())
}
