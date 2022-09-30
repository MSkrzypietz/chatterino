use serde::Serialize;
use tauri::{Manager, RunEvent};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

#[derive(Debug, Clone, Serialize)]
struct Payload {
    username: String,
    content: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let channel = std::env::var("CHANNEL").unwrap();
    client.join(channel).unwrap();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let app_handle = app.app_handle();
    tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(message) = message {
                let username = message.sender.name;
                let content = message.message_text;
                let payload = Payload {
                    username: username.clone(),
                    content: content.clone(),
                };

                match app_handle.emit_all("on_message", payload) {
                    Ok(_) => println!("emitting => {}: {}", username, content),
                    Err(err) => println!("error while emitting a message: {}", err),
                };
            }
        }
    });

    app.run(move |app_handler, event| {
        if let RunEvent::ExitRequested { .. } = event {
            app_handler
                .windows()
                .iter()
                .for_each(|(window_name, window)| {
                    if let Err(e) = window.close() {
                        eprintln!("failed to close window '{}': {:#?}", window_name, e);
                    }
                });

            app_handler.exit(0);
        }
    });
}
