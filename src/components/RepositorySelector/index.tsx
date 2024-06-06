import { DropdownMenu, DropdownMenuTrigger } from '@radix-ui/react-dropdown-menu'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { open } from '@tauri-apps/plugin-dialog'
import { CheckIcon, ChevronDownIcon, CopyIcon, FolderIcon, PlusIcon } from 'lucide-react'
import { commands } from '../../bindings'
import { cn } from '../../utils/cn'
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '../DropdownMenu'
import { TooltipContent } from '../Tooltip'

export const RepositorySelector = () => {
  const queryClient = useQueryClient()

  const { data: repositories } = useQuery({
    queryKey: ['repositories'],
    queryFn: () =>
      commands.getRepositories().then((res) => {
        if (res.status === 'error') throw res.error
        return res.data
      }),
  })
  const { data: state } = useQuery({
    queryKey: ['state'],
    queryFn: () =>
      commands.getState().then((res) => {
        if (res.status === 'error') throw res.error
        return res.data
      }),
  })

  const addRepository = useMutation({
    mutationFn: (localPath: string) => commands.addRepository(localPath, 'now!'),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['repositories'] })
      await queryClient.invalidateQueries({ queryKey: ['state'] })
    },
  })

  const setOpenRepository = useMutation({
    mutationFn: (id: number) => commands.setOpenRepository(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['state'] })
    },
  })

  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="font-semibold flex gap-2 items-center rounded outline-none hover:bg-foreground/10 data-[state='open']:bg-foreground/10 h-8 px-2 select-none">
        {state?.open_repository ? repositories?.find((r) => r.id === state.open_repository)?.name : 'Open a repository'}{' '}
        <ChevronDownIcon className="h-3 w-3 [[data-state='open']_&]:rotate-180" />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {repositories?.map((repo) => (
          <DropdownMenuItem
            key={repo.name}
            className={cn('justify-end pl-3 text-base gap-2', state?.open_repository === repo.id && 'font-semibold')}
            onClick={() => {
              setOpenRepository.mutate(repo.id)
            }}
          >
            {repo.name}{' '}
            {state?.open_repository === repo.id ? (
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
            addRepository.mutate(folder)
          }}
        >
          <span className="text-foreground/80">Open from file</span>
          <FolderIcon className="h-3 w-3 opacity-60" />
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
