import { useAtom, useSetAtom } from 'jotai'
import { useEffect } from 'react'
import { commands } from '../../bindings'
import { useOpenRepository } from '../../data/useOpenRepository'
import { selectedCommitHashAtom, selectedFileIdAtom } from '../../ui-state'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { CommitRow } from './CommitRow'

export const Graph = () => {
  const openRepository = useOpenRepository()

  const { data: commits } = useCommandQuery({
    queryKey: ['graph', openRepository?.id],
    queryFn: commands.getGraph,
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
      <div className="bg-[linear-gradient(color-mix(in_srgb,_var(--color-foreground)_5%,_transparent)_50%,transparent_50%)] [background-size:100%_3.5rem]">
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
