import { ref, watch } from "vue";

const THEME_KEY = "rust-learning-theme";

type Theme = "light" | "dark";

function getPreferredTheme(): Theme {
  const saved = localStorage.getItem(THEME_KEY);
  if (saved === "light" || saved === "dark") {
    return saved;
  }
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

export function useTheme() {
  const theme = ref<Theme>(getPreferredTheme());

  function applyTheme(value: Theme) {
    document.documentElement.setAttribute("data-theme", value);
  }

  function toggleTheme() {
    theme.value = theme.value === "light" ? "dark" : "light";
  }

  watch(
    theme,
    (value) => {
      localStorage.setItem(THEME_KEY, value);
      applyTheme(value);
    },
    { immediate: true }
  );

  return { theme, toggleTheme };
}
