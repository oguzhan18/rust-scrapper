# Rust Scraper Library 1.0.0
**Author:** Oğuzhan ÇART <br>
**Instagram:** <a href="https://www.instagram.com/oguzhan_cart/"> @oguzhan_cart </a> <br>
**LinkedIn:** <a href="https://www.linkedin.com/in/o%C4%9Fuzhan-%C3%A7art-b73405199/"> Oğuzhan ÇART </a>

This library provides a powerful and easy-to-use tool for web scraping with Rust. It supports both synchronous and asynchronous operations and comes with additional features like caching, pagination, headless browser support, and more.

## Features
* **Synchronous and Asynchronous Scraping:** Flexibly choose between blocking and non-blocking scraping.
* **Exporting Capabilities:** Export the scraped data in JSON or CSV format.
* **Caching:** Automatically caches scraped content to reduce unnecessary requests and optimize performance.
* **Headless Browser Integration:** Scrape JavaScript-rendered content using a headless browser.
* **Rate Limiting:** Built-in delay mechanism to avoid overwhelming servers and being blocked.

## Installation
To use this library in your project, add the following dependency to your `Cargo.toml`:
```toml
[dependencies]
rust-scrapper = "1.0.0"
```

## Usage
### Basic Synchronous Scraping
You can scrape a website synchronously with just a few lines of code:
```rust
use rust_scrapper::RustScrapper;

fn main() {
    let mut scrapper = RustScrapper::new();
    let results = scrapper.scrape("https://example.com", "div").unwrap();
    for item in results {
        println!("{}", item);
    }
}
```
### Asynchronous Scraping
For asynchronous scraping, use the `scrape_async` method:
```rust
use rust_scrapper::RustScrapper;
use tokio::runtime::Runtime;

fn main() {
    let mut scrapper = RustScrapper::new();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let results = scrapper.scrape_async("https://example.com", "div").await.unwrap();
        for item in results {
            println!("{}", item);
        }
    });
}
```

### Scraping with Pagination
The library includes a convenient function for paginated scraping. Here’s an example of scraping multiple pages:
```rust
let mut scrapper = RustScrapper::new();
let results = scrapper.scrape_paginated("https://example.com", "page", 5, "div").unwrap();

for item in results {
    println!("{}", item);
}
```

### Scraping with a Delay (Rate Limiting)
To avoid overwhelming the server with requests, you can add a delay between scraping operations:

```rust
let mut scrapper = RustScrapper::new();
scrapper.scrape_with_delay("https://example.com", "div", 2).await.unwrap();
```
This example adds a 2-second delay between scraping requests.

#### Exporting Data
You can export scraped data to JSON or CSV using the `Exporter` struct:
**Export to JSON:**

```rust
let data = vec!["item1".to_string(), "item2".to_string()];
let json_data = Exporter::to_json(data);
println!("{}", json_data);
```
### Export to CSV:
```rust
let data = vec!["item1".to_string(), "item2".to_string()];
Exporter::to_csv(data, "output.csv").expect("Failed to write CSV file");
```
### Scraping JavaScript-Rendered Pages
The library also supports scraping pages that require JavaScript to fully render the content. Using a headless browser, you can fetch the content:
```rust
use rust_scrapper::JsScraper;

fn main() {
    let results = JsScraper::scrape_with_js("https://example.com", "div").unwrap();
    for item in results {
        println!("{}", item);
    }
}
```
### Contributing
Contributions are welcome! If you encounter any issues or have suggestions for improvements, please feel free to submit a pull request or open an issue on the GitHub repository.
### License
This project is licensed under the MIT License. See the LICENSE file for details.
### Contact
For any inquiries, feel free to reach out to the author:<br>
**Instagram:** <a href="https://www.instagram.com/oguzhan_cart/"> @oguzhan_cart </a> <br>
**LinkedIn:** <a href="https://www.linkedin.com/in/o%C4%9Fuzhan-%C3%A7art-b73405199/"> Oğuzhan ÇART </a>
