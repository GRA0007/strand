import { type UseMutationOptions, type UseMutationResult, useMutation } from '@tanstack/react-query'
import type { CommandError, Result } from '../bindings'

export const useCommandMutation = <TData = unknown, TError = CommandError, TVariables = void, TContext = unknown>({
  mutationFn,
  ...options
}: Omit<UseMutationOptions<TData, TError, TVariables, TContext>, 'mutationFn'> & {
  mutationFn: (v: TVariables) => Promise<Result<TData, CommandError>>
}): UseMutationResult<TData, TError, TVariables, TContext> => {
  return useMutation({
    ...options,
    mutationFn: (v) =>
      mutationFn(v).then((res) => {
        if (res.status === 'error') {
          // TODO: show in a toast or something
          console.error(res.error)
          throw res.error
        }
        return res.data
      }),
  })
}
