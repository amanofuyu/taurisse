<script setup lang="ts"></script>

<template>
  <aside data-tauri-drag-region class="w-24 py-6 bg-base-100 flex flex-col items-center justify-end gap-4">
    <div class="flex flex-col items-center gap-3">
      <button class="btn btn-outline i-lucide:house" @click="$router.push('/')" />
      <button class="btn btn-outline i-lucide:settings" @click="$router.push('/setting')" />
    </div>
  </aside>
</template>

<style scoped></style>
