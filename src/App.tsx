import type { Component } from 'solid-js';
import styles from './App.module.css';
import Chat from './components/Chat';
import Navbar from './components/Navbar';

const App: Component = () => {
	return (
		<div class={styles.App}>
			<div class="h-screen flex flex-col mx-auto bg-slate-800">
				<Navbar />
				<Chat />
			</div>
		</div>
	);
};

export default App;
