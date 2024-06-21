import type { Commit } from '../../bindings'
import { cn } from '../../utils/cn'
import { Avatar, AvatarStack } from '../UI/Avatar'

export const CommitRow = ({
  commit,
  isSelected,
  onSelect,
}: { commit: Commit; isSelected: boolean; onSelect: () => void }) => {
  return (
    <div key={commit.hash} className="h-7 pl-2">
      <div
        className="flex-1 flex items-center group/commit-row h-full w-full outline-none"
        onPointerDown={() => onSelect()}
        id={commit.hash}
        tabIndex={-1}
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
      </div>
    </div>
  )
}
