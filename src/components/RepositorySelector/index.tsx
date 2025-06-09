import { DropdownMenu, DropdownMenuTrigger } from '@radix-ui/react-dropdown-menu'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { useQueryClient } from '@tanstack/react-query'
import { open } from '@tauri-apps/plugin-dialog'
import { useSetAtom } from 'jotai'
import { CheckIcon, ChevronDownIcon, CopyIcon, FolderIcon, PlusIcon } from 'lucide-react'
import { commands } from '../../bindings'
import { useOpenRepository } from '../../data/useOpenRepository'
import { selectedCommitHashAtom } from '../../ui-state'
import { cn } from '../../utils/cn'
import { useCommandMutation } from '../../utils/useCommandMutation'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '../UI/DropdownMenu'
import { TooltipContent } from '../UI/Tooltip'

export const RepositorySelector = () => {
  const queryClient = useQueryClient()

  const openRepository = useOpenRepository()

  const { data: repositories } = useCommandQuery({
    queryKey: ['repositories'],
    queryFn: commands.getRepositories,
  })

  const addRepository = useCommandMutation({
    mutationFn: commands.addRepositoryFromPath,
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: ['openRepository'] }),
        queryClient.invalidateQueries({ queryKey: ['repositories'] }),
      ]),
  })

  const setSelectedCommitHash = useSetAtom(selectedCommitHashAtom)

  const setOpenRepository = useCommandMutation({
    mutationFn: commands.setOpenRepository,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['openRepository'] })
      setSelectedCommitHash(null)
    },
  })

  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="font-semibold flex gap-2 items-center rounded-sm outline-hidden hover:bg-foreground/10 data-[state='open']:bg-foreground/10 h-8 px-2 select-none">
        {openRepository?.name ?? 'Open a repository'}{' '}
        <ChevronDownIcon className="h-3 w-3 in-data-[state='open']:rotate-180" />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {repositories?.map((repo) => (
          <DropdownMenuItem
            key={repo.name}
            className={cn('justify-end pl-3 text-base gap-2', openRepository?.id === repo.id && 'font-semibold')}
            onClick={() => {
              setOpenRepository.mutate(repo.id)
            }}
          >
            {repo.name}{' '}
            {openRepository?.id === repo.id ? (
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

        {(repositories?.length ?? 0) > 0 && <DropdownMenuSeparator />}

        <DropdownMenuItem className="gap-2 justify-end" disabled>
          <span className="text-foreground/80">Create</span>
          <PlusIcon className="h-3 w-3 opacity-60" />
        </DropdownMenuItem>
        <DropdownMenuItem className="gap-2 justify-end" disabled>
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
