// Theme management: toggles between `light` and `dark` by setting a class on
// the root <html> element. Persists the user's choice in localStorage and
// falls back to the OS-level `prefers-color-scheme` preference on first run.
import { onMounted, ref, watch } from "vue";

export type Theme = "light" | "dark";

const STORAGE_KEY = "clt.theme";

function detectInitial(): Theme {
  if (typeof window === "undefined") return "dark";
  const saved = window.localStorage.getItem(STORAGE_KEY) as Theme | null;
  if (saved === "light" || saved === "dark") return saved;
  const prefersLight =
    window.matchMedia && window.matchMedia("(prefers-color-scheme: light)").matches;
  return prefersLight ? "light" : "dark";
}

function apply(theme: Theme) {
  const root = document.documentElement;
  root.classList.toggle("dark", theme === "dark");
  root.classList.toggle("light", theme === "light");
  root.style.colorScheme = theme;
}

export function useTheme() {
  const theme = ref<Theme>(detectInitial());

  onMounted(() => {
    apply(theme.value);
  });

  watch(theme, (next) => {
    apply(next);
    try {
      window.localStorage.setItem(STORAGE_KEY, next);
    } catch {
      /* storage may be unavailable (private mode); ignore */
    }
  });

  function toggle() {
    theme.value = theme.value === "dark" ? "light" : "dark";
  }

  return { theme, toggle };
}
