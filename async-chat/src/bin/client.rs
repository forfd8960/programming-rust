use async_chat::utils::{self, ChatResult, FromServer};
use async_std::{io, net};
use async_std::{net::TcpStream, prelude::*, task};

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("");
    task::block_on(async {
        let socket = TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;
        Ok(())
    })
}

async fn send_commands(mut server: net::TcpStream) -> ChatResult<()> {
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(cmd_result) = command_lines.next().await {
        let command = cmd_result?;
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };

        utils::send_as_json(&mut server, &request).await?;
        server.flush().await?;
    }

    Ok(())
}

async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(msg) => {
                println!("error from server: {:?}", msg);
            }
        }
    }
    Ok(())
}
