import { type QueryKey, type UseQueryOptions, type UseQueryResult, useQuery } from '@tanstack/react-query'
import type { CommandError, Result } from '../bindings'

export const useCommandQuery = <
  TQueryFnData = unknown,
  TError = CommandError,
  TData extends TQueryFnData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>({
  queryFn,
  ...options
}: Omit<UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>, 'queryFn'> & {
  // biome-ignore lint/suspicious/noExplicitAny: Allow any params
  queryFn: (...p: any[]) => Promise<Result<TData, CommandError>>
}): UseQueryResult<TData, TError> => {
  return useQuery({
    ...options,
    queryFn: (...p) =>
      queryFn(...p).then((res) => {
        if (res.status === 'error') {
          // TODO: show in a toast or something
          console.error(res.error)
          throw res.error
        }
        return res.data
      }),
  })
}
