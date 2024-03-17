/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./src/**/*.{html,rs}",
		"../yew_app_core/src/components/**/*.{html,rs}",
	],
	theme: {
		extend: {},
	},
	plugins: [],
};
