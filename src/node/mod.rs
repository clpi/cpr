use std::sync::Arc;
use tokio::io::{AsyncWrite, AsyncWriteExt, AsyncRead};
use std::net::{TcpStream, TcpListener};
use tokio::io::AsyncReadExt;
use crate::{StreamingDAG, DAG};
use tokio_util::codec::{
    Framed, LengthDelimitedCodec,
};

pub struct Node;

pub(crate) async fn conn_handler(stream: TcpStream, str_dag: Arc<StreamingDAG>) {
    // let mut framed = Framed::new(stream.into(), LengthDelimitedCodec::new());
    // let mut buf =  [0; 1024];
    // while let tx_bytes = framed.read_buffer() {
    //     let tx: crate::Transaction = bincode::deserialize(tx_bytes).unwrap();
    //     let org_id = tx.send.id.org_id;
    //     let (send, recv, amt) = (tx.send, tx.recv, tx.amt);
    //     str_dag.push_tx(tx, org_id).await;
    // }
}
