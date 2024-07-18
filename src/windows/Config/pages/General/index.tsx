import { Card, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Switch } from '@/components/ui/switch'
import { useConfig } from '@/hooks'
import { locale } from '@tauri-apps/api/os'
import { useEffect } from 'react'

export default function General() {
  const [autoStart, setAutoStart] = useConfig('auto_start', false)
  const [checkUpdate, setCheckUpdate] = useConfig('check_update', true)
  const [appLanguage, setAppLanguage] = useConfig('app_language', '')
  const [appTheme, setAppTheme] = useConfig('app_theme', 'system')
  const [translateTargetLanguage, setTranslateTargetLanguage] = useConfig('translate_target_language', '')

  useEffect(() => {
    if (appLanguage === '') {
      locale().then((lang) => {
        setAppLanguage(lang)
      })
    }

    if (translateTargetLanguage === '') {
      locale().then((lang) => {
        setTranslateTargetLanguage(lang)
      })
    }
  }, [appLanguage, setAppLanguage, translateTargetLanguage, setTranslateTargetLanguage])

  return (
    <ScrollArea className='px-4 w-full h-full overflow-x-hidden'>
      <Card className='my-4'>
        <CardContent className='flex flex-col gap-2 p-4'>
          <div className='flex justify-between items-center'>
            <Label htmlFor='auto-start'>开机时启动应用</Label>
            <Switch id='auto-start' checked={autoStart as boolean} />
          </div>
          <div className='flex justify-between items-center'>
            <Label htmlFor='check-update'>启动时检查更新</Label>
            <Switch id='check-update' checked={checkUpdate as boolean} />
          </div>
        </CardContent>
      </Card>
    </ScrollArea>
  )
}
