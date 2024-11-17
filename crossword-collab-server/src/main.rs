pub mod db;
pub mod server;

use db::postgres;
use server::api_server;

#[tokio::main]
async fn main() {
    // db::postgres::connect_to_db().await;
    // api_server::start().await;
    let db = db::postgres::DB::new();
    let guid = String:: from("123456");
    if let Some(board) = db::postgres::DB::get_board(&db, guid).await {
        print!("{:?}", board);
    }
}


