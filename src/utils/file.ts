import { convertFileSrc, invoke } from '@tauri-apps/api/core'

interface FileInfoGet {
  name: string
  path: string
  size: number
  md5: string
  mtime: string
  birthtime: string
}

export interface FileInfoConvert extends FileInfoGet {
  src: string
}

export async function getFileInfo(dir: string, exts: Array<string>): Promise<Array<FileInfoConvert>> {
  const res = await invoke<Array<FileInfoGet>>('get_file_info', { path: dir, extensions: exts })

  return res.map(item => ({
    ...item,
    src: convertFileSrc(item.path),
  }))
}
