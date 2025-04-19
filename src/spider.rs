use crate::utils::{clean_link, validate_link};
use crate::db::create_tables;
use crate::config::Config;
use reqwest;
use rusqlite::{params, Connection};
use select::document::Document;
use select::predicate::Name;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

/// Starts the crawling process based on the provided configuration.
pub async fn run_crawl(config: Config) -> Result<(), Box<dyn Error>> {
    let conn = Arc::new(Mutex::new(Connection::open(&config.database)?));
    let visited = Arc::new(Mutex::new(Vec::new()));
    create_tables(&conn)?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str(&config.user_agent)?,
    );
    if !config.spoof_ip.is_empty() {
        for &name in &["X-Forwarded-For", "X-Originating-IP", "X-Remote-IP", "X-Remote-Addr"] {
            headers.insert(name, reqwest::header::HeaderValue::from_str(&config.spoof_ip)?);
        }
    }
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()?;
    let base_url = Url::parse(&config.url)?;
    let base_domain = base_url.domain().ok_or("Invalid start URL domain")?.to_string();
    let www_domain = format!("www.{}", base_domain);
    let mut queue = VecDeque::new();
    queue.push_back(config.url.clone());
    while let Some(url) = queue.pop_front() {
        {
            let visited_guard = visited.lock().unwrap();
            if visited_guard.contains(&url) {
                continue;
            }
        }
        if let Err(e) = crawl_and_store(&url, &client, &conn) {
            eprintln!("Error crawling {}: {}", url, e);
            continue;
        }
        visited.lock().unwrap().push(url.clone());
        let body = client.get(&url).send()?.text()?;
        for node in Document::from(body.as_str()).find(Name("a")) {
            if let Some(raw_href) = node.attr("href") {
                let cleaned = clean_link(raw_href);
                let absolute = match Url::parse(&cleaned) {
                    Ok(u) => u,
                    Err(_) => Url::parse(&url)?.join(&cleaned)?,
                };
                let absolute_url = absolute.to_string();
                if !validate_link(&absolute_url) {
                    continue;
                }
                let Some(domain) = absolute.domain() else { continue };
                let is_main = domain == base_domain || domain == www_domain;
                let is_sub = config.crawl_subs && domain.ends_with(&format!(".{}", base_domain));
                if config.crawl_everything {
                    let _ = crawl_and_store(&absolute_url, &client, &conn);
                }
                if is_main || is_sub {
                    queue.push_back(absolute_url);
                }
            }
        }
        sleep(Duration::from_secs(config.throttle_secs)).await;
    }
    second_pass(&client, &conn, &base_domain, &www_domain, &config).await?;
    println!("Crawl complete. Visited {} URLs.", visited.lock().unwrap().len());
    Ok(())
}
/// Second pass for entries that may not have been fully processed earlier.
async fn second_pass(
    client: &reqwest::blocking::Client,
    conn: &Arc<Mutex<Connection>>,
    base_domain: &str,
    www_domain: &str,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let urls: Vec<String> = {
        let db = conn.lock().unwrap();
        let mut stmt = db.prepare("SELECT url FROM links WHERE status_code IS NULL")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        let result = rows.filter_map(Result::ok).collect();
        result
    };
    for url in urls {
        let Ok(parsed) = Url::parse(&url) else { continue };
        let Some(domain) = parsed.domain() else { continue };
        let is_main = domain == base_domain || domain == www_domain;
        let is_sub = config.crawl_subs && domain.ends_with(&format!(".{}", base_domain));
        if config.crawl_everything || is_main || is_sub {
            let _ = crawl_and_store(&url, client, conn);
            sleep(Duration::from_secs(config.throttle_secs)).await;
        }
    }
    Ok(())
}
/// Fetches a URL and stores its metadata in the database.
fn crawl_and_store(
    url: &str,
    client: &reqwest::blocking::Client,
    conn: &Arc<Mutex<Connection>>,
) -> Result<(), Box<dyn Error>> {
    let response = client.get(url).send()?;
    let status = response.status().as_u16() as i64;
    let body = response.text()?;
    let has_form = if Document::from(body.as_str())
        .find(Name("form"))
        .next()
        .is_some()
    {
        "y"
    } else {
        "n"
    };
    {
        let db = conn.lock().unwrap();
        db.execute(
            "INSERT OR IGNORE INTO links (url, status_code, has_form) VALUES (?1, ?2, ?3)",
            params![url, status, has_form],
        )?;
    }
    println!("Crawled: {} [{}] Form: {}", url, status, has_form);
    Ok(())
}
