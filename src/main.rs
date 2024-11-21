use clickhouse::Row;
use serde::Serialize;

mod db;

#[derive(Row, Serialize)]
struct Account{
    pubkey: String,
    lamports: u64,
    owner: String,
    executable: bool,
    rent_epoch: u64,
    data: String,
    write_version: u64,
    update_at: String, 
    slot: u64
}

#[derive(Row, Serialize)]
struct Transaction{
    signature:String,
    slot: u64,
    success: bool,
    fee: u64,
    block_time: String,
    program_id: String,
    instructions: Vec<String> 

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // step 1: create connection with clickhous
    let conn = db::ClickhouseConnection::new();
    let db_name = String::from("SOLANA"); 
    conn.create_db(&db_name).await?;
    conn.create_table(&db_name).await?;

    // step 3: add the data to table

    Ok(())
}
