import { Component } from "solid-js";
import MessageList from "./MessageList";
import MessageSubmit from "./MessageSubmit";

const Chat: Component = () => {
	return (
		<div class="flex flex-col flex-nowrap grow p-2 gap-1 overflow-y-auto">
			<MessageList />
			<MessageSubmit />
		</div>
	);
};

export default Chat;

