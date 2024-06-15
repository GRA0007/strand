import { Popover, PopoverTrigger } from '@radix-ui/react-popover'
import { Select, SelectValue } from '@radix-ui/react-select'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { useQueryClient } from '@tanstack/react-query'
import { FilterIcon, ListTreeIcon, PencilIcon, SearchIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import { events, type GitCommandLog as GitCommandLogType, commands } from '../../bindings'
import { formatDate } from '../../utils/formatDate'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { PopoverContent } from '../UI/Popover'
import { SelectContent, SelectItem, SelectTrigger } from '../UI/Select'
import { TooltipContent } from '../UI/Tooltip'

export const GitCommandLog = () => {
  const queryClient = useQueryClient()
  const [isOpen, setIsOpen] = useState(false)
  const [filter, setFilter] = useState<'all' | 'query' | 'mutation'>('mutation')

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
          className="bg-surface rounded-md flex-1 flex items-center hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 min-w-0"
          disabled={!latestCommand}
        >
          <Tooltip>
            <TooltipTrigger asChild>
              <ListTreeIcon className="h-8 w-8 p-2 shrink-0" />
            </TooltipTrigger>
            <TooltipContent>Git command log</TooltipContent>
          </Tooltip>
          <span className="font-mono text-xs text-foreground/70 whitespace-nowrap overflow-hidden text-ellipsis ml-1 mr-3">
            {latestCommand?.command ?? 'No commands run'}
          </span>
        </button>
      </PopoverTrigger>

      <PopoverContent className="max-h-[min(400px,_var(--radix-popover-content-available-height))] w-[var(--radix-popover-content-available-width)] p-0 flex flex-col">
        <div className="border-b border-foreground/10 pl-2 pr-1 py-1 flex items-center gap-2">
          <span className="text-sm font-semibold text-foreground/70 mr-auto">Git command log</span>
          <FilterIcon className="h-4 w-4" />
          <Select value={filter} onValueChange={(v) => setFilter(v as typeof filter)}>
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent align="end">
              <SelectItem value="all">All</SelectItem>
              <SelectItem value="query">Queries</SelectItem>
              <SelectItem value="mutation">Mutations</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="overflow-auto flex-1 p-2">
          {log?.flatMap((item) => {
            if (filter === 'mutation' && item.command_type === 'Query') return []
            if (filter === 'query' && item.command_type === 'Mutation') return []
            return [<LogItem key={item.id} item={item} />]
          })}
        </div>
      </PopoverContent>
    </Popover>
  )
}

const LogItem = ({ item }: { item: GitCommandLogType }) => {
  return (
    <div className="flex gap-2 text-sm items-center font-mono whitespace-nowrap">
      <span className="text-foreground/60" title={formatDate(item.created_at)}>
        {formatDate(item.created_at, 'yyyy-MM-dd HH:mm:ss')}
      </span>
      {item.command_type === 'Query' && <SearchIcon className="h-3 w-3" />}
      {item.command_type === 'Mutation' && <PencilIcon className="h-3 w-3" />}
      <span>{item.command}</span>
    </div>
  )
}
