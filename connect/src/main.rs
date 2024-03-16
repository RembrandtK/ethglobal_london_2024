use web3::types::Address;
use web3::contract::{Contract, Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http = web3::transports::Http::new("https://stylus-testnet.arbitrum.io/rpc")?;
    let web3 = web3::Web3::new(http);
    let contract_address: Address = "0xa766f1EbC4AaBEFFD5F3F2Fc4Fb589D4c85c2D93".parse().unwrap();

    let contract_abi: Vec<u8> = include_bytes!("../../chain/contract_abi.json").to_vec();

    println!("Contract ABI: {:?}", contract_abi);
    let contract = Contract::from_json(web3.eth(), contract_address, &contract_abi)?;

    println!("Contract: {:?}", contract);

    let result = contract.query("number", (), None, Options::default(), None);
    let value: u32 = result.await?;

    println!("Number: {}", value);

    Ok(())
}
