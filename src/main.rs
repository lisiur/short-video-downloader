mod douyin;
mod kuaishou;
mod page;
mod video;

use crate::page::Page;
use crate::video::download_video;
use std::process::exit;
use tokio::task::JoinHandle;

type AppResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut args = std::env::args();
    if args.len() < 2 {
        log::info!("请提供分享短地址");
        exit(1);
    }
    args.next();
    let urls = args
        .filter(|arg| arg.starts_with("http"))
        .collect::<Vec<String>>();
    let tasks = urls
        .into_iter()
        .map(|url| {
            tokio::spawn(async move {
                if let Some(page) = Page::from(&url) {
                    if let Err(e) = download_video(&*page, 0).await {
                        anyhow::bail!(e.to_string())
                    } else {
                        Ok(())
                    }
                } else {
                    anyhow::bail!("未识别的短地址: {}", url)
                }
            })
        })
        .collect::<Vec<JoinHandle<_>>>();

    for (i, task) in tasks.into_iter().enumerate() {
        if let Err(e) = task.await.unwrap() {
            log::info!("{}", e.to_string());
            println!("第{}个下载失败", i + 1);
        } else {
            println!("第{}个下载成功!", i + 1);
        }
    }
}
