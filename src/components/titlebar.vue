<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { clsx } from 'clsx'

const appWindow = getCurrentWebviewWindow()

const isMaximized = ref(false)

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
})

async function toggleMaximized() {
  await appWindow.toggleMaximize()
  isMaximized.value = !isMaximized.value
}
</script>

<template>
  <div class="flex items-center gap-3">
    <button class="btn btn-outline i-lucide:minus" @click="appWindow.minimize()" />
    <button :class="clsx('btn btn-outline', isMaximized ? 'i-cus:stack' : 'i-cus:square')" @click="toggleMaximized()" />
    <button class="btn btn-outline i-lucide:x" @click="appWindow.close()" />
  </div>
</template>

<style scoped></style>
