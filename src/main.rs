mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // step 1: create connection with clickhous
    let conn = db::ClickhouseConnection::new();
    let db_name = String::from("SOLANA"); 
    conn.create_db(&db_name).await?;
    conn.create_table(&db_name).await?;

    Ok(())
}
