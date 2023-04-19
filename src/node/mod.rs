use std::sync::Arc;
use tokio::io::{AsyncWrite, AsyncWriteExt, AsyncRead};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::AsyncReadExt;
use crate::msg::NetworkMessage;
use crate::{StreamingDAG, DAG, Federation};
use tokio_util::codec::{
    Framed, LengthDelimitedCodec,
};

pub struct Node;

pub(crate) async fn conn_handler(
    stream: TcpStream, 
    str_dag: Arc<StreamingDAG>,
    fed: Arc<Federation>,
) {
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    let mut buf =  [0; 1024];
    while let tx_bytes = framed.read_buffer() {
        let m: NetworkMessage = bincode::deserialize(&tx_bytes).unwrap();
        match m {
            NetworkMessage::Tx(t) => {
                let sender_id = t.send.clone().id;
                let sender_org = sender_id.org_id;
                let sender_fed = sender_org.clone().fed_id;

                let receiver_id = t.recv.clone().id;
                let receiver_org = receiver_id.org_id;
                let receiver_fed = receiver_org.fed_id;

                if sender_fed == receiver_fed {
                    str_dag.push_tx(t, sender_org).await;
                } else {
                    println!("Transaction between federations not supported yet.");
                    continue;
                }
            },
            NetworkMessage::ValidationReq(transaction) => {
                let validation = fed.validate_tx_distributed(&transaction).await;
                let resp = NetworkMessage::ValidationRes(transaction, validation);
                let respbytes = bincode::serialize(&resp).unwrap();
                // framed.write_buffer_mut().join(&respbytes.into());
            },
            NetworkMessage::ValidationRes(t, validation) => {
                if validation {
                    str_dag.confirm_tx(&t).await;
                } else {
                    // Handle invalid tx
                }
            }
        }
        let tx: crate::Transaction = bincode::deserialize(tx_bytes).unwrap();
        let org_id = tx.clone().send.id.org_id;
        let (send, recv, amt) = (tx.clone().send, tx.clone().recv, tx.clone().amt);
        str_dag.push_tx(tx, org_id).await;
    }
}

pub async fn server_start(stream_dag: Arc<StreamingDAG>, fed: Arc<Federation>) {
    let listener = TcpListener::bind("127.0.0.1:8787").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let strdagc = Arc::clone(&stream_dag);
        let fedc = Arc::clone(&fed);
        tokio::spawn(async move {
            conn_handler(stream, strdagc, fedc).await;
        });
    }
}
