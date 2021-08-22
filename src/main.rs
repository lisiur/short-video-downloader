mod douyin;
mod kuaishou;
mod video;

use crate::douyin::DouyinVideoDetailPage;
use crate::kuaishou::KuaishouVideoDetailPage;
use crate::video::{download_video, VideoDetailPage};
use std::process::exit;

type AppResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = std::env::args().collect::<Vec<String>>();
    let url = args.get(1);
    if url.is_none() {
        log::error!("请提供分享短地址");
        exit(1);
    }
    let url = url.unwrap();
    if let Some(page) = Page::from(url) {
        if let Err(e) = download_video(&*page, 2).await {
            log::error!("{}", e.to_string());
            exit(1);
        };
    } else {
        log::error!("未识别的短地址");
        exit(1);
    }
}

struct Page;

impl Page {
    fn from(url: &str) -> Option<Box<dyn VideoDetailPage + Send + Sync>> {
        if url.contains("kuaishou") {
            Some(Box::new(KuaishouVideoDetailPage::new(url)))
        } else if url.contains("douyin") {
            Some(Box::new(DouyinVideoDetailPage::new(url)))
        } else {
            None
        }
    }
}
