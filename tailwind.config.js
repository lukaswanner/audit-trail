/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,svelte,ts}"],
	plugins: [require("daisyui")],
	daisyui: {
		themes: ["dark", "light"]
	}
};
