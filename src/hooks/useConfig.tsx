import { emit, listen } from '@tauri-apps/api/event'
import { useCallback, useEffect } from 'react'
import { debounce } from '../utils'
import { store } from '../utils/store'
import { useGetState } from './useGetState'

export const useConfig = (
  key: string,
  defaultValue: string | number | boolean | null,
  options: { sync?: boolean } = {},
) => {
  const [property, setPropertyState, getProperty] = useGetState(null)
  const { sync = true } = options

  // 同步到Store (State -> Store)
  const syncToStore = useCallback(
    debounce((v) => {
      store.set(key, v)
      store.save()
      const eventKey = key.replaceAll('.', '_').replaceAll('@', ':')
      emit(`${eventKey}_changed`, v)
    }),
    [],
  )

  // 同步到State (Store -> State)
  const syncToState = useCallback(
    (v: string | number | boolean | null) => {
      if (v !== null) {
        setPropertyState(v)
      } else {
        store.get(key).then((v) => {
          if (v === null) {
            setPropertyState(defaultValue)
            store.set(key, defaultValue)
            store.save()
          } else {
            setPropertyState(v)
          }
        })
      }
    },
    [setPropertyState, key, defaultValue],
  )

  const setProperty = useCallback(
    (v: string | number | boolean | null, forceSync = false) => {
      setPropertyState(v)
      const isSync = forceSync || sync
      isSync && syncToStore(v)
    },
    [setPropertyState, syncToStore, sync],
  )

  // 初始化
  useEffect(() => {
    syncToState(null)
    const eventKey = key.replaceAll('.', '_').replaceAll('@', ':')
    const unlisten = listen(`${eventKey}_changed`, (e) => {
      return syncToState(e.payload as string | number | boolean | null)
    })

    return () => {
      unlisten.then((f) => {
        f()
      })
    }
  }, [key, syncToState])

  return [property, setProperty, getProperty] as const
}

export const deleteKey = async (key: string) => {
  if (await store.has(key)) {
    store.delete(key)
    store.save()
  }
}
