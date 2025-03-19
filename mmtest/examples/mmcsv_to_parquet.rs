use datafusion::{dataframe::DataFrameWriteOptions, prelude::*};

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let csv_path: String = std::env::args().skip(1).next().ok_or(
        "First argument must be the csv path").unwrap();

    let mut parquet_path: String = csv_path.split('.').next().unwrap().to_string();
    parquet_path.push_str(".parquet");

    println!("Reading from {}", csv_path);
    println!("Writing to {}", parquet_path);
    
    let ctx = SessionContext::new();
    ctx.read_csv(&csv_path, CsvReadOptions::new())
        .await?
        .write_parquet(
            &parquet_path,
            DataFrameWriteOptions::new(),
            None,
        )
        .await?;

    Ok(())
}
