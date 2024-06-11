import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { useQueryClient } from '@tanstack/react-query'
import { ListTreeIcon } from 'lucide-react'
import { useEffect } from 'react'
import { events, commands } from '../../bindings'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { TooltipContent } from '../Tooltip'

export const GitCommandLog = () => {
  const queryClient = useQueryClient()

  useEffect(() => {
    const unlisten = events.gitCommandEvent.listen(() => {
      queryClient.invalidateQueries({ queryKey: ['state'] })
    })
    return () => {
      unlisten.then((f) => f())
    }
  }, [queryClient])

  const { data: state } = useCommandQuery({
    queryKey: ['state'],
    queryFn: commands.getStateData,
  })

  const latestCommand =
    state && state.git_command_log.length > 0
      ? `git ${state.git_command_log[state.git_command_log.length - 1].command}`
      : null

  return (
    <Tooltip>
      <TooltipTrigger
        type="button"
        className="px-2 bg-surface rounded-md flex-1 flex items-center gap-2 hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 min-w-0"
        disabled={latestCommand === null}
      >
        <ListTreeIcon className="h-4 w-4 shrink-0" />
        <span className="font-mono text-xs text-foreground/70 whitespace-nowrap overflow-hidden text-ellipsis mx-1">
          {latestCommand ?? 'No commands yet'}
        </span>
      </TooltipTrigger>
      <TooltipContent align="start">Git command log</TooltipContent>
    </Tooltip>
  )
}
