import { useQueryClient } from '@tanstack/react-query'
import {
  ArrowDownToLineIcon,
  ArrowUpFromLineIcon,
  GitBranchPlusIcon,
  RedoIcon,
  RefreshCwIcon,
  SearchIcon,
  SettingsIcon,
  UndoIcon,
} from 'lucide-react'
import { commands } from '../../bindings'
import { cn } from '../../utils/cn'
import { useCommandMutation } from '../../utils/useCommandMutation'
import { RepositorySelector } from '../RepositorySelector'
import { IconButton } from '../UI/IconButton'

export const Toolbar = () => {
  const queryClient = useQueryClient()

  const fetchAll = useCommandMutation({
    mutationFn: commands.gitFetch,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['branches'] }),
  })

  return (
    <nav className="flex gap-3 items-center">
      <IconButton
        tooltip={fetchAll.isPending ? 'Fetching...' : 'Fetch'}
        onClick={() => fetchAll.mutate()}
        disabled={fetchAll.isPending}
      >
        <RefreshCwIcon className={cn(fetchAll.isPending && 'animate-spin')} />
      </IconButton>
      <IconButton tooltip="Pull (fast-forward if possible)" disabled>
        <ArrowDownToLineIcon />
      </IconButton>
      <IconButton tooltip="Push" disabled>
        <ArrowUpFromLineIcon />
      </IconButton>
      <IconButton tooltip="Create branch" disabled>
        <GitBranchPlusIcon />
      </IconButton>

      <div className="w-px bg-foreground/20 mx-2 self-stretch" />

      <IconButton tooltip="Undo" disabled>
        <UndoIcon />
      </IconButton>
      <IconButton tooltip="Redo" disabled>
        <RedoIcon />
      </IconButton>

      <div className="relative group ml-4">
        <SearchIcon className="h-4 w-4 absolute left-2 top-2 opacity-50 group-focus-within:opacity-100" />
        <input
          className="bg-foreground/10 rounded-md h-8 px-2 pl-8 text-sm outline-none border-none w-[170px]"
          placeholder="Search..."
          disabled
        />
      </div>

      <div className="flex-1" />

      <RepositorySelector />
      <IconButton tooltip="Settings" disabled>
        <SettingsIcon />
      </IconButton>
    </nav>
  )
}
