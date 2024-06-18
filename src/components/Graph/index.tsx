import { useAtom } from 'jotai'
import { type Commit, commands } from '../../bindings'
import { selectedCommitHashAtom } from '../../ui-state'
import { cn } from '../../utils/cn'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { Avatar, AvatarStack } from '../UI/Avatar'

export const Graph = () => {
  const { data: openRepository } = useCommandQuery({
    queryKey: ['openRepository'],
    queryFn: commands.getOpenRepository,
  })

  const { data: commits } = useCommandQuery({
    queryKey: ['graph'],
    queryFn: commands.getGraph,
    enabled: Boolean(openRepository),
    refetchOnWindowFocus: true,
  })

  const [selectedHash, setSelectedHash] = useAtom(selectedCommitHashAtom)

  return (
    <div className="overflow-y-auto h-full">
      <div className="bg-[linear-gradient(color-mix(in_srgb,_var(--color-foreground)_5%,_transparent)_50%,transparent_50%)] [background-size:100%_3.5rem]">
        {commits?.map((commit) => (
          <CommitRow
            key={commit.hash}
            commit={commit}
            isSelected={selectedHash === commit.hash}
            onSelect={() => setSelectedHash(commit.hash)}
            onKeyUp={() => {
              const currentIndex = commits.findIndex((c) => c.hash === commit.hash)
              if (currentIndex === 0 || !selectedHash) return
              setSelectedHash(commits[currentIndex - 1].hash)
              document.getElementById(commits[currentIndex - 1].hash)?.focus()
            }}
            onKeyDown={() => {
              const currentIndex = commits.findIndex((c) => c.hash === commit.hash)
              if (currentIndex === commits.length - 1 || !selectedHash) return
              setSelectedHash(commits[currentIndex + 1].hash)
              document.getElementById(commits[currentIndex + 1].hash)?.focus()
            }}
          />
        ))}
      </div>
    </div>
  )
}

const CommitRow = ({
  commit,
  isSelected,
  onSelect,
  onKeyUp,
  onKeyDown,
}: { commit: Commit; isSelected: boolean; onSelect: () => void; onKeyUp: () => void; onKeyDown: () => void }) => {
  return (
    <div key={commit.hash} className="h-7 pl-2">
      <button
        type="button"
        className="flex-1 flex items-center group/commit-row h-full w-full outline-none"
        onPointerDown={() => onSelect()}
        onKeyDown={(e) => {
          if (e.code === 'ArrowDown') {
            e.preventDefault()
            onKeyDown()
          }
          if (e.code === 'ArrowUp') {
            e.preventDefault()
            onKeyUp()
          }
        }}
        tabIndex={0}
        id={commit.hash}
      >
        <div
          className={cn(
            'flex items-center h-6 rounded-l-full flex-1',
            isSelected ? 'bg-orange-900' : 'group-hover/commit-row:bg-orange-900/20',
          )}
        >
          <AvatarStack className="h-6 w-6 border-2 border-orange-700">
            <Avatar emailHash={commit.author.email_hash} name={commit.author.name} email={commit.author.email} />

            {commit.author.email !== commit.committer.email && (
              <Avatar
                emailHash={commit.committer.email_hash}
                name={commit.committer.name}
                email={commit.committer.email}
              />
            )}
          </AvatarStack>

          <div className="whitespace-nowrap text-xs text-ellipsis overflow-hidden pl-3">
            <span className={cn(isSelected && 'font-semibold')}>{commit.message}</span>
            {commit.description && <span className="text-foreground/50 ml-2">{commit.description}</span>}
          </div>
        </div>
      </button>
    </div>
  )
}
