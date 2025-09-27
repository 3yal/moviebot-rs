use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use thirtyfour::{error::{WebDriverError, WebDriverResult}, By, Cookie, WebDriver};

use crate::server::functions::DRIVER_INSTANCE;
//Put full cookies here
const COOK: &str = "nfvdid=BQFmAAEBEGG82PJqtHmDiqWkdixVH1Jgj1vw4dpqDxTuM74P8VEm4DuFzkAZHRWmpvSzm0QpZZ2cWuxKOmZ-eKWNeTdwLrt6ay-T2lN9pHr3ZrFSGzWCuFM2xaAioH4XZYSJX1E-33Z5STRS1XmXr3I9ft2adtzQ; netflix-sans-normal-3-loaded=true; netflix-sans-bold-3-loaded=true; OptanonAlertBoxClosed=2025-09-14T22:07:34.723Z; SecureNetflixId=v%3D3%26mac%3DAQEAEQABABSNClRlxNl90B_rCu9Lfd_Z2dQnn4Ad_TE.%26dt%3D1758946807337; NetflixId=ct%3DBgjHlOvcAxLuA3ZVCsRCsX7yuSkWdE6J7H4ogOLg7kVMGcN5x7MKe6Wjenp8OMXAZnulXZxCO22IixHScVuyrmst6XYhflzLI0J0LTeQaqRhg5sGc7cESiaxysL5xcmZzmJsBTMh_ghzyG6anRFGgBONP_lGw6SgTLlUmis2TiH1L2PArMKCGN8aC7BQsf8ZO2lY-NX71j3muyNXuy5BrCS0pTcugjEPAboSxx7Um-cz0_-r-n3K2Bbegtx_WjideDQ5PesHlO4LcvCUClT1QgjTmOLOpFs_k7Np7c2yMQFmNjhlyIiw4i9-cIpwPfKIKrtX6QJUiMRENwXPA4QqwWNCvhDBHd33wAWCSw9toGYStKdDgihylNa7k-45XkmEqCce66XCp2PHkmyKTUAk48xWZKgXFP7l4HqOeCm_BbP2NhWe_-nk1uvMcXwAog8BjMQGnwRLQ89jbywUwJ0225mvev9Ol53iLVpapA9Xm5NfAuzXdse2ryj5jYtz9CDEs_rh8UW9pIw4B6hSf9sPNdtqAujOIjvhFM1m3-oIOzQnnTlhwJ22B-9fnO2m1AHyow-G9GjyCALUrSf0iCRMV9xAWnozJZluiDw1iRkxea1W-scl99yfHVEQqiN24-tzkUAfYS2BiXtjC11EW7ajDmWbr_h0PowrGAYiDgoMkLuXyzVv6JNbktQr%26ch%3DAQEAEAABABQREV5p_q-aW80QDqiOZ-fl4qX9Wyp2a6I.%26v%3D3%26pg%3DJYH2XQN2E5BRZHXFX2IFOXEV5Y; flwssn=67b947df-7a8e-487b-8776-fa6244a80a9b; profilesNewSession=0; gsid=1a780dca-ed71-4aef-8a4a-f910fd27ff7c; OptanonConsent=isGpcEnabled=0&datestamp=Sat+Sep+27+2025+07%3A22%3A29+GMT%2B0300+(Eastern+European+Summer+Time)&version=202508.1.0&browserGpcFlag=0&isIABGlobal=false&hosts=&consentId=c1fe26d4-d8af-4e2b-b5d2-f7673ce32523&interactionCount=2&isAnonUser=1&landingPath=NotLandingPage&AwaitingReconsent=false&groups=C0001%3A1%2CC0002%3A1%2CC0003%3A1%2CC0004%3A1&intType=1&geolocation=EG%3BC";
pub struct Netflix {
    driver: Arc<WebDriver>,
}

#[derive(Debug)]
pub struct ShowResult {
    pub show_data: Value,
    pub is_show: bool,
}



pub struct NetflixSearcher;

impl NetflixSearcher {
    pub async fn is_show(
        movie_id: &str,
    ) -> Result<ShowResult, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();

        let query_id = "7515a9d0-9422-4bbd-b7fe-aefa7f17ef5a";
        let request_body = json!({
            "operationName": "PreviewModalEpisodeSelector",
            "variables": {
                "showId": movie_id,
                "seasonCount": 30
            },
            "extensions": {
                "persistedQuery": {
                    "id": query_id,
                    "version": 102
                }
            }
        });

        let response = client
            .post("https://web.prod.cloud.netflix.com/graphql")
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("cache-control", "no-cache")
            .header("content-type", "application/json")
            .header("pragma", "no-cache")
            .header("priority", "u=1, i")
            .header(
                "sec-ch-ua",
                "\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Opera GX\";v=\"114\"",
            )
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-model", "\"\"")
            .header("sec-ch-ua-platform", "\"Windows\"")
            .header("sec-ch-ua-platform-version", "\"15.0.0\"")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("x-netflix.browsername", "Opera")
            .header("x-netflix.browserversion", "114")
            .header("x-netflix.client.request.name", "ui/falcorUnclassified")
            .header("x-netflix.clienttype", "akira")
            .header("x-netflix.esn", "NFCDOP-01-LKGUFQQ17TKAG2MJMU7PNKTCLEFD02")
            .header("x-netflix.esnprefix", "NFCDOP-01-")
            .header("x-netflix.nq.stack", "prod")
            .header("x-netflix.osfullname", "Windows 10")
            .header("x-netflix.osname", "Windows")
            .header("x-netflix.osversion", "10.0")
            .header("x-netflix.request.attempt", "1")
            .header(
                "x-netflix.request.client.context",
                "{\"appstate\":\"foreground\"}",
            )
            .header(
                "x-netflix.request.client.user.guid",
                "JYH2XQN2E5BRZHXFX2IFOXEV5Y",
            )
            .header("x-netflix.request.id", "bb1b2fee73274e51b55803a14f0639a5")
            .header("x-netflix.uiversion", "v43acacdd")
            .header("Cookie", COOK)
            .json(&request_body)
            .send()
            .await?;

        let response_json: Value = response.json().await?;
        if let Some(videos) = response_json["data"]["videos"].as_array() {
            if videos.is_empty() {
                return Ok(ShowResult {
                    show_data: json!({ "movie_id": movie_id }),
                    is_show: false,
                });
            }

            let show_data = &videos[0];
            let is_tv_show = show_data["seasons"].get("edges").is_some();

            return Ok(ShowResult {
                show_data: if is_tv_show {
                    show_data.clone()
                } else {
                    json!({ "movie_id": movie_id })
                },
                is_show: is_tv_show,
            });
        }

        Ok(ShowResult {
            show_data: json!({ "movie_id": movie_id }),
            is_show: false,
        })
    }
    pub async fn search(
        movie_name: &str,
    ) -> Result<Vec<(String, String, String, ShowResult)>, Box<dyn std::error::Error + Send + Sync>>
    {
        let form_data = Self::create_form_data(movie_name);
        let client = Client::new();
        let response = client
            .post("https://www.netflix.com/nq/website/memberapi/release/pathEvaluator?webp=true&drmSystem=widevine&isVolatileBillboardsEnabled=true&isTop10Supported=true&isTop10KidsSupported=true&hasVideoMerchInBob=true&hasVideoMerchInJaw=true&falcor_server=0.1.0&withSize=true&materialize=true&original_path=%2Fshakti%2Fmre%2FpathEvaluator")
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("cache-control", "no-cache")
            .header("content-type", "application/x-www-form-urlencoded")
            .header("pragma", "no-cache")
            .header("priority", "u=1, i")
            .header("sec-ch-ua", "\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Opera GX\";v=\"114\"")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-model", "\"\"")
            .header("sec-ch-ua-platform", "\"Windows\"")
            .header("sec-ch-ua-platform-version", "\"15.0.0\"")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("x-netflix.browsername", "Opera")
            .header("x-netflix.browserversion", "114")
            .header("x-netflix.client.request.name", "ui/falcorUnclassified")
            .header("x-netflix.clienttype", "akira")
            .header("x-netflix.esn", "NFCDOP-01-LKGUFQQ17TKAG2MJMU7PNKTCLEFD02")
            .header("x-netflix.esnprefix", "NFCDOP-01-")
            .header("x-netflix.nq.stack", "prod")
            .header("x-netflix.osfullname", "Windows 10")
            .header("x-netflix.osname", "Windows")
            .header("x-netflix.osversion", "10.0")
            .header("x-netflix.request.attempt", "1")
            .header("x-netflix.request.client.context", "{\"appstate\":\"foreground\"}")
            .header("x-netflix.request.client.user.guid", "JYH2XQN2E5BRZHXFX2IFOXEV5Y")
            .header("x-netflix.request.id", "bb1b2fee73274e51b55803a14f0639a5")
            .header("x-netflix.uiversion", "v43acacdd")
            .header("Cookie", COOK)
            .body(form_data)
            .send()
            .await?;

        let json: Value = response.json().await?;
        let mut results = Vec::new();

        let search_page = json
            .get("jsonGraph")
            .and_then(|jg| jg.get("searchPage"))
            .and_then(|sp| sp.as_object());

        if let Some(search_page) = search_page {
            if let Some((_, first_value)) = search_page.iter().next() {
                let mut all_items = Vec::new();

                if let Some(items_0) = first_value.get("0").and_then(|v| v.as_object()) {
                    all_items.extend(items_0.values());
                }
                if let Some(items_1) = first_value.get("1").and_then(|v| v.as_object()) {
                    all_items.extend(items_1.values());
                }

                for (i, item) in all_items
                    .iter()
                    .filter(|item| {
                        item.get("summary")
                            .and_then(|v| v.as_object())
                            .and_then(|s| s.get("value"))
                            .and_then(|v| v.get("imgUrl"))
                            .and_then(|v| v.as_str())
                            .map(|url| !url.is_empty())
                            .unwrap_or(false)
                    })
                    .enumerate()
                    .take(9)
                {
                    if let Some(summary) = item
                        .get("summary")
                        .and_then(|v| v.as_object())
                        .and_then(|d| d.get("value"))
                    {
                        let entity_id = summary
                            .get("entityId")
                            .map(|v| v.to_string())
                            .unwrap_or("".to_string());
                        let img_url = summary
                            .get("imgUrl")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let display_string = summary
                            .get("displayString")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let check_show = Self::is_show(&entity_id).await;
                        results.push((
                            format!("https://netflix.com/watch/{}", entity_id),
                            img_url,
                            display_string,
                            check_show.unwrap(),
                        ));
                    } else {
                        println!("Result {}: No displayString found.", i + 1);
                    }
                }
            } else {
                println!("No valid key found in 'searchPage'.");
            }
        } else {
            println!("'searchPage' not found in JSON response.");
        }

        Ok(results)
    }

    fn create_form_data(movie_name: &str) -> String {
        let mut form_data = Vec::new();

        form_data.push(("path", format!("[\"search\",\"query\",\"@@NAPA-49279779-77a7-46f8-a83a-f031d1ef037c\",\"{}\",\"summary\"]", movie_name)));
        form_data.push(("path", format!("[\"search\",\"query\",\"@@NAPA-49279779-77a7-46f8-a83a-f031d1ef037c\",\"{}\",{{\"from\":0,\"to\":1}},\"summary\"]", movie_name)));
        form_data.push(("path", format!("[\"search\",\"query\",\"@@NAPA-49279779-77a7-46f8-a83a-f031d1ef037c\",\"{}\",{{\"from\":0,\"to\":1}},{{\"from\":0,\"to\":47}},\"summary\"]", movie_name)));
        form_data.push(("path", format!("[\"search\",\"query\",\"@@NAPA-49279779-77a7-46f8-a83a-f031d1ef037c\",\"{}\",{{\"from\":0,\"to\":1}},{{\"from\":0,\"to\":47}},\"reference\",[\"availability\",\"episodeCount\",\"inRemindMeList\",\"itemSummary\",\"queue\",\"summary\"]]", movie_name)));

        form_data.push((
            "authURL",
            "1739263844285.2XpQna0eN71gFJg+gzS5ycqyaAo=".to_string(),
        ));

        serde_urlencoded::to_string(&form_data).unwrap()
    }
    pub async fn get_episodes_for_shows(
        season_id: &str,
    ) -> Result<Vec<(i64, String, i64)>, Box<dyn std::error::Error + Send + Sync>> {
        let query_id = "380568aa-ec71-479c-832e-2e1f0ec13ec2";
        let request_body = json!({
            "operationName": "PreviewModalEpisodeSelectorSeasonEpisodes",
            "variables": {
                "seasonId": season_id,
                "count": 30,
                "opaqueImageFormat": "WEBP",
                "artworkContext": {}
            },
            "extensions": {
                "persistedQuery": {
                    "id": query_id,
                    "version": 102
                }
            }
        });
        let clinet = Client::new();
        let response = clinet
            .post("https://web.prod.cloud.netflix.com/graphql")
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("cache-control", "no-cache")
            .header("content-type", "application/json")
            .header("pragma", "no-cache")
            .header("priority", "u=1, i")
            .header(
                "sec-ch-ua",
                "\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Opera GX\";v=\"114\"",
            )
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-model", "\"\"")
            .header("sec-ch-ua-platform", "\"Windows\"")
            .header("sec-ch-ua-platform-version", "\"15.0.0\"")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("x-netflix.browsername", "Opera")
            .header("x-netflix.browserversion", "114")
            .header("x-netflix.client.request.name", "ui/falcorUnclassified")
            .header("x-netflix.clienttype", "akira")
            .header("x-netflix.esn", "NFCDOP-01-LKGUFQQ17TKAG2MJMU7PNKTCLEFD02")
            .header("x-netflix.esnprefix", "NFCDOP-01-")
            .header("x-netflix.nq.stack", "prod")
            .header("x-netflix.osfullname", "Windows 10")
            .header("x-netflix.osname", "Windows")
            .header("x-netflix.osversion", "10.0")
            .header("x-netflix.request.attempt", "1")
            .header(
                "x-netflix.request.client.context",
                "{\"appstate\":\"foreground\"}",
            )
            .header(
                "x-netflix.request.client.user.guid",
                "JYH2XQN2E5BRZHXFX2IFOXEV5Y",
            )
            .header("x-netflix.request.id", "bb1b2fee73274e51b55803a14f0639a5")
            .header("x-netflix.uiversion", "v43acacdd")
            .header("Cookie", COOK)
            .body(request_body.to_string())
            .send()
            .await?;

        let response_json: Value = response.json().await?;
        let mut results = Vec::new();
        if let Some(episodes) = response_json
            .get("data")
            .and_then(|d| d.get("videos"))
            .and_then(|v| v[0].get("episodes"))
            .and_then(|e| e.get("edges"))
            .and_then(|edges| edges.as_array())
        {
            for episode in episodes {
                if let Some(node) = episode.get("node") {
                    let title = node
                        .get("title")
                        .and_then(|t| t.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let number = node.get("number").and_then(|n| n.as_i64()).unwrap_or(0);
                    let video_id = node.get("videoId").and_then(|v| v.as_i64()).unwrap_or(0);
                    results.push((number, title, video_id));
                }
            }
        }
        Ok(results)
    }
}

impl Netflix {
    pub async fn new(driver: WebDriver) -> Self {
        Netflix {
            driver: Arc::new(driver),
        }
    }
    pub async fn start(&self, url: &str) -> WebDriverResult<()> {
        self.driver.goto("https://netflix.com").await?;
        self.driver
            .set_implicit_wait_timeout(std::time::Duration::from_secs(10))
            .await?;
        self.driver.delete_all_cookies().await?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let existing_cookies = self.driver.get_all_cookies().await?;
        if !existing_cookies.is_empty() {
            println!("Warning: Some cookies were not deleted!");
        }
        //Put cookie json here
        let cookies_json = r#"[
{
    "domain": ".netflix.com",
    "expirationDate": 1758957606.602383,
    "hostOnly": false,
    "httpOnly": false,
    "name": "flwssn",
    "path": "/",
    "sameSite": "unspecified",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "67b947df-7a8e-487b-8776-fa6244a80a9b",
    "id": 1
},
{
    "domain": ".netflix.com",
    "expirationDate": 1759033207.438299,
    "hostOnly": false,
    "httpOnly": true,
    "name": "gsid",
    "path": "/",
    "sameSite": "no_restriction",
    "secure": true,
    "session": false,
    "storeId": "0",
    "value": "1a780dca-ed71-4aef-8a4a-f910fd27ff7c",
    "id": 2
},
{
    "domain": ".netflix.com",
    "expirationDate": 1766722953.708635,
    "hostOnly": false,
    "httpOnly": false,
    "name": "netflix-sans-bold-3-loaded",
    "path": "/",
    "sameSite": "unspecified",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "true",
    "id": 3
},
{
    "domain": ".netflix.com",
    "expirationDate": 1766722953.708465,
    "hostOnly": false,
    "httpOnly": false,
    "name": "netflix-sans-normal-3-loaded",
    "path": "/",
    "sameSite": "unspecified",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "true",
    "id": 4
},
{
    "domain": ".netflix.com",
    "expirationDate": 1790482806.015352,
    "hostOnly": false,
    "httpOnly": true,
    "name": "NetflixId",
    "path": "/",
    "sameSite": "lax",
    "secure": true,
    "session": false,
    "storeId": "0",
    "value": "ct%3DBgjHlOvcAxLuA3ZVCsRCsX7yuSkWdE6J7H4ogOLg7kVMGcN5x7MKe6Wjenp8OMXAZnulXZxCO22IixHScVuyrmst6XYhflzLI0J0LTeQaqRhg5sGc7cESiaxysL5xcmZzmJsBTMh_ghzyG6anRFGgBONP_lGw6SgTLlUmis2TiH1L2PArMKCGN8aC7BQsf8ZO2lY-NX71j3muyNXuy5BrCS0pTcugjEPAboSxx7Um-cz0_-r-n3K2Bbegtx_WjideDQ5PesHlO4LcvCUClT1QgjTmOLOpFs_k7Np7c2yMQFmNjhlyIiw4i9-cIpwPfKIKrtX6QJUiMRENwXPA4QqwWNCvhDBHd33wAWCSw9toGYStKdDgihylNa7k-45XkmEqCce66XCp2PHkmyKTUAk48xWZKgXFP7l4HqOeCm_BbP2NhWe_-nk1uvMcXwAog8BjMQGnwRLQ89jbywUwJ0225mvev9Ol53iLVpapA9Xm5NfAuzXdse2ryj5jYtz9CDEs_rh8UW9pIw4B6hSf9sPNdtqAujOIjvhFM1m3-oIOzQnnTlhwJ22B-9fnO2m1AHyow-G9GjyCALUrSf0iCRMV9xAWnozJZluiDw1iRkxea1W-scl99yfHVEQqiN24-tzkUAfYS2BiXtjC11EW7ajDmWbr_h0PowrGAYiDgoMkLuXyzVv6JNbktQr%26ch%3DAQEAEAABABQREV5p_q-aW80QDqiOZ-fl4qX9Wyp2a6I.%26v%3D3%26pg%3DJYH2XQN2E5BRZHXFX2IFOXEV5Y",
    "id": 5
},
{
    "domain": ".netflix.com",
    "expirationDate": 1776325600.996144,
    "hostOnly": false,
    "httpOnly": false,
    "name": "nfvdid",
    "path": "/",
    "sameSite": "unspecified",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "BQFmAAEBEGG82PJqtHmDiqWkdixVH1Jgj1vw4dpqDxTuM74P8VEm4DuFzkAZHRWmpvSzm0QpZZ2cWuxKOmZ-eKWNeTdwLrt6ay-T2lN9pHr3ZrFSGzWCuFM2xaAioH4XZYSJX1E-33Z5STRS1XmXr3I9ft2adtzQ",
    "id": 6
},
{
    "domain": ".netflix.com",
    "expirationDate": 1789423654,
    "hostOnly": false,
    "httpOnly": false,
    "name": "OptanonAlertBoxClosed",
    "path": "/",
    "sameSite": "lax",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "2025-09-14T22:07:34.723Z",
    "id": 7
},
{
    "domain": ".netflix.com",
    "expirationDate": 1790482949,
    "hostOnly": false,
    "httpOnly": false,
    "name": "OptanonConsent",
    "path": "/",
    "sameSite": "lax",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "isGpcEnabled=0&datestamp=Sat+Sep+27+2025+07%3A22%3A29+GMT%2B0300+(Eastern+European+Summer+Time)&version=202508.1.0&browserGpcFlag=0&isIABGlobal=false&hosts=&consentId=c1fe26d4-d8af-4e2b-b5d2-f7673ce32523&interactionCount=2&isAnonUser=1&landingPath=NotLandingPage&AwaitingReconsent=false&groups=C0001%3A1%2CC0002%3A1%2CC0003%3A1%2CC0004%3A1&intType=1&geolocation=EG%3BC",
    "id": 8
},
{
    "domain": ".netflix.com",
    "expirationDate": 1758948831.497596,
    "hostOnly": false,
    "httpOnly": false,
    "name": "profilesNewSession",
    "path": "/",
    "sameSite": "unspecified",
    "secure": false,
    "session": false,
    "storeId": "0",
    "value": "0",
    "id": 9
},
{
    "domain": ".netflix.com",
    "expirationDate": 1790482806.014867,
    "hostOnly": false,
    "httpOnly": true,
    "name": "SecureNetflixId",
    "path": "/",
    "sameSite": "strict",
    "secure": true,
    "session": false,
    "storeId": "0",
    "value": "v%3D3%26mac%3DAQEAEQABABSNClRlxNl90B_rCu9Lfd_Z2dQnn4Ad_TE.%26dt%3D1758946807337",
    "id": 10
}
]"#;

        let cookies: Vec<CookieData> = serde_json::from_str(cookies_json).expect("Failed to parse cookies");
        add_cookies(&self.driver, &cookies).await?;
        
        self.driver.goto(url).await?;
        println!("Cookies added successfully!");
        // self.driver.find(By::XPath("//*[@id=\"appMountPoint\"]/div/div/div/div/div[1]/div[2]/div/div[1]/div[4]/div[1]/div[1]/a/button")).await?.click().await?;
        Ok(())
    }
    pub async fn pause() -> WebDriverResult<()> {
        let driver_lock = DRIVER_INSTANCE.lock().await;
    
        if let Some(driver) = driver_lock.as_ref() {
            let window_size = driver.get_window_rect().await?;
            let center_x = window_size.width / 2;
            let center_y = window_size.height / 2;
    
            driver.action_chain()
                .move_to(center_x - 1, center_y - 1) // Small nudge
                .move_to(center_x, center_y) // Move back
                .click()
                .perform()
                .await?;
    
            println!("Clicked");
        } else {
            println!("Nice");
        }
    
        Ok(())
    }
    pub async fn skipfront() -> WebDriverResult<()> {
        let driver_lock = DRIVER_INSTANCE.lock().await;
        if let Some(driver) = driver_lock.as_ref() {
            let window_size = driver.get_window_rect().await?;
            let center_x = window_size.width / 2;
            let center_y = window_size.height / 2;
    
            driver.action_chain()
                .move_to(center_x - 1, center_y - 1) // Small nudge
                .move_to(center_x, center_y) // Move back
                .perform()
                .await?;
            driver.execute("document.querySelector(\"#appMountPoint > div > div > div > div > div.watch-video > div > div > div.ltr-1m81c36 > div.watch-video--bottom-controls-container.ltr-gpipej > div > div > div.ltr-100d0a9 > div > div:nth-child(1) > div:nth-child(5) > button\").click()", Vec::new()).await?;
        } else {
            println!("Nice");
        }

        Ok(())
    }
    pub async fn skipback() -> WebDriverResult<()> {
        let driver_lock = DRIVER_INSTANCE.lock().await;
        if let Some(driver) = driver_lock.as_ref() {
            let window_size = driver.get_window_rect().await?;
            let center_x = window_size.width / 2;
            let center_y = window_size.height / 2;
    
            driver.action_chain()
                .move_to(center_x - 1, center_y - 1) // Small nudge
                .move_to(center_x, center_y) // Move back
                .perform()
                .await?;
            driver.execute("document.querySelector(\"div > div.watch-video--bottom-controls-container.ltr-gpipej > div > div > div.ltr-100d0a9 > div > div:nth-child(1) > div:nth-child(3) > button\").click()", Vec::new()).await?;
        } else {
            println!("Nice");
        }

        Ok(())
    }
    pub async fn skip_to_specific_timeline(time_input: &str) -> WebDriverResult<()> {
        let driver_lock = DRIVER_INSTANCE.lock().await;
        if let Some(driver) = driver_lock.as_ref() {
            // Convert input time format to milliseconds
            let target_time_ms = if let Some(ms) = convert_time_format(time_input) {
                ms
            } else {
                println!("Invalid time format: {}", time_input);
                return Err(WebDriverError::NotFound("Invalid time format.".into(), "nice".into()));
            };
    
            // Get screen center (small movement to avoid focus issues)
            let window_size = driver.get_window_rect().await?;
            let center_x = window_size.width / 2;
            let center_y = window_size.height / 2;
    
            driver.action_chain()
                .move_to(center_x - 1, center_y - 1) // Small nudge
                .move_to(center_x, center_y) // Move back
                .perform()
                .await?;
    
            // Locate timeline elements
            let timeline_bar = match driver.find(By::Css("div[data-uia='timeline-bar']")).await {
                Ok(el) => el,
                Err(_) => {
                    println!("Timeline bar not found.");
                    return Err(WebDriverError::NotFound("Invalid time format.".into(), "nice".into()));
                }
            };
    
            let slider_knob = match driver.find(By::Css("button[data-uia='timeline-knob']")).await {
                Ok(el) => el,
                Err(_) => {
                    println!("Timeline knob not found.");
                    return Err(WebDriverError::NotFound("Invalid time format.".into(), "nice".into()));
                }
            };
    
            // Get the maximum duration from the knob
            let max_time_str = slider_knob.attr("aria-valuemax").await?;
            let max_time = max_time_str
            .unwrap_or("0".to_string())
            .parse::<u64>()
            .unwrap_or(0) * 1000; // Convert seconds to milliseconds
            
            if max_time == 0 {
                println!("Error: max time is 0.");
                return Err(WebDriverError::NotFound("Invalid time format.".into(), "nice".into()));
            }
    
            // Ensure target time is within range
            let target_time_ms = target_time_ms.min(max_time);
    
            // Calculate percentage position
            let percentage = target_time_ms as f64 / max_time as f64;
    
            // Get the timeline bar's location and size
            let rect = timeline_bar.rect().await?;
            let target_x = rect.x + (rect.width * percentage); // Calculate target X position
            let target_y = rect.y + (rect.height / 2.0); // Middle of the timeline bar
    
            let offset_x: i64 = ((target_x - rect.x) as i32).into();
            let offset_y: i64 = 0; // No vertical movement needed
    
            // Move to timeline position and click
            driver.action_chain()
                .move_to_element_center(&timeline_bar) // Moves to the center of the timeline
                .move_by_offset(offset_x, offset_y) // Adjust offset based on percentage
                .click()
                .perform()
                .await?;
    
            println!("Skipped to {}ms ({:.2}% of the video).", target_time_ms, percentage * 100.0);
        } else {
            println!("Error: WebDriver instance is not initialized.");
            return Err(WebDriverError::NotFound("Invalid time format.".into(), "nice".into()));
        }
        
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct CookieData {
    domain: String,
    name: String,
    path: String,
    value: String,
    secure: bool,
}

fn convert_time_format(time_str: &str) -> Option<u64> {
    let parts: Vec<&str> = time_str.split('.').collect();
    
    let hours: u64 = parts.get(0)?.parse().ok()?; // Extract hours
    let minutes: u64 = parts.get(1).unwrap_or(&"0").parse().ok()?; // Extract minutes

    if minutes >= 60 { return None; } // Prevent invalid minute values

    let total_minutes = (hours * 60) + minutes;
    Some(total_minutes * 60 * 1000) // Convert to milliseconds
}
async fn add_cookies(driver: &WebDriver, cookies: &[CookieData]) -> WebDriverResult<()> {
    for cookie in cookies {
        let mut webdriver_cookie = Cookie::new(&cookie.name, &cookie.value);
            webdriver_cookie.set_domain(&cookie.domain);
            webdriver_cookie.set_path(&cookie.path);
            webdriver_cookie.set_secure(cookie.secure);

        driver.add_cookie(webdriver_cookie).await?;
    }
    Ok(())
}
