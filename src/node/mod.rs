use std::sync::Arc;
use tokio::{net::{
    TcpStream,  TcpListener,
}, io::AsyncReadExt};
use crate::{StreamingDAG, DAG};
use tokio_util::codec::{
    Framed, LengthDelimitedCodec,
};

pub(crate) async fn conn_handler(stream: TcpStream, str_dag: Arc<StreamingDAG>) {
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    let mut buf =  [0; 1024];
    while let tx_bytes = framed.read_buffer() {
        let tx: crate::Transaction = bincode::deserialize(tx_bytes).unwrap();
        let (send, recv, amt) = (tx.send, tx.recv, tx.amt);
        str_dag.push_tx(tx, org_id).await;
    }
}
