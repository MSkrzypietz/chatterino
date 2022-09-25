import type { Component } from 'solid-js';
import MessageList from './components/MessageList';
import styles from './App.module.css';

const App: Component = () => {
	return (
		<div class={styles.App}>
			<div class="min-h-screen p-4 mx-auto bg-slate-800">
				<MessageList />
			</div>
		</div>
	);
};

export default App;
