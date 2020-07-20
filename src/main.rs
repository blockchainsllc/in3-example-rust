use futures_executor::block_on;
use in3::eth1::*;
use in3::json_rpc::json::*;
use in3::prelude::*;

fn eth_block_number() -> In3Result<()> {
    // configure client and API
    let client = Client::new(chain::MAINNET);
    let mut eth_api = Api::new(client);

    // get block number
    let number = block_on(eth_api.block_number())?;
    println!("Latest block number => {:?}", number);

    Ok(())
}

fn eth_get_block_by_number() -> In3Result<()> {
    // configure client and API
    let mut eth_api = Api::new(Client::new(chain::MAINNET));

    // get latest block
    let block: Block = block_on(eth_api.get_block_by_number(BlockNumber::Latest, false))?;
    println!("Block => {:?}", block);

    Ok(())
}

fn eth_call() -> In3Result<()> {
    // configure client and API
    let mut eth_api = Api::new(Client::new(chain::MAINNET));

    // ABI encode params
    let contract: Address = from_str(r#""0x2736D225f85740f42D17987100dc8d58e9e16252""#).unwrap();
    let mut abi = abi::In3EthAbi::new();
    let params = block_on(abi.encode("totalServers():uint256", json!([])))
        .expect("failed to ABI encode params");

    // create transaction to call
    let txn = CallTransaction {
        to: Some(contract),
        data: Some(params),
        ..Default::default()
    };
    let output: Bytes = block_on(eth_api.call(txn, BlockNumber::Latest)).expect("ETH call failed");

    // ABI decode output
    let output = block_on(abi.decode("uint256", output)).expect("failed to ABI decode output");
    let total_servers: U256 = from_value(output).unwrap();
    println!("Total servers => {:?}", total_servers);

    Ok(())
}

fn main() -> In3Result<()> {
    eth_block_number()?;
    eth_get_block_by_number()?;
    eth_call()?;
    Ok(())
}
