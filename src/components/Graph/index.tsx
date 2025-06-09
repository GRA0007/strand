import { useAtom, useSetAtom } from 'jotai'
import { useEffect } from 'react'
import { commands } from '../../bindings'
import { useOpenRepository } from '../../data/useOpenRepository'
import { selectedCommitHashAtom, selectedFileIdAtom } from '../../ui-state'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { CommitStats } from '../CommitDetails/Stats'
import { CommitRow } from './CommitRow'

export const Graph = () => {
  const openRepository = useOpenRepository()

  const { data: commits } = useCommandQuery({
    queryKey: ['graph', openRepository?.id],
    queryFn: commands.getGraph,
    enabled: Boolean(openRepository),
    refetchOnWindowFocus: true,
  })

  const { data: changes } = useCommandQuery({
    queryKey: ['status'],
    queryFn: commands.getChangedFiles,
    enabled: Boolean(openRepository),
    refetchOnWindowFocus: true,
  })

  const [selectedHash, _setSelectedHash] = useAtom(selectedCommitHashAtom)
  const setSelectedFileId = useSetAtom(selectedFileIdAtom)
  const setSelectedHash = (hash: string | null) => {
    _setSelectedHash((h) => {
      if (h !== hash) {
        setSelectedFileId(null)
      }
      return hash
    })
  }

  // Select the most recent commit if none selected
  useEffect(() => {
    if (commits && selectedHash === null) {
      setSelectedHash(commits[0].hash)
    }
  }, [commits, selectedHash, setSelectedHash])

  // TODO: Move this somewhere more central
  const handleKeyDown = (e: KeyboardEvent) => {
    if (
      (e.target instanceof HTMLElement &&
        (e.target.tagName === 'INPUT' ||
          e.target.tagName === 'TEXTAREA' ||
          e.target.tagName === 'BUTTON' ||
          e.target.role === 'combobox' ||
          e.target.role === 'menuitem' ||
          e.target.role === 'option')) ||
      !commits ||
      !selectedHash
    )
      return

    if (e.code === 'ArrowDown') {
      e.preventDefault()
      const currentIndex = commits.findIndex((c) => c.hash === selectedHash)
      if (currentIndex === commits.length - 1) return
      setSelectedHash(commits[currentIndex + 1].hash)
      document.getElementById(commits[currentIndex + 1].hash)?.focus()
    }
    if (e.code === 'ArrowUp') {
      e.preventDefault()
      const currentIndex = commits.findIndex((c) => c.hash === selectedHash)
      if (currentIndex === 0) return
      setSelectedHash(commits[currentIndex - 1].hash)
      document.getElementById(commits[currentIndex - 1].hash)?.focus()
    }
  }

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [handleKeyDown])

  return (
    <div className="overflow-y-auto h-full">
      <div className="bg-[linear-gradient(transparent_50%,color-mix(in_srgb,rgb(var(--color-foreground))_5%,transparent)_50%)] [background-size:100%_3.5rem]">
        {((changes?.[0].length ?? 0) > 0 || (changes?.[1].length ?? 0) > 0) && (
          <div className="h-7 pl-2">
            <div className="flex items-center w-full h-full outline-hidden group/commit-row" tabIndex={-1}>
              <div className="flex items-center h-6 rounded-l-full flex-1 min-w-0 group-hover/commit-row:bg-orange-300/20 dark:group-hover/commit-row:bg-orange-900/20">
                <div className="h-6 w-6 rounded-full border-orange-400 dark:border-orange-700 border-2 border-dashed shrink-0 bg-surface" />

                <div className="ml-3 bg-surface rounded-md h-5 flex items-center px-1">
                  <CommitStats files={changes?.flat() ?? []} />
                </div>
              </div>
            </div>
          </div>
        )}

        {commits?.map((commit) => (
          <CommitRow
            key={commit.hash}
            commit={commit}
            isSelected={selectedHash === commit.hash}
            onSelect={() => setSelectedHash(commit.hash)}
          />
        ))}
      </div>
    </div>
  )
}
