export const THEMES = ["default", "ayu-dark", "green", "purple"] as const;

export type Theme = (typeof THEMES)[number];

const STORAGE_KEY = "theme";

const useTheme = () => {
  const getTheme: () => Theme = () => {
    const theme = localStorage.getItem(STORAGE_KEY);
    if (THEMES.includes(theme as Theme)) {
      return theme as Theme;
    }
    return "default"; // Default theme
  };

  const loadTheme = () => {
    const theme = getTheme();
    saveTheme(theme);
    setTheme(theme);
  };

  const saveTheme = (theme: Theme) => {
    localStorage.setItem(STORAGE_KEY, theme);
  };

  const setTheme = (theme: Theme) => {
    document.body.classList.remove(...THEMES);
    document.body.classList.add(theme);
    saveTheme(theme);
  };

  const toggleTheme = () => {
    const currentThemeIndex = THEMES.indexOf(getTheme());
    const nextThemeIndex = (currentThemeIndex + 1) % THEMES.length;
    const newTheme = THEMES[nextThemeIndex];
    setTheme(newTheme);
  };

  return {
    getTheme,
    loadTheme,
    saveTheme,
    toggleTheme,
    setTheme,
  };
};

export default useTheme;