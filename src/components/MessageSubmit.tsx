import { Component } from "solid-js";
import { invoke } from '@tauri-apps/api/tauri';

const MessageSubmit: Component = () => {
	let inputEl: HTMLInputElement | undefined;

	const sendMessage = () => {
		if (inputEl) {
			invoke('send_message', { message: inputEl.value });
		}
	};

	return (
		<div class="flex gap-2 text-white">
			<input
				class="grow px-2 rounded bg-slate-700 border-2 border-slate-700 focus:border-purple-600"
				ref={inputEl}
				placeholder="Send a message" />
			<button class="p-2 bg-purple-600 rounded" onClick={sendMessage}>Send</button>
		</div>
	);
};

export default MessageSubmit;

