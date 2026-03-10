use server::classes::shahid::ShahidSearcher;
use thirtyfour::{prelude::*, ChromeCapabilities};
mod server;
mod client;
use server::functions::{start_discord, DiscordData};
use std::fs;
use std::env;
struct Streamer;
#[warn(unused_must_use)]
impl Streamer {
    async fn start(_url: &str, r#type: &str, _id: &str) -> WebDriverResult<()>  {
        let exe_dir = env::current_exe().unwrap()
        .parent().unwrap()
        .to_path_buf();

        println!("{:?}", exe_dir);
        let extensions_dir = exe_dir.join("Extensions");

        let extensions: Vec<String> = fs::read_dir(extensions_dir)
            .unwrap()
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_dir() {
                    path.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect();

        let extensions_arg = extensions.join(",");
        let mut options = ChromeCapabilities::new();
        options.add_arg("--ignore-ssl-errors=yes").unwrap();
        options.add_arg("--ignore-certificate-errors").unwrap();
        options.add_arg(
            "--auto-select-tab-capture-source-by-title=about:blank"
        ).unwrap();
        options.add_arg("--disable-gpu").unwrap();
        options.add_arg("--enable-chrome-browser-cloud-management").unwrap();
        options.add_arg("--enable-javascript").unwrap();
        options.add_arg("--disable-blink-features=AutomationControlled").unwrap();
        options.add_arg("--auto-accept-camera-and-microphone-capture").unwrap();
        options.add_arg("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0").unwrap();
        options.add_arg("--start-maximized").unwrap();
        options.add_arg(&format!("--load-extension={}", extensions_arg)).unwrap();

        println!("Loading extensions from: {}", extensions_arg);

        let driver = WebDriver::new("http://localhost:50000", options).await?;
        start_discord(driver, Some(DiscordData { id: _id.to_string()}), _url, r#type).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    client::client::main().await;
    // match ShahidSearcher::search("shush").await {
    //     Ok(results) => {
    //         ShahidSearcher::fetch_shahid_playlist(&results[0].1).await;
    //     }
    //     Err(e) => { println!("{}", e) } 
    // }
    
}
