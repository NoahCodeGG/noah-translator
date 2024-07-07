import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu.tsx'
import configRoutes from '@/windows/Config/routes'
import { Link } from '@radix-ui/react-navigation-menu'

export default function SideBar() {
  return (
    <div className='mx-[12px] overflow-y-auto'>
      <NavigationMenu>
        {configRoutes.map((route) => (
          <NavigationMenuItem key={route.path}>
            <Link href={route.path}>
              <NavigationMenuLink className={navigationMenuTriggerStyle()}>{route.name}</NavigationMenuLink>
            </Link>
          </NavigationMenuItem>
        ))}
      </NavigationMenu>
    </div>
  )
}
