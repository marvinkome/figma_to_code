mod api;

use dotenv::dotenv;
use api::Figma;

#[tokio::main]
async fn main() {
    // setup
    dotenv().ok();

    // init
    let figma_token = std::env::var("FIGMA_TOKEN").expect("FIGMA");
    let figma = Figma::new(figma_token);

    figma.get_file("L2XXjhJpf3MbBQIrf7uqdY").await;
}
