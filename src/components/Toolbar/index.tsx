import {
  ArrowDownToLineIcon,
  ArrowUpFromLineIcon,
  ChevronDownIcon,
  GitBranchPlusIcon,
  RedoIcon,
  RefreshCwIcon,
  SearchIcon,
  SettingsIcon,
  UndoIcon,
} from 'lucide-react'
import { IconButton } from '../IconButton'

export const Toolbar = () => {
  return (
    <nav className="flex gap-3 items-center">
      <IconButton tooltip="Fetch">
        <RefreshCwIcon />
      </IconButton>
      <IconButton tooltip="Pull (fast-forward if possible)">
        <ArrowDownToLineIcon />
      </IconButton>
      <IconButton tooltip="Push">
        <ArrowUpFromLineIcon />
      </IconButton>
      <IconButton tooltip="Create branch">
        <GitBranchPlusIcon />
      </IconButton>

      <div className="w-px bg-white/20 mx-2 self-stretch" />

      <IconButton tooltip="Undo">
        <UndoIcon />
      </IconButton>
      <IconButton tooltip="Redo" disabled>
        <RedoIcon />
      </IconButton>

      <div className="relative group ml-4">
        <SearchIcon className="h-4 w-4 absolute left-2 top-2 opacity-50 group-focus-within:opacity-100" />
        <input
          className="bg-white/10 rounded h-8 px-2 pl-8 text-sm outline-none border-none w-[170px]"
          placeholder="Search..."
        />
      </div>

      <div className="flex-1" />

      <button
        type="button"
        className="font-semibold flex gap-2 items-center rounded hover:bg-foreground/10 active:bg-foreground/20 h-8 px-2"
      >
        strand <ChevronDownIcon className="h-3 w-3" />
      </button>
      <IconButton tooltip="Settings">
        <SettingsIcon />
      </IconButton>
    </nav>
  )
}
