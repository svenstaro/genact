use anyhow::Result;
use yansi::Paint;

#[async_std::main]
async fn main() -> Result<()> {
    Paint::enable_windows_ascii();

    genact::main().await?;
    Ok(())
}
