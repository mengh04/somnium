mod media;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::app::run();
    Ok(())
}
