import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { HistoryIcon } from 'lucide-react'
import { TooltipContent } from '../UI/Tooltip'

export const History = () => {
  return (
    <button
      type="button"
      className="bg-surface rounded-md flex items-center hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 min-w-0"
      disabled
    >
      <Tooltip>
        <TooltipTrigger asChild>
          <HistoryIcon className="h-8 w-8 p-2 shrink-0" />
        </TooltipTrigger>
        <TooltipContent>History</TooltipContent>
      </Tooltip>
      <span className="text-sm text-foreground/70 whitespace-nowrap overflow-hidden text-ellipsis ml-1 mr-3">
        Not implemented
      </span>
    </button>
  )
}
