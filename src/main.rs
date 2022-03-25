use axum::{extract::Path, response::Json, routing::get, Router};
use serde::Serialize;
use serde_json::{json, Value};
use std::env;
use std::net::SocketAddr;
use web3::types::{H160, U256};

// Defining structs
#[derive(Serialize)]
struct Balance {
    balance: U256,
    address: H160,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /balance/:address` goes to `get_balance`
        .route("/balance/:address", get(get_balance));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("rust-web3 listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Rust Web3 is working properly!"
}

async fn get_balance(Path(address): Path<H160>) -> Json<Value> {
    let wei = fetch_balance(address).await.unwrap();
    println!("Wei balance is: {}", wei);
    Json(json!(Balance {
        balance: wei,
        address: address
    }))
}

async fn fetch_balance(address: H160) -> web3::Result<U256> {
    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);
    let balance = web3s.eth().balance(address, None).await?;
    Ok(balance)
}