use serde::Serialize;
use tauri::{Manager, RunEvent};
use tokio::sync::mpsc;
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

    let (tx, mut rx) = mpsc::channel(32);

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(message) = message {
                tx.send(Payload {
                    username: message.sender.name,
                    content: message.message_text,
                })
                .await
                .unwrap();
            }
        }
    });

    let channel = std::env::var("CHANNEL").unwrap();
    client.join(channel).unwrap();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let app_handle = app.app_handle();
    tokio::spawn(async move {
        while let Some(payload) = rx.recv().await {
            println!("Emitting => {}: {}", payload.username, payload.content);
            app_handle.emit_all("on_message", payload).unwrap();
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
