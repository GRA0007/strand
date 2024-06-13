import { Popover, PopoverTrigger } from '@radix-ui/react-popover'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { useQueryClient } from '@tanstack/react-query'
import { ListTreeIcon, PencilIcon, SearchIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import { events, type GitCommandLog as GitCommandLogType, commands } from '../../bindings'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { PopoverContent } from '../UI/Popover'
import { TooltipContent } from '../UI/Tooltip'

export const GitCommandLog = () => {
  const queryClient = useQueryClient()
  const [isOpen, setIsOpen] = useState(false)

  const [latestCommand, setLatestCommand] = useState<GitCommandLogType>()
  useEffect(() => {
    const unlisten = events.gitCommandEvent.listen(({ payload }) => {
      setLatestCommand(payload)
    })
    return () => {
      unlisten.then((f) => f())
    }
  }, [])

  const { data: log } = useCommandQuery({
    queryKey: ['gitCommandLog'],
    queryFn: commands.getGitCommandLog,
    enabled: isOpen,
  })

  return (
    <Popover
      open={isOpen}
      onOpenChange={(open) => {
        if (open) queryClient.invalidateQueries({ queryKey: ['gitCommandLog'] })
        setIsOpen(open)
      }}
    >
      <PopoverTrigger asChild>
        <button
          type="button"
          className="px-2 bg-surface rounded-md flex-1 flex items-center gap-2 hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 min-w-0"
          disabled={!latestCommand}
        >
          <Tooltip>
            <TooltipTrigger asChild>
              <ListTreeIcon className="h-4 w-4 shrink-0" />
            </TooltipTrigger>
            <TooltipContent>Git command log</TooltipContent>
          </Tooltip>
          <span className="font-mono text-xs text-foreground/70 whitespace-nowrap overflow-hidden text-ellipsis mx-1">
            {latestCommand?.command ?? 'No commands run'}
          </span>
        </button>
      </PopoverTrigger>

      <PopoverContent className="max-h-[min(400px,_var(--radix-popover-content-available-height))] w-[var(--radix-popover-content-available-width)] p-2 flex flex-col gap-1">
        {log?.map((item) => (
          <LogItem key={item.id} item={item} />
        ))}
      </PopoverContent>
    </Popover>
  )
}

const LogItem = ({ item }: { item: GitCommandLogType }) => {
  return (
    <div className="flex gap-2 text-sm items-center font-mono whitespace-nowrap">
      <span className="text-foreground/60">{item.created_at}</span>
      {item.command_type === 'Query' && <SearchIcon className="h-3 w-3" />}
      {item.command_type === 'Mutation' && <PencilIcon className="h-3 w-3" />}
      <span>{item.command}</span>
    </div>
  )
}
