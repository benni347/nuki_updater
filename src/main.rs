use reqwest::{Client, Error};
use reqwest::header::{HeaderMap, USER_AGENT};
use scraper::{Html, Selector};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let chrome_ua = "Mozilla/5.0 (Windows NT 10.0;Win64) AppleWebkit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.82 Safari/537.36";
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, chrome_ua.parse().unwrap());
    let resp = client.get("https://apkpure.com/nuki-smart-lock/io.nuki")
        .headers(headers)
        .send()
        .await?;

    let body = resp.text().await?;
    let dom = parse_document(<dyn TendrilSink>::new(), Default::default()).one(body);
    if dom.errors.is_empty() {
        println!("The HTML string is valid.");
    } else {
        println!("The HTML string is not valid: {:?}", dom.errors);
    }
    let document = Html::parse_document(&body);
    let selector = Selector::parse("p.details_sdk").unwrap();

    for span in document.select(&selector) {
        let span_text = span.text().next().unwrap();
        if span_text.len() > 2 {
            println!("{}", span_text);
        } else {
            println!("no span")
        }
        break;
    }
    Ok(())
}
