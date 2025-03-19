use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let mut args = std::env::args().skip(1);
    let csv_path: String = args.next().ok_or(
        "First argument must be the path to the csv file").unwrap();

    let ctx = SessionContext::new();
    let df = ctx.read_csv(csv_path, CsvReadOptions::new()).await?;
    df.show().await?;
    Ok(())
}
