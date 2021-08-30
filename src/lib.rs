mod douyin;
mod kuaishou;
mod page;
mod video;

use crate::page::Page;
use crate::video::download_video;
use std::process::exit;
use tokio::task::JoinHandle;
use wasm_bindgen::prelude::*;
type AppResult<T> = anyhow::Result<T>;

#[no_mangle]
pub async fn download(arg: String) -> AppResult<()> {
    let urls = arg
        .split(" ")
        .filter(|arg| arg.starts_with("http"))
        .map(|s| s.to_string())
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
    Ok(())
}
