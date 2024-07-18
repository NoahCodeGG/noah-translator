import { ScrollArea, ScrollBar } from '@/components/ui/scroll-area'
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
    <ScrollArea className='w-fit h-full overflow-x-hidden px-3' data-tauri-drag-region>
      {text}
      <ScrollBar orientation='vertical' />
    </ScrollArea>
  )
}
