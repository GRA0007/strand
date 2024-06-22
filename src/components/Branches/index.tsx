import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import {
  CircleDashedIcon,
  CircleIcon,
  MoveDownIcon,
  MoveUpIcon,
  PaintbrushIcon,
  PlusIcon,
  TriangleAlertIcon,
} from 'lucide-react'
import { PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { type UpstreamTrack, commands } from '../../bindings'
import { useOpenRepository } from '../../data/useOpenRepository'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { BranchListPanel } from '../BranchListPanel'
import { IconButton } from '../UI/IconButton'
import { TooltipContent } from '../UI/Tooltip'

export const Branches = () => {
  const openRepository = useOpenRepository()

  const { data } = useCommandQuery({
    queryKey: ['branches', openRepository?.id],
    enabled: Boolean(openRepository),
    queryFn: commands.getBranches,
    refetchOnWindowFocus: true,
  })

  return (
    <PanelGroup direction="vertical">
      <BranchListPanel
        icon={<CircleIcon className="h-4 w-4" />}
        title="Local"
        actions={
          <IconButton tooltip="Clean merged branches" size="sm" disabled>
            <PaintbrushIcon />
          </IconButton>
        }
        items={
          data?.local.map((d) => ({
            hash: d.hash,
            path: d.name,
            children: (
              <>
                <span className="mr-auto">{d.name[d.name.length - 1]}</span>
                <BranchDelta upstreamTrack={d.upstream_track} />
              </>
            ),
          })) ?? []
        }
      />

      <PanelResizeHandle className="h-4" />

      <BranchListPanel
        icon={<CircleDashedIcon className="h-4 w-4" />}
        title="Remote"
        actions={
          <IconButton tooltip="Add remote" size="sm" disabled>
            <PlusIcon />
          </IconButton>
        }
        items={
          data?.remote.map((d) => ({
            hash: d.hash,
            path: d.name,
            children: (
              <>
                <span className="mr-auto">{d.name[d.name.length - 1]}</span>
              </>
            ),
          })) ?? []
        }
      />
    </PanelGroup>
  )
}

const BranchDelta = ({ upstreamTrack }: { upstreamTrack: UpstreamTrack }) => {
  if (!upstreamTrack)
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <div>
            <TriangleAlertIcon className="h-3 w-3" />
          </div>
        </TooltipTrigger>
        <TooltipContent>Upstream branch missing</TooltipContent>
      </Tooltip>
    )

  if (upstreamTrack[0] === 0 && upstreamTrack[1] === 1) return null

  return (
    <>
      {upstreamTrack[0] !== 0 && (
        <Tooltip>
          <TooltipTrigger asChild className="flex items-center text-xs gap-px">
            <div>
              {upstreamTrack[0]} <MoveUpIcon className="h-3 w-3" />
            </div>
          </TooltipTrigger>
          <TooltipContent>{upstreamTrack[0]} ahead</TooltipContent>
        </Tooltip>
      )}
      {upstreamTrack[1] !== 0 && (
        <Tooltip>
          <TooltipTrigger asChild className="flex items-center text-xs gap-px">
            <div>
              {upstreamTrack[1]} <MoveDownIcon className="h-3 w-3" />
            </div>
          </TooltipTrigger>
          <TooltipContent>{upstreamTrack[1]} behind</TooltipContent>
        </Tooltip>
      )}
    </>
  )
}
