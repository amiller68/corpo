/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        typewriter: {
          'to': { left: '100%' }
        },
       blink: {
          '0%': {
            opacity: '0',
          },
          '50%': {
            opacity: '1',
          },
        },
      },
      animation: {
        typewriter: 'typewriter 2s steps(11) forwards', // Adjust the steps based on the text length
        caret: 'typewriter 2s steps(11) forwards, blink 1s steps(2) infinite 2s',
      },
      colors: {
        black: "#000",
        white: "#fff",
        accent: "#f56565",
      },
    },
  },
  plugins: []
};
