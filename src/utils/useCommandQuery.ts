import { type QueryKey, type UseQueryOptions, type UseQueryResult, useQuery } from '@tanstack/react-query'
import type { Result } from '../bindings'
import { toast } from '../components/Toaster'

export const useCommandQuery = <
  TQueryFnData = unknown,
  TError = string,
  TData extends TQueryFnData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>({
  queryFn,
  ...options
}: Omit<UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>, 'queryFn'> & {
  // biome-ignore lint/suspicious/noExplicitAny: Allow any params
  queryFn: (...p: any[]) => Promise<Result<TData, string>>
}): UseQueryResult<TData, TError> => {
  return useQuery({
    ...options,
    queryFn: (...p) =>
      queryFn(...p).then((res) => {
        if (res.status === 'error') {
          console.error(res.error)
          toast({ variant: 'error', title: 'Something went wrong', children: res.error })
          throw res.error
        }
        return res.data
      }),
  })
}
