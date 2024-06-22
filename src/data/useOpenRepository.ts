import { commands } from '../bindings'
import { useCommandQuery } from '../utils/useCommandQuery'

export const useOpenRepository = () => {
  const { data: openRepository } = useCommandQuery({
    queryKey: ['openRepository'],
    queryFn: commands.getOpenRepository,
  })

  return openRepository
}
