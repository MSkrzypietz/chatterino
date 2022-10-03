import { Component } from "solid-js";

const Navbar: Component = () => {
		const parameters = `response_type=code&client_id=rdhdtr6t6lq7qs4qhd1yzqcu6gipg8&scope=chat%3Aedit&redirect_uri=http://localhost:1420`;
	return (
		<div class="flex text-white p-2 bg-slate-900 border-b-2 border-black">
			<div class="grow text-left m-auto">Channel: zackrawrr</div>
			<a href={`https://id.twitch.tv/oauth2/authorize?${parameters}`}
				class="p-2 bg-purple-600 rounded">Login</a>
		</div>
	);
};

export default Navbar;
