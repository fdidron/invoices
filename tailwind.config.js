/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/markup/**/*.rs"],
  darkMode: "selector",
  theme: {
    extend: {
      colors: {
        sidebar: "#373B53",
        lila: "#7C5DFA",
        lilafade: "#9277FF",
        body: "#F8F8FB",
        darkbody: "#141625",
        typo: "#0C0E16",
        label: "#7E88C3", 
        labeld: "#DFE3FA",
        inputd: "#1E2139",
        borderd: "#252945",
      },
      fontFamily: {
        sans: ["League Spartan", "sans-serif"],
      },
    },
  },
  plugins: [],
};
