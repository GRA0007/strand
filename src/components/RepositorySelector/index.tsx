import { DropdownMenu, DropdownMenuTrigger } from '@radix-ui/react-dropdown-menu'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { useQueryClient } from '@tanstack/react-query'
import { open } from '@tauri-apps/plugin-dialog'
import { CheckIcon, ChevronDownIcon, CopyIcon, FolderIcon, PlusIcon } from 'lucide-react'
import { commands } from '../../bindings'
import { cn } from '../../utils/cn'
import { useCommandMutation } from '../../utils/useCommandMutation'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '../UI/DropdownMenu'
import { TooltipContent } from '../UI/Tooltip'

export const RepositorySelector = () => {
  const queryClient = useQueryClient()

  const { data: state } = useCommandQuery({
    queryKey: ['state'],
    queryFn: commands.getStateData,
  })

  const addRepository = useCommandMutation({
    mutationFn: commands.addRepositoryFromPath,
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: ['state'] }),
        queryClient.invalidateQueries({ queryKey: ['branches'] }),
      ]),
    onError: () => {
      console.error('Failed')
    },
  })

  const setOpenRepository = useCommandMutation({
    mutationFn: commands.setOpenRepository,
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: ['state'] }),
        queryClient.invalidateQueries({ queryKey: ['branches'] }),
      ]),
  })

  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="font-semibold flex gap-2 items-center rounded outline-none hover:bg-foreground/10 data-[state='open']:bg-foreground/10 h-8 px-2 select-none">
        {state?.open_repository?.name ?? 'Open a repository'}{' '}
        <ChevronDownIcon className="h-3 w-3 [[data-state='open']_&]:rotate-180" />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {state?.repositories?.map((repo) => (
          <DropdownMenuItem
            key={repo.name}
            className={cn('justify-end pl-3 text-base gap-2', state.open_repository?.id === repo.id && 'font-semibold')}
            onClick={() => {
              setOpenRepository.mutate(repo.id)
            }}
          >
            {repo.name}{' '}
            {state?.open_repository?.id === repo.id ? (
              <CheckIcon className="h-3 w-3" />
            ) : repo.has_changes ? (
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

        {(state?.repositories?.length ?? 0) > 0 && <DropdownMenuSeparator />}

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
            addRepository.mutate(folder)
          }}
        >
          <span className="text-foreground/80">Open</span>
          <FolderIcon className="h-3 w-3 opacity-60" />
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
