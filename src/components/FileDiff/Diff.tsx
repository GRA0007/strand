import { Fragment } from 'react'
import type { FileDiff, WordDiff } from '../../bindings'
import { cn } from '../../utils/cn'

export const Diff = ({ diff }: { diff: FileDiff }) => {
  return (
    <pre className="flex-1 overflow-auto py-2 text-sm select-text cursor-auto">
      <code className="w-max min-w-full grid grid-cols-[auto_auto_1fr]">
        {diff.map((hunk) => (
          <Fragment key={hunk.header}>
            <div className="bg-info/30 py-1 px-3 text-foreground/70 col-span-full">{hunk.header}</div>
            {hunk.lines.map((line, i) => (
              <DiffLine
                key={`${hunk.header}-${i}`}
                id={`${hunk.header}-${i}`}
                line={line}
                srcLineNumber={hunk.src_line + i}
                dstLineNumber={hunk.dst_line + i}
              />
            ))}
          </Fragment>
        ))}
      </code>
    </pre>
  )
}

const DiffLine = ({
  line,
  id,
  srcLineNumber,
  dstLineNumber,
}: { line: WordDiff[]; id: string; srcLineNumber: number; dstLineNumber: number }) => {
  const [numAdded, numRemoved, numUnmodified] = line.reduce(
    ([a, r, u], word) => [
      a + (word.status === 'Added' ? 1 : 0),
      r + (word.status === 'Removed' ? 1 : 0),
      u + (word.status === 'Unmodified' && word.text.trim().length > 0 ? 1 : 0),
    ],
    [0, 0, 0],
  )

  // Context line
  if (numAdded === 0 && numRemoved === 0)
    return (
      <Line srcLineNumber={srcLineNumber} dstLineNumber={dstLineNumber}>
        {line.map((word, i) => (
          <DiffWord key={`${id}-${i}`}>{word.text}</DiffWord>
        ))}
      </Line>
    )

  return (
    <>
      {numRemoved > 0 && (
        <Line status="removed" srcLineNumber={srcLineNumber}>
          {line
            .filter((w) => w.status !== 'Added')
            .map((word, i) => (
              <DiffWord
                key={`${id}-${i}`}
                className={cn(word.status === 'Removed' && numUnmodified > 0 && 'bg-error/20')}
              >
                {word.text}
              </DiffWord>
            ))}
        </Line>
      )}
      {(numAdded > 0 || numRemoved > 0) && (
        <Line status="added" dstLineNumber={dstLineNumber}>
          {line
            .filter((w) => w.status !== 'Removed')
            .map((word, i) => (
              <DiffWord
                key={`${id}-${i}`}
                className={cn(word.status === 'Added' && numUnmodified > 0 && 'bg-success/20')}
              >
                {word.text}
              </DiffWord>
            ))}
        </Line>
      )}
    </>
  )
}

const Line = ({
  status,
  children,
  srcLineNumber,
  dstLineNumber,
}: { status?: 'added' | 'removed'; children: React.ReactNode; srcLineNumber?: number; dstLineNumber?: number }) => {
  const lineNumberClassName = cn(
    'px-2 select-none cursor-default text-right',
    !status && 'text-foreground/60',
    status === 'added' && 'bg-success/40',
    status === 'removed' && 'bg-error/40',
  )

  return (
    <>
      <span className={lineNumberClassName}>{srcLineNumber}</span>
      <span className={lineNumberClassName}>{dstLineNumber}</span>
      <div className={cn('pr-3', status === 'added' && 'bg-success/20', status === 'removed' && 'bg-error/20')}>
        <span className="inline-block select-none cursor-default px-2">
          {!status && ' '}
          {status === 'added' && '+'}
          {status === 'removed' && '-'}
        </span>
        {children}
      </div>
    </>
  )
}

const DiffWord = ({ children, className }: { children: React.ReactNode; className?: string }) => {
  return <span className={cn('inline-block', className)}>{children}</span>
}
