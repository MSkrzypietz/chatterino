use yew::{html, Component, Context, Html, Properties};

use crate::components::message::Message;

#[derive(Properties, Clone, PartialEq)]
pub struct MessageListProps {
    #[prop_or_default]
    pub messages: Vec<Message>,
}

pub struct MessageList;

impl Component for MessageList {
    type Message = ();
    type Properties = MessageListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {
                ctx.props().messages.iter().map(|msg| {
                    html! {
                        <div>
                            {format!("{}: {}", msg.username, msg.content)}
                        </div>
                    }
                }).collect::<Html>()
            }
        }
    }
}
