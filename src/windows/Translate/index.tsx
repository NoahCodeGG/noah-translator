import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { cn } from '@/utils/tailwind'
import { emit } from '@tauri-apps/api/event'
import { Lock, LockOpen, X } from 'lucide-react'
import { useEffect, useState } from 'react'
import TranslateTextarea from './components/TranslateTextarea'

export default function Translate() {
  const [isLock, setIsLock] = useState<boolean>(false)
  const [isEnter, setIsEnter] = useState<boolean>(true)

  const handlerMouseEnter = () => setIsEnter(true)
  const handlerMouseLeave = () => setIsEnter(false)
  const handlerUnlockClick = () => setIsLock(false)

  const operations = [
    {
      key: 'lock',
      name: '锁定',
      icon: () => <Lock size={20} />,
      action: () => {
        setIsLock(true)
      },
    },
    {
      key: 'close',
      name: '关闭',
      icon: () => <X size={20} />,
      action: async () => {
        await emit('close')
      },
    },
  ]

  useEffect(() => {
    // 设置 body, root 背景色为透明
    document.body.style.backgroundColor = 'transparent'
    const rootDOM = document.getElementById('root')
    if (rootDOM) {
      rootDOM.style.backgroundColor = 'transparent'
    }
  }, [])

  return isLock ? (
    <div
      className={cn(
        'w-fit h-fit rounded-md text-white cursor-default overflow-hidden',
        isEnter ? 'bg-black opacity-70 w-full h-full' : '',
      )}
      onMouseEnter={handlerMouseEnter}
      onMouseLeave={handlerMouseLeave}
    >
      <div className='w-fit h-8 flex items-center m-auto gap-2'>
        {isEnter && (
          <Tooltip>
            <TooltipTrigger asChild>
              <div className='cursor-pointer' onClick={handlerUnlockClick}>
                <LockOpen />
              </div>
            </TooltipTrigger>
            <TooltipContent>解锁</TooltipContent>
          </Tooltip>
        )}
      </div>
      <TranslateTextarea isLock={isLock} />
    </div>
  ) : (
    <div
      className={[
        'w-fit h-fit rounded-md text-white cursor-move overflow-hidden',
        isEnter ? 'bg-black opacity-70 w-full h-full' : '',
      ].join(' ')}
      onMouseEnter={handlerMouseEnter}
      onMouseLeave={handlerMouseLeave}
      data-tauri-drag-region
    >
      <div className='w-fit h-8 flex items-center m-auto gap-2'>
        {isEnter &&
          operations.map((operation) => (
            <Tooltip key={operation.key}>
              <TooltipTrigger asChild>
                <div className='cursor-pointer' onClick={operation.action}>
                  <operation.icon />
                </div>
              </TooltipTrigger>
              <TooltipContent>{operation.name}</TooltipContent>
            </Tooltip>
          ))}
      </div>
      <TranslateTextarea isLock={isLock} />
    </div>
  )
}
