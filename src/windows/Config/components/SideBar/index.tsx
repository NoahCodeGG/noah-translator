import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu.tsx'
import { cn } from '@/utils/tailwind'
import configRoutes from '@/windows/Config/routes'

interface Props {
  currentPath: string
}

export default function ConfigSideBar({ currentPath }: Props) {
  return (
    <div className='h-full p-[12px] overflow-y-auto flex flex-col justify-center border-r border-zinc-200 dark:border-zinc-800'>
      <div className='h-[120px] flex justify-center items-center'>
        <img alt='pot logo' src='icon.svg' className='h-[60px] w-[60px]' draggable={false} />
      </div>

      <NavigationMenu className='flex-1 w-full h-full'>
        <NavigationMenuList className='flex flex-col gap-2'>
          {configRoutes.map((route) => (
            <NavigationMenuItem className='w-full' key={route.key}>
              <NavigationMenuLink
                className={cn(navigationMenuTriggerStyle(), 'w-full flex justify-around gap-2')}
                active={currentPath === route.path}
                href={route.path}
              >
                <route.icon />
                {route.name}
              </NavigationMenuLink>
            </NavigationMenuItem>
          ))}
        </NavigationMenuList>
      </NavigationMenu>
    </div>
  )
}
