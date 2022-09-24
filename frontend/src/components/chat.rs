use crate::components::message_list::MessageList;
use yew::{html, Component, Context, Html, Properties};

use super::message::Message;

pub struct Chat;

impl Component for Chat {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let messages: Vec<Message> = vec![
            Message {
                id: 0,
                username: "me".to_string(),
                content: "test message 1".to_string(),
            },
            Message {
                id: 1,
                username: "you".to_string(),
                content: "test message 2".to_string(),
            },
        ];

        html! {
            <div>
                <MessageList messages={messages} />
            </div>
        }
    }
}
