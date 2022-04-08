import { Command } from '@tauri-apps/api/shell'

export const isVideo = async (path: string): Promise<boolean> => {
  const { code } = await Command.sidecar('bin/ffprobe', path).execute()
  return code === 0
}
