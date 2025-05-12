use thirtyfour::{error::WebDriverResult, By, WebDriver};
use crate::server::classes::fasel::Fasel;
use tokio::sync::Mutex;
use std::time::Duration;
use once_cell::sync::Lazy;

use super::classes::netflix::Netflix;

pub fn return_script(token: &str) -> String {
    format!(
        r#"
        function login(token) {{
            setInterval(() => {{
                document.body.appendChild(document.createElement('iframe')).contentWindow.localStorage.token = `"${{token}}"`;
            }}, 50);
            setTimeout(() => {{
                location.reload();
            }}, 2500);
        }}
        login(`{}`);
        "#,
        token
    )
}


pub struct DiscordData {
    pub id: String,
}

impl Default for DiscordData {
    fn default() -> Self {
        Self {
            id: "1000710976343134293".to_string(),
        }
    }
}

static SHOULD_QUIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DRIVER_INSTANCE: Lazy<Mutex<Option<WebDriver>>> = Lazy::new(|| Mutex::new(None));

pub async fn set_driver(driver: WebDriver) {
    let mut instance = DRIVER_INSTANCE.lock().await;
    *instance = Some(driver);
}

pub async fn quit_browser() -> WebDriverResult<()> {
    let mut instance = DRIVER_INSTANCE.lock().await;
    if let Some(driver) = instance.take() {
        driver.quit().await?;
    }
    let mut should_quit = SHOULD_QUIT.lock().await;
    *should_quit = true; 
    Ok(())
}

pub async fn keep_browser_alive() {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await; 
        let should_quit = SHOULD_QUIT.lock().await;
        if *should_quit {
            break; 
        }
    }
}

pub async fn start_discord(driver: WebDriver, discord_data: Option<DiscordData>, url: &str, r#type: &str) -> WebDriverResult<()> {
    set_driver(driver.clone()).await;

    driver.goto("https://discord.com/login").await?;
    //Put token here
    driver.execute(return_script("Token"), Vec::new()).await?;
    driver.goto("https://discord.com/channels/1000710976343134289/1010686745840472104").await?;
    tokio::time::sleep(Duration::from_secs(4)).await;
    driver.find(By::Css(format!("[data-list-item-id='channels___{}']", discord_data.unwrap_or_default().id))).await?.click().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    driver.execute("window.open('about:blank')", Vec::new()).await?;
    let windows = driver.windows().await?;
    driver.switch_to_window(windows[1].clone()).await?;

    driver.find(By::XPath("//*[@id=\"app-mount\"]/div[2]/div[1]/div[1]/div/div[2]/div/div/div/div/div[1]/section/div[1]/div/div[2]/button[2]")).await?.click().await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    driver.switch_to_window(windows[2].clone()).await?;
    if r#type == "Fasel" {
        let fasel = Fasel::new(driver.clone()).await;
        fasel.start(url).await?;
    } else if r#type == "Netflix" {
        let netflix = Netflix::new(driver.clone()).await;
        netflix.start(url).await?;
    }

    tokio::spawn(async {
        keep_browser_alive().await;
    });

    Ok(())
}
