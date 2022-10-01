import { Component } from "solid-js";
import MessageList from "./MessageList";
import MessageSubmit from "./MessageSubmit";

const Chat: Component = () => {
	return (
		<div class="h-full flex flex-col gap-1">
			<MessageList />
			<MessageSubmit />
		</div>
	);
};

export default Chat;
