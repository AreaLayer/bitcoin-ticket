use std::net::UdpSocket;
use std::io::{self, IoSlice, IoSliceMut};
use std::str::FromStr;
use bdk::wallet::AddressIndex;
use bdk::Wallet;
use bdk::SignOptions;
use bdk::Error;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_wallet() -> Result<String, JsValue> {
    let wallet = Wallet::new(
        Network::Testnet,
        MemoryDatabase::default(),
        EsploraBlockchain::new("https://blockstream.info/testnet/api", 1),
    ).map_err(|e| JsValue::from_str(&format!("Failed to create wallet: {:?}", e)))?;

    Ok(wallet.to_string())
}#[derive(Serialize, Deserialize)]
pub struct ConsensusEncode {
    pub encoded: String,
}

impl Clone for ConsensusEncode {
    fn clone(&self) -> Self {
        ConsensusEncode {
            encoded: self.encoded.clone(),
        }
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
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
pub fn create_wallet() -> Result<String, JsValue> {
    impl wallet::Wallet {
        fn new(
            network: Network,
            address: Address,
            amount: Amount,
            wallet_type: WalletType,
        ) -> Result<Self, Error> {
            let wallet = Wallet::new(
                network,
                MemoryDatabase::default(),
                EsploraBlockchain::new("https://blockstream.info/testnet/api", 1),
            ).map_err(|e| Error::Generic(format!("Failed to create wallet: {:?}", e)))?;

            Ok(Self {
                wallet,
                address,
                amount,
                wallet_type,
            })
        }

    }

        pub fn get_address(&self) -> Result<String, JsValue> {
            let address = self.wallet.get_address(AddressIndex::New)
                .map_err(|e| JsValue::from_str(&format!("Failed to get address: {:?}", e)))?;
            Ok(address.to_string())
        }
    }

    #[wasm_bindgen]
    pub fn create_wallet() -> Result<String, JsValue> {
        let wallet = Wallet::new(
            Network::Testnet,
            MemoryDatabase::default(),
            EsploraBlockchain::new("https://blockstream.info/testnet/api", 1),
        ).map_err(|e| JsValue::from_str(&format!("Failed to create wallet: {:?}", e)))?;

        let address = wallet.get_address(AddressIndex::New)
            .map_err(|e| JsValue::from_str(&format!("Failed to get address: {:?}", e)))?;
        Ok(address.to_string())
    }

    #[wasm_bindgen]
    pub fn create_ticket_event(name: &str, price: u64) -> Result<JsValue, JsValue> {
        let address = create_wallet()?;
        let event = TicketEvent {
            name: name.to_string(),
            price,
            address,
        };
        JsValue::from_serde(&event).map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    }
#[wasm_bindgen]
pub fn purchase_ticket(event: &JsValue) -> Result<String, JsValue> {
    let event: TicketEvent = event.into_serde().map_err(|e| JsValue::from_str(&format!("Deserialization error: {:?}", e)))?;
    let txid = perform_transaction(&event.address, event.price).map_err(|e| JsValue::from_str(&format!("Transaction error: {:?}", e)))?;
    Ok(format!("Transaction sent: {}", txid))
}

fn perform_transaction(to_address: &str, amount: u64) -> Result<String, Error> {
    let wallet = Wallet::new(
        "wpkh([c0123456/84'/0'/0']tpubD6NzVbkrYhZ4W6Bh5xmhs1FSnbvX23pEzyL5QSKziXYXSKZXUNkFLZAKf8DL7PSL6cmWW5BLaepuD3kQnEw8QoXBWQroiyxKHDUNeLxh5uT/0/*)",
        None,
        Network::Testnet,
        MemoryDatabase::default(),
        EsploraBlockchain::new("https://blockstream.info/testnet/api", 1),
    )?;

    let to_address = Address::from_str(to_address)
        .map_err(|e| format!("Invalid address format: {:?}", e))?;

    let mut builder = wallet.build_tx();
    builder
        .add_recipient(to_address.script_pubkey(), amount)
        .enable_rbf();

    let (mut psbt, _) = builder.finish()
        .map_err(|e| format!("Failed to build transaction: {:?}", e))?;

    wallet.sign(&mut psbt, SignOptions::default())
        .map_err(|e| format!("Failed to sign transaction: {:?}", e))?;

    let tx = psbt.extract_tx();
    let txid = wallet.broadcast(&tx)
        .map_err(|e| format!("Failed to broadcast transaction: {:?}", e))?;

    Ok(txid.to_string())
}

// Define the trait
pub trait WritevExt {
    fn writev(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize>;
    fn readv(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>;
}

// Implement the trait for UdpSocket
impl WritevExt for UdpSocket {
    fn writev(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.send_vectored(bufs)
    }

    fn readv(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.recv_vectored(bufs)
    }
}

// Example usage of UdpSocket
pub fn udp_socket_example() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Couldn't bind to address");

    let buf1 = b"Hello";
    let buf2 = b"World";
    let bufs = [IoSlice::new(buf1), IoSlice::new(buf2)];

    socket.writev(&bufs).expect("writev failed");

    let mut buf1 = [0; 5];
    let mut buf2 = [0; 5];
    let mut bufs = [IoSliceMut::new(&mut buf1), IoSliceMut::new(&mut buf2)];

    socket.readv(&mut bufs).expect("readv failed");
}
