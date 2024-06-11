import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { BellIcon } from 'lucide-react'
import { TooltipContent } from '../Tooltip'

export const Notifications = () => {
  return (
    <Tooltip>
      <TooltipTrigger
        type="button"
        className="w-8 bg-surface rounded-md flex items-center justify-center hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20"
      >
        <BellIcon className="h-4 w-4" />
      </TooltipTrigger>
      <TooltipContent>Notifications</TooltipContent>
    </Tooltip>
  )
}
