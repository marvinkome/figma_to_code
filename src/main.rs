mod figma;

use dotenv::dotenv;
use std::fs::File;
use std::io::ErrorKind;

#[tokio::main]
async fn main() {
    // setup
    dotenv().ok();

    // init
    // let figma_token = std::env::var("FIGMA_TOKEN").expect("FIGMA_TOKEN needs to set");
    // let figma = Figma::new(figma_token);
    // figma.get_file("L2XXjhJpf3MbBQIrf7uqdY").await;

    // use temporary file for now
    let file = File::open("response.json").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("File 'response.json' not found");
        } else {
            panic!("Problem opening file 'response.json': {:?}", error);
        }
    });

    let reader = std::io::BufReader::new(file);
    let json: figma::types::File = serde_json::from_reader(reader).unwrap();

    figma::run(json);
}
