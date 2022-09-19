const invoke = window.__TAURI__.invoke

export const invokeHello = async (name) => {
	return await invoke("hello", { name: name });
}
