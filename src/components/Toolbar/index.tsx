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
import { Button } from '../Button'

export const Toolbar = () => {
  return (
    <nav className="flex gap-4 items-center">
      <Button>
        <RefreshCwIcon className="h-5 w-5" />
      </Button>
      <Button>
        <ArrowDownToLineIcon className="h-5 w-5" />
      </Button>
      <Button>
        <ArrowUpFromLineIcon className="h-5 w-5" />
      </Button>
      <Button>
        <GitBranchPlusIcon className="h-5 w-5" />
      </Button>

      <div className="w-px bg-white/20 self-stretch" />

      <Button>
        <UndoIcon className="h-5 w-5" />
      </Button>
      <Button disabled>
        <RedoIcon className="h-5 w-5" />
      </Button>

      <div className="relative group ml-4">
        <SearchIcon className="h-4 w-4 absolute left-2 top-2 opacity-50 group-focus-within:opacity-100" />
        <input className="bg-white/10 rounded h-8 px-2 pl-8 text-sm outline-none" placeholder="Search..." />
      </div>

      <div className="flex-1" />

      <div className="font-semibold flex gap-2 items-center">
        stevent <ChevronDownIcon className="h-3 w-3" />
      </div>
      <Button>
        <SettingsIcon className="h-5 w-5" />
      </Button>
    </nav>
  )
}
