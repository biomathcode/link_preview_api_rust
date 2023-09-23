use axum::{ extract::{ Extension, Query }, response::Json, routing::get, Router, http::StatusCode };
use reqwest::Client;
use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
struct WebsiteData {
    title: String,
    description: String,
    favicon: String,
    og_image: String,
}

#[derive(Deserialize)]
struct WebsiteQuery {
    url: String,
}

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let http_client = reqwest::Client::new();

    let app = Router::new().route("/", get(handler)).layer(Extension(http_client));

    // Run it with hyper on localhost:3000
    axum::Server
        ::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service()).await
        .unwrap();
}

async fn handler(
    Extension(http_client): Extension<Client>,
    Query(query): Query<WebsiteQuery>
) -> Result<Json<WebsiteData>, StatusCode> {
    let response = http_client
        .get(&query.url) // Use the URL from the query parameter
        .send().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = response.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let document = Html::parse_document(&body);

    let title_selector = Selector::parse("title").unwrap();
    let description_selector = Selector::parse("meta[name='description']").unwrap();
    let favicon_selector = Selector::parse("link[rel='icon']").unwrap();
    let og_image_selector = Selector::parse("meta[property='og:image']").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|title| title.text().collect::<String>())
        .unwrap_or_else(|| "Title not found".to_string());

    // Extract the website description
    let description = document
        .select(&description_selector)
        .next()
        .and_then(|desc| desc.value().attr("content"))
        .map(|desc| desc.to_string())
        .unwrap_or_else(|| "Description not found".to_string());

    // Extract the favicon URL
    let favicon = document
        .select(&favicon_selector)
        .next()
        .and_then(|icon| icon.value().attr("href"))
        .map(|icon| icon.to_string())
        .unwrap_or_else(|| "Favicon not found".to_string());

    let og_image = document
        .select(&og_image_selector)
        .next()
        .and_then(|og| og.value().attr("content"))
        .map(|og| og.to_string())
        .unwrap_or_else(|| "og:image not found".to_string());

    let website_data = WebsiteData {
        title,
        description,
        favicon,
        og_image,
    };

    Ok(Json(website_data))
}
