import daisyui from 'daisyui';
import type { Config } from 'tailwindcss';

const config: Config = {
	darkMode: ['selector', '[data-theme="dark"]'],
	content: ['./index.html', './src/**/*.{js,ts,jsx,tsx,vue}'],
	theme: {
		extend: {},
	},
	plugins: [daisyui],
	daisyui: {
		themes: ['light', 'dark'],
	},
};

export default config;
