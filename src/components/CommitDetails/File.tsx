import { FileInputIcon, MinusIcon, PencilIcon, PlusIcon } from 'lucide-react'
import type { File } from '../../bindings'

export const CommitFile = ({ file }: { file: File }) => {
  const pathParts = (file.dst_path ?? file.src_path).split('/')

  return (
    <div className="flex items-center gap-2 px-3 py-1 text-sm hover:bg-foreground/10">
      {(file.status === 'Added' || file.status === 'Copied') && <PlusIcon className="text-success h-3 w-3 shrink-0" />}
      {file.status === 'Deleted' && <MinusIcon className="text-error h-3 w-3 shrink-0" />}
      {(file.status === 'Modified' || file.status === 'TypeChanged') && (
        <PencilIcon className="text-warn h-3 w-3 shrink-0" />
      )}
      {file.status === 'Renamed' && <FileInputIcon className="text-info h-3 w-3 shrink-0" />}
      <div className="whitespace-nowrap overflow-hidden">
        <span className="text-foreground/60 text-ellipsis">
          {pathParts.slice(0, -1).join('/')}
          {pathParts.length > 1 && '/'}
        </span>
        <span>{pathParts.slice(-1)[0]}</span>
      </div>
    </div>
  )
}
