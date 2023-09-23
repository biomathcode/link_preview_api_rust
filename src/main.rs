mod website_fetcher; // Import the new module

use axum::{ extract::{ Extension, Query }, response::Json, routing::get, Router, http::StatusCode };
use reqwest::Client;

use serde::Deserialize;
use url::Url; // Import the url crate

use website_fetcher::{ fetch_website_data, WebsiteData }; // Import the function

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
    // Parse the URL and validate it
    let url = match Url::parse(&query.url) {
        Ok(url) => url,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        } // Invalid URL format
    };

    // Ensure the URL has a scheme (e.g., http or https)
    if url.scheme().is_empty() {
        return Err(StatusCode::BAD_REQUEST); // Missing scheme
    }

    // Fetch website data using the new function
    let website_data = fetch_website_data(&http_client, &url).await.map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;

    Ok(Json(website_data))
}
