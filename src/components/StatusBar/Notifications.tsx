import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { BellIcon } from 'lucide-react'
import { TooltipContent } from '../UI/Tooltip'

export const Notifications = () => {
  return (
    <button
      type="button"
      className="w-8 bg-surface rounded-md flex items-center justify-center hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20"
    >
      <Tooltip>
        <TooltipTrigger asChild>
          <BellIcon className="h-8 w-8 p-2" />
        </TooltipTrigger>
        <TooltipContent>Notifications</TooltipContent>
      </Tooltip>
    </button>
  )
}
