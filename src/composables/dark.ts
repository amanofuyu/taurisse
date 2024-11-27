export const isDark = useDark({
  attribute: 'data-theme',
  valueLight: 'light',
  valueDark: 'dark',
  storageKey: 'color-schema',
})
export const toggleDark = useToggle(isDark)
