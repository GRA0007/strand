import { DropdownMenu, DropdownMenuTrigger } from '@radix-ui/react-dropdown-menu'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { open } from '@tauri-apps/plugin-dialog'
import { CheckIcon, ChevronDownIcon, CopyIcon, FolderIcon, PlusIcon } from 'lucide-react'
import { useStore } from '../../store'
import { cn } from '../../utils/cn'
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '../DropdownMenu'
import { TooltipContent } from '../Tooltip'

type Repository = {
  name: string
  path: string
  hasChanges: boolean
}

export const RepositorySelector = () => {
  const [repositories, setRepositories] = useStore<Repository[]>('repositories', [])
  const [activeRepository, setActiveRepository] = useStore<string>('active-repository')

  console.log(repositories)

  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="font-semibold flex gap-2 items-center rounded outline-none hover:bg-foreground/10 data-[state='open']:bg-foreground/10 h-8 px-2 select-none">
        {activeRepository ?? 'Open a repository'}{' '}
        <ChevronDownIcon className="h-3 w-3 [[data-state='open']_&]:rotate-180" />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {repositories?.map((repo) => (
          <DropdownMenuItem
            key={repo.name}
            className={cn('justify-end pl-3 text-base gap-2', repo.name === activeRepository && 'font-semibold')}
          >
            {repo.name}{' '}
            {repo.name === activeRepository ? (
              <CheckIcon className="h-3 w-3" />
            ) : repo.hasChanges ? (
              <Tooltip>
                <TooltipTrigger className="h-3 w-3 flex items-center justify-center">
                  <div className="rounded-full h-1 w-1 bg-foreground/40" />
                </TooltipTrigger>
                <TooltipContent>New updates</TooltipContent>
              </Tooltip>
            ) : (
              <div className="h-3 w-3" />
            )}
          </DropdownMenuItem>
        ))}

        {(repositories?.length ?? 0) > 0 && <DropdownMenuSeparator />}

        <DropdownMenuItem className="gap-2 justify-end">
          <span className="text-foreground/80">Create</span>
          <PlusIcon className="h-3 w-3 opacity-60" />
        </DropdownMenuItem>
        <DropdownMenuItem className="gap-2 justify-end">
          <span className="text-foreground/80">Clone</span>
          <CopyIcon className="h-3 w-3 opacity-60" />
        </DropdownMenuItem>
        <DropdownMenuItem
          className="gap-2 justify-end"
          onClick={async () => {
            const folder = await open({
              canCreateDirectories: false,
              directory: true,
              multiple: false,
            })
            if (!folder) return
            // TODO: check it's a valid git folder
            const name = folder.split('/')[folder.split('/').length - 1]
            setRepositories([...(repositories ?? []), { name, path: folder, hasChanges: false }])
            setActiveRepository(name)
          }}
        >
          <span className="text-foreground/80">Open from file</span>
          <FolderIcon className="h-3 w-3 opacity-60" />
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
