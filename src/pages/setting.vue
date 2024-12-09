<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'

async function chooseDirs() {
  const dirs = await open({
    directory: true,
    multiple: true,
  })

  if (!dirs) {
    return
  }

  console.log(dirs)
}
</script>

<template>
  <div>
    <button class="btn" @click="chooseDirs()">
      open
    </button>
  </div>
</template>

<style scoped></style>
