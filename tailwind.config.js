/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamaily: {},
      colors: {
        black: "#000",
        white: "#fff",
        accent: "#f56565",
      },
      userSelect: ["none"],
    },
  },
  plugins: [],
};
