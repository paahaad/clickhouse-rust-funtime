use clickhouse::Row;
use serde::Serialize;
use time::OffsetDateTime;
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
    #[serde(with = "clickhouse::serde::time::datetime64::millis")]
    updated_at: OffsetDateTime, 
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
    let conn: db::ClickhouseConnection = db::ClickhouseConnection::new();
    let db_name: String = String::from("SOLANA"); 
    conn.create_db(&db_name).await?;
    conn.create_table(&db_name).await?;

    // step 3: add the data to table
    let mut insert: clickhouse::insert::Insert<Account> = conn.client.insert("accounts")?;
    insert.write(&Account {
        pubkey: "7ad7abf34a98e8ef4df54e3df8b993ca26ae18f7".to_string(),
        lamports: 1500000,
        owner: "11111111111111111111111111111111".to_string(),
        executable: false,
        rent_epoch: 328,
        data: "base64encodedaccountdata==".to_string(),
        write_version: 42,
        updated_at: OffsetDateTime::now_utc(), 
        slot: 153762389,
    }).await?;
    insert.end().await?;
    println!("Completed the task");
    Ok(())
}
