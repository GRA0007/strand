import { commands } from '../../bindings'
import { useCommandQuery } from '../../utils/useCommandQuery'
import { Avatar, AvatarStack } from '../UI/Avatar'

export const Graph = () => {
  const { data: commits } = useCommandQuery({
    queryKey: ['graph'],
    queryFn: commands.getGraph,
  })

  return (
    <div className="overflow-y-auto h-full">
      <div className="bg-[linear-gradient(color-mix(in_srgb,_var(--color-foreground)_5%,_transparent)_50%,transparent_50%)] [background-size:100%_3.5rem]">
        {commits?.map((commit) => (
          <div key={commit.hash} className="h-7 flex items-center pl-2">
            <div className="flex items-center h-6 hover:bg-info/10 rounded-l-full flex-1">
              <AvatarStack>
                <Avatar
                  emailHash={commit.author.email_hash}
                  name={commit.author.name}
                  email={commit.author.email}
                  className="h-6 w-6 border-2 border-info"
                />

                {commit.author.email !== commit.committer.email && (
                  <Avatar
                    emailHash={commit.committer.email_hash}
                    name={commit.committer.name}
                    email={commit.committer.email}
                    className="h-6 w-6 border-2 border-info"
                  />
                )}
              </AvatarStack>

              <div className="whitespace-nowrap text-xs text-ellipsis overflow-hidden pl-3">
                <span>{commit.message}</span>
                {commit.description && <span className="text-foreground/50 ml-2">{commit.description}</span>}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
