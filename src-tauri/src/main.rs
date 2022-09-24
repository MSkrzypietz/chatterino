use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(message) = message {
                println!(
                    "Received from {}: {}",
                    message.sender.name, message.message_text
                );
            }
        }
    });

    let channel = std::env::var("CHANNEL").unwrap();
    client.join(channel).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    join_handle.await.unwrap();
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    if name.contains(' ') {
        Err("Name shoudl not contain spaces".to_string())
    } else {
        Ok(format!("Hello, {}", name))
    }
}
