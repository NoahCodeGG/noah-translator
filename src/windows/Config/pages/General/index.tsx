import { Card, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Switch } from '@/components/ui/switch'

export default function General() {
  return (
    <ScrollArea className='px-4 w-full h-full overflow-x-hidden'>
      <Card className='my-4'>
        <CardContent className='flex flex-col gap-2 p-4'>
          <div className='flex justify-between items-center'>
            <Label htmlFor='auto-start'>开机时启动应用</Label>
            <Switch id='auto-start' />
          </div>
          <div className='flex justify-between items-center'>
            <Label htmlFor='check-update'>启动时检查更新</Label>
            <Switch id='check-update' />
          </div>
        </CardContent>
      </Card>
    </ScrollArea>
  )
}
