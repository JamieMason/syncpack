use {
  crate::{context::Context, effects::ui::Ui},
  reqwest::{header::ACCEPT, Client},
  std::error::Error,
};

/// Run the update command side effects
pub async fn run(ctx: Context) -> Result<Context, Box<dyn Error>> {
  let ui = Ui { ctx: &ctx };

  // Create a reqwest client
  let client = Client::new();

  // Send a GET request with the Accept header set to application/json
  let response = client
    .get("https://icanhazdadjoke.com")
    .header(ACCEPT, "application/json")
    .send()
    .await?;

  // Ensure the request was successful
  if response.status().is_success() {
    // Parse the response body as JSON
    let json: serde_json::Value = response.json().await?;
    println!("Response JSON: {:?}", json);
  } else {
    println!("Request failed with status: {}", response.status());
  }

  Ok(ctx)
}
