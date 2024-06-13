import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { HistoryIcon } from 'lucide-react'
import { TooltipContent } from '../UI/Tooltip'

export const History = () => {
  return (
    <Tooltip>
      <TooltipTrigger
        type="button"
        className="px-2 bg-surface rounded-md flex items-center gap-2 hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 min-w-0"
      >
        <HistoryIcon className="h-4 w-4 shrink-0" />
        <span className="text-sm text-foreground/70 whitespace-nowrap overflow-hidden text-ellipsis mx-1">Commit</span>
      </TooltipTrigger>
      <TooltipContent>History</TooltipContent>
    </Tooltip>
  )
}
