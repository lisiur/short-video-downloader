use crate::douyin::DouyinVideoDetailPage;
use crate::kuaishou::KuaishouVideoDetailPage;
use crate::video::VideoDetailPage;

pub struct Page;

impl Page {
    pub fn from(url: &str) -> Option<Box<dyn VideoDetailPage + Send + Sync>> {
        if url.contains("kuaishou") {
            Some(Box::new(KuaishouVideoDetailPage::new(url)))
        } else if url.contains("douyin") {
            Some(Box::new(DouyinVideoDetailPage::new(url)))
        } else {
            None
        }
    }
}
