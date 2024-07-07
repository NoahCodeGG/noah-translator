import { type Event, listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

export default function Translate() {
  const [text, setText] = useState<string>()

  useEffect(() => {
    // 设置 body, root 背景色为透明
    // document.body.style.backgroundColor = 'transparent'
    // const rootDOM = document.getElementById('root')
    // if (rootDOM) {
    //   rootDOM.style.backgroundColor = 'transparent'
    // }
    
    const unlisten = listen('realtime_translation', (e: Event<string>) => {
      setText(e.payload)

      return () => {
        unlisten.then((f) => {
          f()
        })
      }
    })
  }, [])

  return <div>{text}</div>
}
