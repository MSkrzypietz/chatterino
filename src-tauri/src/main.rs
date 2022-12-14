use serde::Serialize;
use std::sync::Arc;
use tauri::{Manager, RunEvent};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{RGBColor, ServerMessage};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

#[derive(Debug, Clone, Serialize)]
struct Payload {
    username: String,
    username_color: Option<RGBColor>,
    content: String,
}

struct AppState {
    irc_client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    channel: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = ClientConfig::default();
    let (mut incoming_messages, irc_client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let channel = std::env::var("CHANNEL").unwrap();
    irc_client.join(channel.clone()).unwrap();

    let app = tauri::Builder::default()
        .manage(AppState {
            irc_client,
            channel,
        })
        .invoke_handler(tauri::generate_handler![send_message])
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
                    username_color: message.name_color,
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

#[tauri::command]
async fn send_message(state: tauri::State<'_, AppState>, message: String) -> Result<(), ()> {
    match state
        .irc_client
        .say(state.channel.clone(), message.clone())
        .await
    {
        Ok(_) => println!("Sending: {}", message),
        Err(e) => println!("Error sending message: {}", e),
    }

    Ok(())
}
