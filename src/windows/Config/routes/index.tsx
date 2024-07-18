import { FileSliders, Settings2 } from 'lucide-react'
import Configuration from '../pages/Configuration'
import General from '../pages/General'

const configRoutes = [
  {
    key: 'general',
    icon: () => <Settings2 size={20} />,
    name: '通用设置',
    path: '/',
    element: <General />,
  },
  {
    key: 'configuration',
    icon: () => <FileSliders size={20} />,
    name: '配置管理',
    path: '/configuration',
    element: <Configuration />,
  },
]

export default configRoutes
