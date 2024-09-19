//! # Rust Scraper Library 1.0.0
//! # Author Oğuzhan ÇART Instagram: @oguzhan_cart LinkedIn: https://www.linkedin.com/in/o%C4%9Fuzhan-%C3%A7art-b73405199/
//! 
//! This library provides easy-to-use, efficient scraping tools for developers. 
//! It supports both synchronous and asynchronous scraping operations. 
//! Additionally, it includes features like pagination support, caching, exporting data in multiple formats, and headless browser integration.
//!
//! ## Features
//! - Synchronous and asynchronous scraping
//! - Pagination support
//! - JSON and CSV export capabilities
//! - Caching for performance optimization
//! - Headless browser integration for JavaScript-rendered pages
//! - Rate limiting to prevent overloading websites
//! 
//! ## Example Usage
//!
//! ```rust
//! let scrapper = RustScrapper::new();
//! let results = scrapper.scrape("https://example.com", "div").unwrap();
//! ```

use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error};
use std::fs::File;
use std::io::Write;

/// Trait for basic scraping operations. 
/// This allows us to extend scraping functionality easily in the future.
pub trait Scraper {
    fn scrape(&self, url: &str, element: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn scrape_async(&self, url: &str, element: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

/// Struct to hold cache functionality. 
/// Responsible only for managing cached scraping data.
pub struct ScrapeCache {
    cache: HashMap<String, Vec<String>>,
}

impl ScrapeCache {
    /// Creates a new instance of the cache.
    pub fn new() -> Self {
        ScrapeCache {
            cache: HashMap::new(),
        }
    }

    /// Retrieves cached data if available.
    pub fn get(&self, url: &str) -> Option<&Vec<String>> {
        self.cache.get(url)
    }

    /// Sets new data into the cache.
    pub fn set(&mut self, url: &str, data: Vec<String>) {
        self.cache.insert(url.to_string(), data);
    }
}

/// Struct that manages the scraping logic.
/// Implements the Scraper trait for synchronous and asynchronous scraping.
pub struct RustScrapper {
    client: Client,
    cache: ScrapeCache,
}

impl RustScrapper {
    /// Creates a new instance of the RustScrapper with caching enabled.
    pub fn new() -> Self {
        RustScrapper {
            client: Client::new(),
            cache: ScrapeCache::new(),
        }
    }

    /// Scraping with rate limiting between requests.
    /// This can be used to prevent being blocked by websites due to too many requests.
    pub async fn scrape_with_delay(
        &self,
        url: &str,
        element: &str,
        delay: u64,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        sleep(Duration::from_secs(delay)).await;
        self.scrape_async(url, element).await
    }

    /// Scrapes paginated content from multiple pages.
    /// `pages` is the number of pages to scrape, and `page_param` is the query parameter used for pagination.
    pub fn scrape_paginated(
        &self,
        base_url: &str,
        page_param: &str,
        pages: usize,
        element: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        for page in 1..=pages {
            let url = format!("{}?{}={}", base_url, page_param, page);
            let page_results = self.scrape(&url, element)?;
            results.extend(page_results);
        }
        Ok(results)
    }
}

/// Sync scraping operations.
/// This is an implementation of the `Scraper` trait for synchronous scraping.
impl Scraper for RustScrapper {
    /// Scrape synchronously.
    /// It fetches the page content and parses the HTML using the provided CSS selector.
    fn scrape(&mut self, url: &str, element: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if let Some(cached_data) = self.cache.get(url) {
            info!("Cache hit for URL: {}", url);
            return Ok(cached_data.clone());
        }

        let body = reqwest::blocking::get(url)?.text()?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse(element).map_err(|e| format!("Selector parse error: {:?}", e))?;

        let results = document
            .select(&selector)
            .map(|elem| elem.inner_html())
            .collect::<Vec<_>>();

        self.cache.set(url, results.clone());
        Ok(results)
    }

    /// Scrape asynchronously.
    /// It asynchronously fetches the page content and parses the HTML using the provided CSS selector.
    async fn scrape_async(&mut self, url: &str, element: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if let Some(cached_data) = self.cache.get(url) {
            info!("Cache hit for URL: {}", url);
            return Ok(cached_data.clone());
        }

        let response = self.client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&response);
        let selector = Selector::parse(element).map_err(|e| format!("Selector parse error: {:?}", e))?;

        let results = document
            .select(&selector)
            .map(|elem| elem.inner_html())
            .collect::<Vec<_>>();

        self.cache.set(url, results.clone());
        Ok(results)
    }


/// Handles exporting scraped data to different formats.
pub struct Exporter;

impl Exporter {
    /// Exports data to JSON format.
    pub fn to_json(data: Vec<String>) -> String {
        serde_json::json!(data).to_string()
    }

    /// Exports data to a CSV file.
    pub fn to_csv(data: Vec<String>, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        for row in data {
            writeln!(file, "{}", row)?;
        }
        Ok(())
    }
}

/// Scrape JavaScript-rendered pages using headless browser.
/// This uses `headless_chrome` to load and scrape websites that require JavaScript execution.
pub struct JsScraper;

impl JsScraper {
    pub fn scrape_with_js(url: &str, element: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let browser = headless_chrome::Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;

        tab.navigate_to(url)?.wait_until_navigated()?;
        let body = tab.find_element(element)?
                      .call_js_fn("function() { return this.innerHTML; }", vec![])?
                      .value
                      .unwrap_or_default()
                      .to_string();
        Ok(vec![body])
    }
}
