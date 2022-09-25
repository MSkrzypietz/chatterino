import { Component, createEffect, createRenderEffect, For, Ref } from "solid-js";
import { createStore } from "solid-js/store";
import { Event, listen } from '@tauri-apps/api/event';

interface Message {
	username: string;
	content: string;
}

const MessageList: Component = () => {
	const [state, setState] = createStore({
		messages: [] as Message[],
		scrollTop: 0,
	});

	let messagesEnd: HTMLDivElement | undefined;

	createEffect(() => {
		listen("on_message", (event: Event<Message>) => {
			setState('messages', (messages: Message[]) => [...messages, event.payload]);
			messagesEnd?.scrollIntoView({ behavior: 'smooth' });
		})
	});

	return (
		<div>
			<For each={state.messages}>
				{(message: Message) => (
					<div class="flex gap-2 text-left text-white">
						<div>{message.username}:</div>
						<div>{message.content}</div>
					</div>
				)}
			</For>
			<div style={{ float: 'left', clear: 'both' }}
				ref={messagesEnd}>
			</div>
		</div>
	);
};

export default MessageList;
