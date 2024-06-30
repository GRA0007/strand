import { Tooltip, type TooltipContentProps, TooltipTrigger } from '@radix-ui/react-tooltip'
import { FileInputIcon, MinusIcon, PencilIcon, PlusIcon } from 'lucide-react'
import type { FileStatus } from '../../bindings'
import { cn } from '../../utils/cn'
import { TooltipContent } from '../UI/Tooltip'

export const FileName = ({
  path,
  status,
  iconClassName,
  tooltipProps,
}: { status?: FileStatus; path: string; iconClassName?: string; tooltipProps?: TooltipContentProps }) => {
  const pathParts = path.split('/')
  const folderPath = pathParts.slice(0, -1).join('/')
  const filename = pathParts.slice(-1)[0]

  return (
    <div className="flex items-center gap-2 flex-1">
      {(status === 'Added' || status === 'Copied') && (
        <Tooltip>
          <TooltipTrigger asChild>
            <PlusIcon className={cn('text-success h-4 w-4 shrink-0', iconClassName)} />
          </TooltipTrigger>
          <TooltipContent>Added</TooltipContent>
        </Tooltip>
      )}
      {status === 'Deleted' && (
        <Tooltip>
          <TooltipTrigger asChild>
            <MinusIcon className={cn('text-error h-4 w-4 shrink-0', iconClassName)} />
          </TooltipTrigger>
          <TooltipContent>Deleted</TooltipContent>
        </Tooltip>
      )}
      {(status === 'Modified' || status === 'TypeChanged') && (
        <Tooltip>
          <TooltipTrigger asChild>
            <PencilIcon className={cn('text-warn h-4 w-4 shrink-0', iconClassName)} />
          </TooltipTrigger>
          <TooltipContent>Modified</TooltipContent>
        </Tooltip>
      )}
      {status === 'Renamed' && (
        <Tooltip>
          <TooltipTrigger asChild>
            <FileInputIcon className={cn('text-info h-4 w-4 shrink-0', iconClassName)} />
          </TooltipTrigger>
          <TooltipContent>Renamed</TooltipContent>
        </Tooltip>
      )}

      <Tooltip>
        <TooltipTrigger asChild>
          <div className="whitespace-nowrap inline-flex flex-1 w-0">
            {folderPath && (
              <>
                <span className="text-foreground/60 text-ellipsis flex-[0_1_content] overflow-hidden">
                  {folderPath}
                </span>
                <span className="text-foreground/60">/</span>
              </>
            )}
            <span>{filename}</span>
          </div>
        </TooltipTrigger>
        <TooltipContent {...tooltipProps}>{path}</TooltipContent>
      </Tooltip>
    </div>
  )
}
