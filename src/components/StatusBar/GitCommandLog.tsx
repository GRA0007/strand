import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { ListTreeIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import { events, type GitCommandLog as GitCommandLogType } from '../../bindings'
import { TooltipContent } from '../Tooltip'

export const GitCommandLog = () => {
  const [latestCommand, setLatestCommand] = useState<GitCommandLogType>()
  useEffect(() => {
    const unlisten = events.gitCommandEvent.listen(({ payload }) => {
      setLatestCommand(payload)
    })
    return () => {
      unlisten.then((f) => f())
    }
  }, [])

  return (
    <Tooltip>
      <TooltipTrigger
        type="button"
        className="px-2 bg-surface rounded-md flex-1 flex items-center gap-2 hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 min-w-0"
        disabled={!latestCommand}
      >
        <ListTreeIcon className="h-4 w-4 shrink-0" />
        <span className="font-mono text-xs text-foreground/70 whitespace-nowrap overflow-hidden text-ellipsis mx-1">
          {latestCommand?.command ?? 'No commands yet'}
        </span>
      </TooltipTrigger>
      <TooltipContent align="start">Git command log</TooltipContent>
    </Tooltip>
  )
}
