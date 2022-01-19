module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    fontFamily: {
      sans: ["Hack"],
    },
    colors: {
       "black": "#1E222A",
       "black-light": "#2B2F37",
       "white": "#ABB2BF",
       "white-light": "#C8CCD4",
       "red": "#BE5046",
       "red-light": "#E06C75",
       "orange": "#D19A66",
       "yellow": "#E5C07B",
       "green": "#98C379",
       "cyan": "#56B6C2",
       "blue": "#61AFEF",
       "magenta": "#C678DD",
    },
  },
  variants: {},
  plugins: [
    function ({ addComponents }) {
      addComponents({
        ".container": {
          maxWidth: "100%",
          "@screen sm": {
            maxWidth: "640px",
          },
          "@screen md": {
            maxWidth: "768px",
          },
          "@screen lg": {
            maxWidth: "1000px",
          },
          "@screen xl": {
            maxWidth: "1000px",
          },
        }
      })
    }
  ],
};
