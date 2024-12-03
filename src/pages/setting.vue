<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { readDir, readFile, stat } from '@tauri-apps/plugin-fs'

async function chooseDirs() {
  const dirs = await open({
    directory: true,
    multiple: true,
  })

  if (!dirs) {
    return
  }

  const dirEntries = (await Promise.all(
    dirs.map(dir => readDir(dir)),
  )).flatMap((entries, i) => entries.map(entry => ({ ...entry, dir: dirs[i] }))).filter(entry => entry.isFile)

  console.log(dirEntries)

  const filePaths = dirEntries.map(entry => convertFileSrc(`${entry.dir}/${entry.name}`))

  console.log(filePaths)

  const fileStats = await Promise.all(
    dirEntries.map(entry => (stat(`${entry.dir}/${entry.name}`))),
  )

  console.log(fileStats)

  const files = await Promise.all(
    dirEntries.map(entry => readFile(`${entry.dir}/${entry.name}`)),
  )

  console.log(files)
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
