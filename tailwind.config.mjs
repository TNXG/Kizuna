/** @type {import('tailwindcss').Config} */
import daisyui from "daisyui";

const twConfig = {
	content: [
		"src/**/*.{js,vue,ts,html}",
	],
	theme: {
		extend: {},
	},
	plugins: [
		daisyui,
	],
};

export default twConfig;
