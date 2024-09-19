use futures::stream::{self, StreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.vinmonopolet.no/vmpws/v2/vmp/search?q=:name-asc&currentPage=";
    let client = Arc::new(Client::new());
    let products_data = Arc::new(Mutex::new(Vec::new()));
    let mut current_page = 0;
    let mut total_pages = 1;

    while current_page < total_pages {
        let page_url = format!("{}{}", url, current_page);
        let response = client.get(&page_url).send().await?.text().await?;
        let json: Value = serde_json::from_str(&response)?;

        total_pages = json["productSearchResult"]["pagination"]["totalPages"]
            .as_u64()
            .unwrap_or(1) as usize;

        let products = json["productSearchResult"]["products"]
            .as_array()
            .unwrap_or(&Vec::new())
            .to_vec();

        let tasks = stream::iter(products)
            .map(|product| {
                let client = Arc::clone(&client);
                let products_data = Arc::clone(&products_data);
                tokio::spawn(async move {
                    if let Err(e) = process_product(&product, &client, &products_data).await {
                        eprintln!("Error processing product: {}", e);
                    }
                })
            })
            .buffer_unordered(50);

        tasks.collect::<Vec<_>>().await;

        current_page += 1;
        print!(
            "\rPage {}/{}. Products processed: {}",
            current_page,
            total_pages,
            products_data.lock().await.len()
        );
        std::io::stdout().flush()?;
    }

    let csv_file_name = "products_data.csv";
    let file = File::create(csv_file_name)?;
    let mut writer = csv::WriterBuilder::new().delimiter(b';').from_writer(file);

    for row in products_data.lock().await.iter() {
        writer.write_record(row)?;
    }

    println!("\nFinished. Data written to {}.", csv_file_name);
    Ok(())
}

async fn process_product(
    product: &Value,
    client: &Client,
    products_data: &Arc<Mutex<Vec<Vec<String>>>>,
) -> Result<(), Box<dyn Error>> {
    let product_code = product["code"].as_str().unwrap_or("");
    let product_url = format!("https://www.vinmonopolet.no/p/{}", product_code);
    let name = product["name"].as_str().unwrap_or("").to_string();
    let price = product["price"]["value"].as_f64().unwrap_or(0.0);
    let volume_cl = product["volume"]["value"].as_f64().unwrap_or(0.0);

    let product_page_response = client.get(&product_url).send().await?.text().await?;
    let alcohol_percentage = extract_alcohol_percentage(&product_page_response);

    let alcohol_cl = if alcohol_percentage != 0.0 {
        (alcohol_percentage * volume_cl * 100.0).round() / 100.0
    } else {
        0.0
    };

    let alcohol_price_per_cl = if alcohol_cl != 0.0 {
        (price / alcohol_cl * 100.0).round() / 100.0
    } else {
        0.0
    };

    let product_data = vec![
        product_url,
        name,
        format!("{:.2}", price),
        format!("{:.2}", volume_cl),
        format!("{:.2}", alcohol_percentage),
        format!("{:.2}", alcohol_cl),
        format!("{:.2}", alcohol_price_per_cl),
    ];

    products_data.lock().await.push(product_data);

    Ok(())
}

fn extract_alcohol_percentage(html: &str) -> f64 {
    let document = Html::parse_document(html);
    let selector = Selector::parse(".content-item-wLPXgMvT span").unwrap();

    document
        .select(&selector)
        .next()
        .and_then(|element| element.inner_html().parse::<String>().ok())
        .and_then(|percentage_text| {
            percentage_text
                .replace('%', "")
                .replace(',', ".")
                .parse::<f64>()
                .ok()
        })
        .unwrap_or(0.0)
        / 100.0
}
