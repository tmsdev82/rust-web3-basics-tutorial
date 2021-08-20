use std::env;
use std::str::FromStr;

use web3::types::{H160, U256};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    let wei_conv: U256 = U256::exp10(18);
    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!(
            "Eth balance of {:?}: {}",
            account,
            balance.checked_div(wei_conv).unwrap()
        );
    }

    Ok(())
}
