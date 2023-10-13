use async_chat::utils::{self, ChatResult};
use async_std::prelude::*;
use async_std::{io, net};

fn main() {
    println!("client");
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
