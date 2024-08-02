use bdk::bitcoin::consensus::encode;
use bdk::bitcoin::network::constants::Network;
use bdk::bitcoin::util::address::Address;
use bdk::bitcoin::Amount;
use bdk::blockchain::esplora::EsploraBlockchain;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex;
use bdk::Wallet;
use bdk::SignOptions;
use bdk::Error;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct TicketEvent {
    name: String,
    price: u64,
    address: String,
}

#[wasm_bindgen]
impl TicketEvent {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, price: u64, address: String) -> TicketEvent {
        TicketEvent { name, price, address }
    }
}

#[wasm_bindgen]
pub fn create_wallet() -> String {
    let wallet = Wallet::new(
        "wpkh([c0123456/84'/0'/0']tpubD6NzVbkrYhZ4W6Bh5xmhs1FSnbvX23pEzyL5QSKziXYXSKZXUNkFLZAKf8DL7PSL6cmWW5BLaepuD3kQnEw8QoXBWQroiyxKHDUNeLxh5uT/0/*)",
        None,
        Network::Testnet,
        MemoryDatabase::default(),
        EsploraBlockchain::new("https://blockstream.info/testnet/api", 1),
    ).unwrap();

    let address = wallet.get_address(AddressIndex::New).unwrap();
    address.to_string()
}

#[wasm_bindgen]
pub fn create_ticket_event(name: &str, price: u64) -> JsValue {
    let address = create_wallet();
    let event = TicketEvent {
        name: name.to_string(),
        price,
        address,
    };
    JsValue::from_serde(&event).unwrap()
}

#[wasm_bindgen]
pub fn purchase_ticket(event: &JsValue) -> String {
    let event: TicketEvent = event.into_serde().unwrap();
    let txid = perform_transaction(&event.address, event.price);
    match txid {
        Ok(txid) => format!("Transaction sent: {}", txid),
        Err(e) => format!("Transaction failed: {:?}", e),
    }
}

fn perform_transaction(to_address: &str, amount: u64) -> Result<String, Error> {
    let wallet = Wallet::new(
        "wpkh([c0123456/84'/0'/0']tpubD6NzVbkrYhZ4W6Bh5xmhs1FSnbvX23pEzyL5QSKziXYXSKZXUNkFLZAKf8DL7PSL6cmWW5BLaepuD3kQnEw8QoXBWQroiyxKHDUNeLxh5uT/0/*)",
        None,
        Network::Testnet,
        MemoryDatabase::default(),
        EsploraBlockchain::new("https://blockstream.info/testnet/api", 1),
    )?;

    let to_address = Address::from_str(to_address)?;

    let mut builder = wallet.build_tx();
    builder
        .add_recipient(to_address.script_pubkey(), amount)
        .enable_rbf();

    let (mut psbt, _) = builder.finish()?;

    wallet.sign(&mut psbt, SignOptions::default())?;

    let tx = psbt.extract_tx();
    let txid = wallet.broadcast(&tx)?;

    Ok(txid.to_string())
}