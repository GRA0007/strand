import type { File } from '../../bindings'
import { cn } from '../../utils/cn'
import { FileName } from '../FileName'

export const CommitFile = ({
  id,
  file,
  isSelected,
  onSelect,
}: { id: string; file: File; isSelected: boolean; onSelect: () => void }) => {
  return (
    <div
      className={cn(
        'px-3 py-1 text-sm active:bg-foreground/20',
        isSelected ? 'bg-foreground/20' : 'hover:bg-foreground/10',
      )}
      onPointerDown={() => onSelect()}
      id={id}
      tabIndex={-1}
    >
      <FileName
        path={file.dst_path ?? file.src_path}
        status={file.status}
        tooltipProps={{ side: 'left', sideOffset: 24 }}
        iconClassName="h-3 w-3"
      />
    </div>
  )
}
