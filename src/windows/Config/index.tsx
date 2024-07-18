import { DragRegion } from '@/components/drag-region'
import { useLocation, useRoutes } from 'react-router-dom'
import ConfigSideBar from './components/SideBar'
import configRoutes from './routes'

export default function Config() {
  const page = useRoutes(configRoutes)
  const location = useLocation()

  return (
    <div className='w-full h-full rounded-md overflow-hidden border-2 border-zinc-200 dark:border-zinc-800'>
      <DragRegion />
      <div className='flex w-full h-full'>
        <ConfigSideBar currentPath={location.pathname} />
        <div className='flex-1 overflow-hidden'>
          <div className='py-2'>
            <h1 className='p-4 text-xl font-bold text-zinc-900 dark:text-zinc-50 border-b border-zinc-200 dark:border-zinc-800'>
              {configRoutes.map((route) => (route.path === location.pathname ? route.name : ''))}
            </h1>
          </div>
          {page}
        </div>
      </div>
    </div>
  )
}
