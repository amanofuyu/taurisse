<script setup lang="ts">
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

interface FileInfoGet {
  name: string
  path: string
  size: number
  md5: string
  mtime: string
  birthtime: string
}

interface FileInfoConvert extends FileInfoGet {
  src: string
}

async function chooseDirs() {
  const dirs = await open({
    directory: true,
    multiple: true,
  })

  if (!dirs) {
    return
  }

  const res = (await Promise.all(dirs.map(dir => invoke<Array<FileInfoGet>>('get_file_info', { path: dir, extensions: ['svg', 'png'] })))).flat().map(item => ({
    ...item,
    src: convertFileSrc(item.path),
  })) satisfies Array<FileInfoConvert>

  console.log(res)
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
