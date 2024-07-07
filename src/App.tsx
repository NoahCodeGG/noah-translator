import { BrowserRouter } from 'react-router-dom'

import './App.css'
import Config from '@/windows/Config'
import Screenshot from '@/windows/Screenshot'
import Translate from '@/windows/Translate'
import { appWindow } from '@tauri-apps/api/window'

const windowMap: Record<string, JSX.Element> = {
  screenshot: <Screenshot />,
  config: <Config />,
  translate: <Translate />,
}

function App() {
  return <BrowserRouter>{windowMap[appWindow.label]}</BrowserRouter>
}

export default App
