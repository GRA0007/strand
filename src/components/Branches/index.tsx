import { useQuery } from '@tanstack/react-query'
import { CircleDashedIcon, CircleIcon, PaintbrushIcon, PlusIcon } from 'lucide-react'
import { PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { listBranches } from '../../commands'
import { BranchListPanel } from '../BranchListPanel'
import { IconButton } from '../IconButton'

export const Branches = () => {
  const { data } = useQuery({
    queryKey: ['branches'],
    queryFn: () => listBranches(),
  })

  return (
    <PanelGroup direction="vertical">
      <BranchListPanel
        icon={<CircleIcon className="h-4 w-4" />}
        title="Local"
        actions={
          <IconButton tooltip="Clean merged branches" size="sm">
            <PaintbrushIcon />
          </IconButton>
        }
      >
        {data
          ?.filter((b) => !b.remote)
          .map((b) => (
            <li key={b.name}>{b.name}</li>
          ))}
      </BranchListPanel>

      <PanelResizeHandle className="h-4" />

      <BranchListPanel
        icon={<CircleDashedIcon className="h-4 w-4" />}
        title="Remote"
        actions={
          <IconButton tooltip="Add remote" size="sm">
            <PlusIcon />
          </IconButton>
        }
      >
        {data
          ?.filter((b) => b.remote)
          .map((b) => (
            <li key={b.name}>
              {b.remote}/{b.name}
            </li>
          ))}
      </BranchListPanel>
    </PanelGroup>
  )
}
