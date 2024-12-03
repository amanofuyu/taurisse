<script setup lang="ts">
// onMounted(() => {
//   useEventListener('contextmenu', (e) => {
//     e.preventDefault()
//   })
// })
</script>

<template>
  <div class="w-full h-screen flex bg-base-100 text-base-content rounded-3xl overflow-hidden maximized:rounded-none">
    <Sidebar />
    <div class="flex-1">
      <div class="flex flex-col h-full">
        <header data-tauri-drag-region class="h-20 px-6 flex items-center justify-end bg-base-100">
          <Titlebar />
        </header>
        <main class="bg-base-300 flex-1 overflow-x-hidden overflow-y-auto rounded-tl-xl">
          <RouterView />
        </main>
      </div>
    </div>
  </div>
</template>
