import { emit } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import type React from 'react'
import { useEffect, useState } from 'react'
import { info, warn } from 'tauri-plugin-log-api'

export default function Screenshot() {
  const [isSelecting, setIsSelecting] = useState(false)
  const [startPoint, setStartPoint] = useState({ x: 0, y: 0 })
  const [movePoint, setMovePoint] = useState({ x: 0, y: 0 })
  const [hasSelection, setHasSelection] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    if (e.buttons === 1) {
      setHasSelection(false)
      setIsSelecting(true)
      setStartPoint({ x: e.clientX, y: e.clientY })
      setMovePoint({ x: e.clientX, y: e.clientY })
    }
  }

  const handleMouseMove = (e: MouseEvent) => {
    if (isSelecting) {
      setMovePoint({ x: e.clientX, y: e.clientY })
    }
  }

  const handleMouseUp = async (e: MouseEvent) => {
    setIsSelecting(false)
    setHasSelection(true)

    const width = Math.abs(startPoint.x - e.clientX)
    const height = Math.abs(startPoint.y - e.clientY)
    await info(`width: ${width}, height: ${height}`)
    if (width > 0 && height > 0) {
      await emit('translate_area', {
        x: Math.min(startPoint.x, e.clientX),
        y: Math.min(startPoint.y, e.clientY),
        width: width,
        height: height,
      })
    } else {
      await warn('Screenshot area is too small')
      await appWindow.close()
    }
  }

  const handleKeyDown = async (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === 'Escape') {
      await appWindow.close()
    }
  }

  const getSelectionStyle = () => {
    return {
      top: Math.min(startPoint.y, movePoint.y),
      left: Math.min(startPoint.x, movePoint.x),
      width: Math.abs(startPoint.x - movePoint.x),
      height: Math.abs(startPoint.y - movePoint.y),
    }
  }

  // 防止 mouseup 事件丢失
  // biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
  useEffect(() => {
    // 设置 body, root 背景色为透明
    document.body.style.backgroundColor = 'transparent'
    const rootDOM = document.getElementById('root')
    if (rootDOM) {
      rootDOM.style.backgroundColor = 'transparent'
    }

    if (isSelecting) {
      document.addEventListener('mousemove', handleMouseMove)
      document.addEventListener('mouseup', handleMouseUp)
    } else {
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }
  }, [isSelecting])

  return (
    <div onKeyDown={handleKeyDown}>
      <div
        className='fixed top-0 left-0 bottom-0 right-0 cursor-crosshair select-none bg-transparent'
        onMouseDown={handleMouseDown}
      />
      <div
        className={`fixed bg-[#2080f020] border border-solid border-sky-500 ${!isSelecting && !hasSelection && 'hidden'}`}
        style={getSelectionStyle()}
      />
    </div>
  )
}
