/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.tsx'],
  theme: {
    fontFamily: {
      sans: 'var(--font-family-sans)',
    },
    extend: {
      colors: {
        background: 'rgb(var(--color-background))',
        foreground: 'rgb(var(--color-foreground))',
        surface: 'rgb(var(--color-surface))',

        info: 'rgb(var(--color-info))',
        success: 'rgb(var(--color-success))',
        warn: 'rgb(var(--color-warn))',
        error: 'rgb(var(--color-error))',
      },
    },
  },
  plugins: [require('tailwindcss-animate')],
}
