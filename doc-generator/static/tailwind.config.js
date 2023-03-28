/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.html"],
  theme: {
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      white: "#ffffff",

      "js-color": "var(--js-color)",
      "dark-color": "var(--dark-color)",
      "light-color": "var(--light-color)",
      "error-color": "var(--error-color)",
      "error-text-color": "var(--error-text-color)",
      "error-bg-color": "var(--error-bg-color)",
      "c1": "var(--c1)",
      "c2": "var(--c2)",
      "c3": "var(--c3)",
      "c4": "var(--c4)",
      "c5": "var(--c5)",
      "c3-transparent": "var(--c3-transparent)",
      "main-color": "var(--color)",
      "code-bg-color": "var(--code-bg-color)",
      "code-color": "var(--code-color)",
      "border-color": "var(--border-color)",

      "c1-primary": "var(--c1-primary)",

      "c2-primary": "var(--c2-primary)",
      "c2-on-primary": "var(--c2-on-primary)",
      "c2-primary-container": "var(--c2-primary-container)",
      "c2-on-primary-container": "var(--c2-on-primary-container)",

      "c3-primary": "var(--c3-primary)",
      "c3-on-primary": "var(--c3-on-primary)",
      "c3-primary-container": "var(--c3-primary-container)",
      "c3-on-primary-container": "var(--c3-on-primary-container)",
      "c4-primary": "var(--c4-primary)",
      "c5-primary": "var(--c5-primary)",
      "c5-on-primary": "var(--c5-on-primary)",
      "c5-primary-container": "var(--c5-primary-container)",
      "c5-on-primary-container": "var(--c5-on-primary-container)",
      "bg-color": "var(--bg-color)",
    },
    borderWidth: {
      '1': '1px'
    },
    extend: {},
  },
  plugins: [],
}
