import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { initEnv } from './utils/env'
import { initStore } from './utils/store'

initStore().then(async () => {
  await initEnv()
  const rootElement = document.getElementById('root')
  const root = ReactDOM.createRoot(rootElement as HTMLElement)
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  )
})
