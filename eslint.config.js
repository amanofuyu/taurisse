import antfu from '@antfu/eslint-config'

export default antfu({
  rules: {
    'no-console': 'off',
  },
  ignores: ['**/node_modules/**', '**/dist/**', '**/build/**', 'src-tauri/gen/**', 'src-tauri/target/**', 'src-tauri/src/**', 'Cargo.lock'],
})
