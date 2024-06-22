import { useAtomValue } from 'jotai'
import { LoaderCircleIcon } from 'lucide-react'
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { type CommitUser, commands } from '../../bindings'
import { useOpenRepository } from '../../data/useOpenRepository'
import { selectedCommitHashAtom } from '../../ui-state'
import { formatDate } from '../../utils/formatDate'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { Avatar } from '../UI/Avatar'
import { CommitFile } from './File'
import { CommitStats } from './Stats'

export const CommitDetails = () => {
  const openRepository = useOpenRepository()
  const selectedHash = useAtomValue(selectedCommitHashAtom)

  const { data: commits } = useCommandQuery({
    queryKey: ['graph', openRepository?.id],
    queryFn: commands.getGraph,
    enabled: Boolean(openRepository),
  })

  const { data: files } = useCommandQuery({
    queryKey: ['graph', openRepository?.id, selectedHash],
    queryFn: () => commands.getCommitFiles(selectedHash as string),
    enabled: Boolean(openRepository && selectedHash),
  })

  const selectedCommit = commits?.find((c) => c.hash === selectedHash)

  return (
    <PanelGroup direction="vertical">
      {!selectedCommit && (
        <div className="h-full w-full flex items-center justify-center">
          <LoaderCircleIcon className="animate-spin h-4 w-4" />
        </div>
      )}
      {selectedCommit && (
        <>
          <Panel className="bg-surface rounded-md rounded-b-none" defaultSize={15}>
            <div className="overflow-y-auto h-full px-3 py-2">
              <div>{selectedCommit.message}</div>
              {selectedCommit.description && <div className="text-xs pt-2">{selectedCommit.description}</div>}
            </div>
          </Panel>

          <PanelResizeHandle className="h-4 bg-surface rounded-b-md flex items-center justify-center group">
            <div className="bg-foreground/20 h-1 w-[40%] rounded-full group-hover:bg-foreground/30 group-active:bg-foreground/40" />
          </PanelResizeHandle>

          <Panel className="flex flex-col">
            <div className="text-xs mt-2">
              <span className="text-foreground/60">Authored</span> {formatDate(selectedCommit.author.date)}
            </div>
            <User user={selectedCommit.author} />

            {selectedCommit.author.email !== selectedCommit.committer.email && (
              <>
                <div className="text-xs mt-2">
                  <span className="text-foreground/60">Committed</span> {formatDate(selectedCommit.committer.date)}
                </div>
                <User user={selectedCommit.committer} />
              </>
            )}

            {files && (
              <>
                <CommitStats files={files} />

                <div className="bg-surface rounded-md flex-1 flex flex-col min-h-0">
                  <div className="overflow-y-auto flex-1 pt-1">
                    {files.map((file) => (
                      <CommitFile key={`${file.src_hash ?? 0}_${file.dst_hash ?? 0}`} file={file} />
                    ))}
                  </div>

                  <div className="border-t border-foreground/20 px-3 py-2 text-xs flex gap-4 items-center">
                    <div>
                      <span className="text-foreground/60">Commit</span> {selectedCommit.hash.slice(0, 6)}
                    </div>
                    <div>
                      <span className="text-foreground/60">Parents</span>{' '}
                      {selectedCommit.parent_hashes.map((p) => p.slice(0, 6)).join(', ') || 'none'}
                    </div>
                  </div>
                </div>
              </>
            )}
          </Panel>
        </>
      )}
    </PanelGroup>
  )
}

const User = ({ user }: { user: CommitUser }) => {
  return (
    <div className="mt-1 flex gap-2 items-center">
      <Avatar name={user.name} emailHash={user.email_hash} className="h-8 w-8 text-sm" size={64} />
      <div className="leading-tight">
        <span className="text-sm">{user.name}</span>
        <span className="block text-xs text-foreground/60">{user.email}</span>
      </div>
    </div>
  )
}
