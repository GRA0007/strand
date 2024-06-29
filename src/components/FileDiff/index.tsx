import { useAtom, useAtomValue } from 'jotai'
import { LoaderCircleIcon, XIcon } from 'lucide-react'
import { Fragment, useEffect, useState } from 'react'
import { commands } from '../../bindings'
import { useOpenRepository } from '../../data/useOpenRepository'
import { calculateFileId, selectedCommitHashAtom, selectedFileIdAtom } from '../../ui-state'
import { cn } from '../../utils/cn'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { FileName } from '../FileName'
import { IconButton } from '../UI/IconButton'

export const FileDiff = () => {
  const openRepository = useOpenRepository()
  const selectedCommitHash = useAtomValue(selectedCommitHashAtom)
  const [selectedFileId, setSelectedFileId] = useAtom(selectedFileIdAtom)

  const { data: files } = useCommandQuery({
    queryKey: ['graph', openRepository?.id, selectedCommitHash],
    queryFn: () => commands.getCommitFiles(selectedCommitHash as string),
    enabled: Boolean(openRepository && selectedCommitHash),
  })

  // Keep file loaded on the page after deselected while animating away
  const [cachedFileId, setCachedFileId] = useState(selectedFileId)
  useEffect(() => {
    if (selectedFileId) setCachedFileId(selectedFileId)
  }, [selectedFileId])

  const selectedFile = selectedCommitHash
    ? files?.find((f) => calculateFileId(selectedCommitHash, f.src_path) === cachedFileId)
    : undefined

  const { data: diff } = useCommandQuery({
    queryKey: ['diff', selectedCommitHash, selectedFile?.src_path],
    queryFn: () => commands.getFileDiff(selectedCommitHash as string, selectedFile?.src_path as string),
    enabled: Boolean(openRepository && selectedCommitHash && selectedFile),
  })

  return (
    <div
      className={cn(
        'pointer-events-none invisible opacity-0 transition-all translate-x-4 absolute inset-0 left-4 bg-surface rounded-md flex flex-col',
        selectedFileId && 'opacity-100 visible pointer-events-auto translate-x-0',
      )}
    >
      {selectedFile && (
        <div className="pl-3 pr-1 h-10 border-b border-foreground/20 flex items-center shrink-0">
          <FileName
            path={selectedFile.dst_path ?? selectedFile.src_path}
            status={selectedFile.status}
            tooltipProps={{ align: 'start' }}
          />

          <IconButton tooltip="Close" onClick={() => setSelectedFileId(null)}>
            <XIcon />
          </IconButton>
        </div>
      )}

      {(!selectedFile || !diff) && (
        <div className="flex-1 flex items-center justify-center">
          <LoaderCircleIcon className="h-4 w-4 animate-spin" />
        </div>
      )}

      {diff && (
        <pre className="flex-1 overflow-auto py-2 text-sm">
          <code className="w-max min-w-full block">
            {diff.map((hunk) => (
              <Fragment key={hunk.header}>
                <div className="bg-info/30 py-1 px-3 text-foreground/70">{hunk.header}</div>
                {hunk.lines.map((line, i) => (
                  <Fragment key={`${hunk.header}-${i}`}>
                    {line.some((w) => w.status !== 'Added') && (
                      <div className={cn('px-3', line.some((w) => w.status === 'Removed') && 'bg-error/20')}>
                        {line.some((w) => w.status === 'Removed') ? '- ' : '  '}
                        {line
                          .filter((w) => w.status !== 'Added')
                          .map((word, j) => (
                            <span
                              key={`${hunk.header}-${i}-${j}`}
                              className={cn(
                                'inline-block',
                                word.status === 'Removed' &&
                                  line.filter((l) => l.text.trim().length > 0).some((l) => l.status === 'Unmodified') &&
                                  'bg-error/20',
                              )}
                            >
                              {word.text}
                            </span>
                          ))}
                      </div>
                    )}
                    {line.some((w) => w.status === 'Added') && (
                      <div className="px-3 bg-success/20">
                        +{' '}
                        {line
                          .filter((w) => w.status !== 'Removed')
                          .map((word, j) => (
                            <span
                              key={`${hunk.header}-${i}-${j}`}
                              className={cn(
                                'inline-block',
                                word.status === 'Added' &&
                                  line.filter((l) => l.text.trim().length > 0).some((l) => l.status === 'Unmodified') &&
                                  'bg-success/20',
                              )}
                            >
                              {word.text}
                            </span>
                          ))}
                      </div>
                    )}
                  </Fragment>
                ))}
              </Fragment>
            ))}
          </code>
        </pre>
      )}
    </div>
  )
}
