use std::sync::Arc;

use async_chat::utils::ChatResult;
use async_std::{net, stream::StreamExt, task};

use crate::connection::serve;

mod connection;
mod group;
mod group_table;

fn main() -> ChatResult<()> {
    println!("server");

    let address = std::env::args().nth(1).expect("usage: server address");
    let chant_group_table = Arc::new(group_table::GroupTable::new());

    task::block_on(async {
        let listener = net::TcpListener::bind(address).await?;
        let mut new_conns = listener.incoming();
        while let Some(socket_result) = new_conns.next().await {
            let socket = socket_result?;
            let groups = chant_group_table.clone();
            task::spawn(async {
                log_error(serve(socket, groups).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}
