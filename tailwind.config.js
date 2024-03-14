/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./yew_app_core/src/components/**/*.{html,rs}",
		"./storybook_yew_app/src/components/**/*.{html,rs}",
	],
	theme: {
		extend: {},
	},
	plugins: [],
};
