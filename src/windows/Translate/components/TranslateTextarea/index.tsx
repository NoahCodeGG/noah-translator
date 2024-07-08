import { type Event, listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

export default function TranslateTextarea() {
  const [text, setText] = useState<string>()

  useEffect(() => {
    const unlisten = listen('realtime_translation', (e: Event<string>) => {
      setText(e.payload)

      return () => {
        unlisten.then((f) => {
          f()
        })
      }
    })
  }, [])

  return (
    <div className='w-fit h-full select-none overflow-y-scroll overflow-x-hidden px-3' data-tauri-drag-region>
      网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐网易云音乐{text}
    </div>
  )
}
