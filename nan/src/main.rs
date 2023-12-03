mod game;

use mage_core::{run, Config};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::game::Game;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    let filter = EnvFilter::from_default_env()
        .add_directive("wgpu=warn".parse().unwrap())
        .add_directive("mage-core=trace".parse().unwrap())
        .add_directive("nan=trace".parse().unwrap());
    tracing_subscriber::fmt::fmt()
        .without_time()
        .compact()
        .with_env_filter(filter)
        .init();

    info!("Starting...");

    let game = Game::new();
    let config = Config {
        font: mage_core::Font::Custom(
            mage_core::load_font_image(include_bytes!("../../mage-core/examples/font3.png"))
                .unwrap(),
        ),
        title: Some("Naughty and Nice".into()),
        ..Default::default()
    };

    let _ = run(game, config).await;
}
