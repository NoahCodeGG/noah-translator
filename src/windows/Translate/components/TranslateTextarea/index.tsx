import { type Event, listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

interface Props {
  isLock: boolean
}
export default function TranslateTextarea({ isLock }: Props) {
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

  return isLock ? (
    <div className='w-fit h-full select-none overflow-y-scroll overflow-x-hidden px-3'>{text}</div>
  ) : (
    <div className='w-fit h-full select-none overflow-y-scroll overflow-x-hidden px-3' data-tauri-drag-region>
      {text}
    </div>
  )
}
