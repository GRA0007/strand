import { Fragment } from 'react'
import type { DiffStatus, FileDiff, LineDiff } from '../../bindings'
import { cn } from '../../utils/cn'

export const Diff = ({ diff }: { diff: FileDiff }) => {
  return (
    <pre className="flex-1 overflow-auto py-2 text-sm select-text cursor-auto">
      <code className="w-max min-w-full grid grid-cols-[auto_auto_1fr]">
        {diff.map((hunk) => (
          <Fragment key={hunk.header}>
            <div className="bg-info/30 py-1 px-3 text-foreground/70 col-span-full">{hunk.header}</div>
            {hunk.lines.map((line, i) => (
              <DiffLine key={`${hunk.header}-${i}`} id={`${hunk.header}-${i}`} line={line} />
            ))}
          </Fragment>
        ))}
      </code>
    </pre>
  )
}

const DiffLine = ({ line, id }: { line: LineDiff; id: string }) => {
  const numUnmodified = line.words.filter((word) => word.status === 'Unmodified' && word.text.trim().length > 0).length

  return (
    <Line srcLineNumber={line.src_line_number} dstLineNumber={line.dst_line_number} status={line.status}>
      {line.words.map((word, i) => (
        <DiffWord
          key={`${id}-${i}`}
          className={cn(
            word.status === 'Removed' && numUnmodified > 0 && 'bg-error/20',
            word.status === 'Added' && numUnmodified > 0 && 'bg-success/20',
          )}
        >
          {word.text}
        </DiffWord>
      ))}
    </Line>
  )
}

const Line = ({
  status,
  children,
  srcLineNumber,
  dstLineNumber,
}: { status?: DiffStatus; children: React.ReactNode; srcLineNumber: number | null; dstLineNumber: number | null }) => {
  const lineNumberClassName = cn(
    'px-2 select-none cursor-default text-right',
    status === 'Unmodified' && 'text-foreground/60',
    status === 'Added' && 'bg-success/40',
    status === 'Removed' && 'bg-error/40',
  )

  return (
    <>
      <span className={lineNumberClassName}>{srcLineNumber ?? ''}</span>
      <span className={lineNumberClassName}>{dstLineNumber ?? ''}</span>
      <div className={cn('pr-3', status === 'Added' && 'bg-success/20', status === 'Removed' && 'bg-error/20')}>
        <span className="inline-block select-none cursor-default px-2">
          {status === 'Unmodified' && ' '}
          {status === 'Added' && '+'}
          {status === 'Removed' && '-'}
        </span>
        {children}
      </div>
    </>
  )
}

const DiffWord = ({ children, className }: { children: React.ReactNode; className?: string }) => {
  return <span className={cn('inline-block', className)}>{children}</span>
}
