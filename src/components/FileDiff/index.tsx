import { useAtom, useAtomValue } from 'jotai'
import { LoaderCircleIcon, XIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import { type ShikiTransformer, codeToHtml } from 'shiki'
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

  const [html, setHtml] = useState('')
  useEffect(() => {
    if (!selectedFile) return
    const pathParts = selectedFile.src_path.split('.')
    codeToHtml(diff ?? '', {
      lang: pathParts[pathParts.length - 1],
      theme: 'github-dark-default',
      transformers: [diffTransformer],
    }).then(setHtml)
  }, [diff])

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

      {diff && <div className="flex-1 overflow-auto py-2 text-sm" dangerouslySetInnerHTML={{ __html: html }} />}
    </div>
  )
}

const diffTransformer: ShikiTransformer = {
  line(node) {
    let firstTextElement = node.children[0]
    while (firstTextElement.type !== 'text' && firstTextElement.type === 'element') {
      firstTextElement = firstTextElement.children[0]
    }
    const status = firstTextElement.value[0]
    if (status === '@') return this.addClassToHast(node, 'hunk')

    firstTextElement.value = firstTextElement.value.slice(1)
    if (status === '+') return this.addClassToHast(node, 'added')
    if (status === '-') return this.addClassToHast(node, 'removed')
  },
}
