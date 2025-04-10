import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { FileInputIcon, MinusIcon, PencilIcon, PlusIcon } from 'lucide-react'
import type { File } from '../../bindings'
import { TooltipContent } from '../UI/Tooltip'

export const CommitStats = ({ files, className }: { files: File[]; className?: string }) => {
  const stats = calculateStats(files)

  return (
    <div className={className}>
      <div className="flex gap-2.5 items-center">
        {stats.modified > 0 && (
          <Tooltip>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-1 text-warn text-xs font-semibold">
                <PencilIcon className="h-3 w-3" />
                {stats.modified}
              </div>
            </TooltipTrigger>
            <TooltipContent>{stats.modified} modified</TooltipContent>
          </Tooltip>
        )}
        {stats.added > 0 && (
          <Tooltip>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-1 text-success text-xs font-semibold">
                <PlusIcon className="h-3 w-3" />
                {stats.added}
              </div>
            </TooltipTrigger>
            <TooltipContent>{stats.added} added</TooltipContent>
          </Tooltip>
        )}
        {stats.deleted > 0 && (
          <Tooltip>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-1 text-error text-xs font-semibold">
                <MinusIcon className="h-3 w-3" />
                {stats.deleted}
              </div>
            </TooltipTrigger>
            <TooltipContent>{stats.deleted} deleted</TooltipContent>
          </Tooltip>
        )}
        {stats.renamed > 0 && (
          <Tooltip>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-1 text-info text-xs font-semibold">
                <FileInputIcon className="h-3 w-3" />
                {stats.renamed}
              </div>
            </TooltipTrigger>
            <TooltipContent>{stats.renamed} renamed</TooltipContent>
          </Tooltip>
        )}
      </div>
    </div>
  )
}

const calculateStats = (files: File[] | undefined) => {
  let added = 0
  let deleted = 0
  let modified = 0
  let renamed = 0

  for (const file of files ?? []) {
    if (file.status === 'Added' || file.status === 'Copied') added++
    if (file.status === 'Deleted') deleted++
    if (file.status === 'Modified' || file.status === 'TypeChanged') modified++
    if (file.status === 'Renamed') renamed++
  }

  return { added, deleted, modified, renamed }
}
