/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,ts,tsx,js,jsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          "Segoe UI Variable",
          "Segoe UI",
          "Inter",
          "system-ui",
          "-apple-system",
          "Roboto",
          "Helvetica Neue",
          "sans-serif",
        ],
      },
      colors: {
        // Fluent-inspired palette
        accent: {
          DEFAULT: "#0078d4",
          hover: "#106ebe",
          pressed: "#005a9e",
        },
        surface: {
          DEFAULT: "#202020",
          card: "#2b2b2b",
          stroke: "#3a3a3a",
        },
      },
      boxShadow: {
        flyout: "0 8px 16px rgba(0,0,0,0.18), 0 0 1px rgba(0,0,0,0.4)",
      },
      borderRadius: {
        fluent: "8px",
      },
    },
  },
  plugins: [],
};
