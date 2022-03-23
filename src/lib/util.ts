export const fileSize = (size: number): string => {
  const unitsSi = ['B', 'KB', 'MB', 'GB', 'TB']
  const unitsIec = ['B', 'KiB', 'MiB', 'GiB', 'TiB']

  let i = 0
  let j = 0
  let sizeSi = size
  let sizeIec = size

  for (; sizeSi >= 1000; i++) {
    sizeSi /= 1000
  }
  for (; sizeIec >= 1024; j++) {
    sizeIec /= 1024
  }

  return `${sizeSi.toFixed(1)} ${unitsSi[i] as string} (${sizeIec.toFixed(1)} ${
    unitsIec[j] as string
  })`
}
