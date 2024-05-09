let app = window.app || {};

const DEFAULT_THEME = "light";

document.addEventListener("DOMContentLoaded", function () {
  try {
    let theme = localStorage.getItem("theme") || DEFAULT_THEME;
    if (theme === "dark") {
      document.documentElement.classList.add("dark");
    }
  } catch {
    console.info("Failed to load theme from localStorage");
  }
});

app.toggleTheme = function () {
  const theme = localStorage.getItem("theme") || DEFAULT_THEME;
  const newTheme = theme === "light" ? "dark" : "light";
  try {
    localStorage.setItem("theme", newTheme);
  } catch {
    console.info("Failed to save theme to localStorage");
  }
  if (newTheme === "dark") {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
};
