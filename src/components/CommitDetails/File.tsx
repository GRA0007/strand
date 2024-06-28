import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { FileInputIcon, MinusIcon, PencilIcon, PlusIcon } from 'lucide-react'
import type { File } from '../../bindings'
import { cn } from '../../utils/cn'
import { TooltipContent } from '../UI/Tooltip'

export const CommitFile = ({
  id,
  file,
  isSelected,
  onSelect,
}: { id: string; file: File; isSelected: boolean; onSelect: () => void }) => {
  const pathParts = (file.dst_path ?? file.src_path).split('/')
  const path = pathParts.slice(0, -1).join('/')
  const filename = pathParts.slice(-1)[0]

  return (
    <div
      className={cn(
        'flex items-center gap-2 px-3 py-1 text-sm active:bg-foreground/20',
        isSelected ? 'bg-foreground/20' : 'hover:bg-foreground/10',
      )}
      onPointerDown={() => onSelect()}
      id={id}
      tabIndex={-1}
    >
      {(file.status === 'Added' || file.status === 'Copied') && (
        <Tooltip>
          <TooltipTrigger asChild>
            <PlusIcon className="text-success h-3 w-3 shrink-0" />
          </TooltipTrigger>
          <TooltipContent>Added</TooltipContent>
        </Tooltip>
      )}
      {file.status === 'Deleted' && (
        <Tooltip>
          <TooltipTrigger asChild>
            <MinusIcon className="text-error h-3 w-3 shrink-0" />
          </TooltipTrigger>
          <TooltipContent>Deleted</TooltipContent>
        </Tooltip>
      )}
      {(file.status === 'Modified' || file.status === 'TypeChanged') && (
        <Tooltip>
          <TooltipTrigger asChild>
            <PencilIcon className="text-warn h-3 w-3 shrink-0" />
          </TooltipTrigger>
          <TooltipContent>Modified</TooltipContent>
        </Tooltip>
      )}
      {file.status === 'Renamed' && (
        <Tooltip>
          <TooltipTrigger asChild>
            <FileInputIcon className="text-info h-3 w-3 shrink-0" />
          </TooltipTrigger>
          <TooltipContent>Renamed</TooltipContent>
        </Tooltip>
      )}

      <Tooltip>
        <TooltipTrigger asChild>
          <div className="whitespace-nowrap inline-flex flex-1 w-0">
            {path && (
              <>
                <span className="text-foreground/60 text-ellipsis flex-[0_1_content] overflow-hidden">{path}</span>
                <span className="text-foreground/60">/</span>
              </>
            )}
            <span>{filename}</span>
          </div>
        </TooltipTrigger>
        <TooltipContent side="left" sideOffset={24}>
          {file.dst_path ?? file.src_path}
        </TooltipContent>
      </Tooltip>
    </div>
  )
}
