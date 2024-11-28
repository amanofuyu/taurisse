import type { Config } from 'tailwindcss'
import daisyui from 'daisyui'
import plugin from 'tailwindcss/plugin'

const config: Config = {
  darkMode: ['selector', '[data-theme="dark"]'],
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx,vue}'],
  theme: {
    extend: {},
  },
  plugins: [daisyui, plugin(({ addVariant }) => {
    addVariant('maximized', '&:where([data-maximized="true"], [data-maximized="true"] *)')
  })],
  daisyui: {
    themes: ['light', 'dark'],
    base: false,
  },
}

export default config
