export const debounce = (fn: (...args: unknown[]) => void, delay = 500) => {
  let timer: NodeJS.Timeout | null = null
  return (...args: unknown[]) => {
    timer && clearTimeout(timer)
    timer = setTimeout(() => fn(args), delay)
  }
}
