import type { Component } from 'solid-js';
import styles from './App.module.css';
import Chat from './components/Chat';

const App: Component = () => {
	return (
		<div class={styles.App}>
			<div class="h-screen p-4 mx-auto bg-slate-800">
				<Chat />
			</div>
		</div>
	);
};

export default App;
