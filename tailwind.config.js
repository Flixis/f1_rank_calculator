/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        primary: '#007bff',
        secondary: '#6c757d',
        neutralLight: '#f8f9fa',
        neutralDark: '#212529',
        accent1: '#ffc107',
        accent2: '#f56565',
        accent3: '#ed8936',
      },
    },
  },
  variants: {},
  plugins: [],
}
