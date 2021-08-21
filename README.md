# 抖音/快手短视频下载器

## 编译

```bash
cargo build --release
```

## 运行
```bash
./target/release/downloader <url>
```

> url 为短视频分享的短链接

## 注意事项

下载快手短视频时需要在执行目录下添加 did.txt 文件，文件内容为快手的 did  

### 快手 did 的获取方式为：  

1. 浏览器打开 https://kuaishou.com (无需登录)
2. 打开控制台，在 application 面板中的 Cookies 中可以看到 did