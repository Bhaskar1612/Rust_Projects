use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use std::sync::Arc;
use std::error::Error;
use tokio::sync::Mutex;
use dashmap::DashMap;
use tokio::task;
use url::Url;

async fn fetch_url(client: &Client, url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn extract_links(base_url: &str, html: &str) -> Vec<String> {
    let document = Document::from(html);
    let base_url = Url::parse(base_url).expect("Invalid base URL");
    document.find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_url.join(link).ok())
        .map(|u| u.to_string())
        .collect()
}

async fn crawl(client: &Client, url: String, seen: Arc<DashMap<String, bool>>, to_visit: Arc<Mutex<Vec<String>>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if seen.contains_key(&url) {
        return Ok(());
    }
    seen.insert(url.clone(), true);

    if let Ok(body) = fetch_url(client, &url).await {
        let links = extract_links(&url, &body);
        let mut to_visit_guard = to_visit.lock().await;
        for link in links {
            if !seen.contains_key(&link) {
                to_visit_guard.push(link);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let start_url = "http://example.com";
    let seen = Arc::new(DashMap::new());
    let to_visit = Arc::new(Mutex::new(vec![start_url.to_string()]));
    let client = Client::new();

    let mut handles = vec![];

    for _ in 0..10 {
        let seen = Arc::clone(&seen);
        let to_visit = Arc::clone(&to_visit);
        let client = client.clone();

        let handle = task::spawn(async move {
            while let Some(url) = {
                let mut to_visit_guard = to_visit.lock().await;
                to_visit_guard.pop()
            } {
                if let Err(e) = crawl(&client, url, seen.clone(), to_visit.clone()).await {
                    eprintln!("Error crawling: {}", e);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task error: {:?}", e);
        }
    }

    Ok(())
}
