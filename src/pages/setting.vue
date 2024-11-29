<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
</script>

<template>
  <div>
    <button class="btn" @click="open()">
      open
    </button>
  </div>
</template>

<style scoped></style>
