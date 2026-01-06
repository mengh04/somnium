pub mod media;
pub mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::app::run();
    Ok(())
}
