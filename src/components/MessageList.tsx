import { Component, createEffect, For } from "solid-js";
import { createStore } from "solid-js/store";
import { debounce } from "@solid-primitives/scheduled";
import { Event, listen } from '@tauri-apps/api/event';

interface Message {
	username: string;
	username_color: { r: number, g: number, b: number } | null;
	content: string;
}

const MessageList: Component = () => {
	const [state, setState] = createStore({
		messages: [] as Message[],
		autoScrollToEnd: true
	});

	let messagesEndDiv: HTMLDivElement | undefined;
	let rootDiv: HTMLDivElement | undefined;

	createEffect(() => {
		listen("on_message", (event: Event<Message>) => {
			setState('messages', (messages: Message[]) => [...messages, event.payload]);
			if (messagesEndDiv && state.autoScrollToEnd) {
				messagesEndDiv.scrollIntoView({ behavior: 'smooth' });
			}
		})
	});

	const onScroll = debounce(() => {
		if (rootDiv) {
			const { scrollTop, scrollHeight, clientHeight } = rootDiv;
			setState('autoScrollToEnd', () => scrollTop + clientHeight === scrollHeight);
		}
	}, 200);

	return (
		<div ref={rootDiv} onWheel={onScroll} class="h-full overflow-y-auto">
			<For each={state.messages}>
				{(message: Message) => (
					<div class="flex gap-2 text-left text-white">
						<div style={{
							color: `rgb(${message.username_color?.r} ${message.username_color?.g}  ${message.username_color?.b})`
						}}>
							{message.username}
							<span class="text-white">:</span>
						</div>
						<div>{message.content}</div>
					</div>
				)}
			</For>
			<div style={{ float: 'left', clear: 'both' }}
				ref={messagesEndDiv}>
			</div>
		</div>
	);
};

export default MessageList;
