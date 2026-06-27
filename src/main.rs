use anyhow::Result;
use flow::{Args, execute};

#[tokio::main]
async fn main() -> Result<()> {
    ort::set_api(ort_tract::api());
    let args = Args::parse();
    execute(args.command).await
}
