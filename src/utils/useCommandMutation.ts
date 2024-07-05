import { type UseMutationOptions, type UseMutationResult, useMutation } from '@tanstack/react-query'
import type { Result } from '../bindings'
import { toast } from '../components/Toaster'

export const useCommandMutation = <TData = unknown, TError = string, TVariables = void, TContext = unknown>({
  mutationFn,
  ...options
}: Omit<UseMutationOptions<TData, TError, TVariables, TContext>, 'mutationFn'> & {
  mutationFn: (v: TVariables) => Promise<Result<TData, string>>
}): UseMutationResult<TData, TError, TVariables, TContext> => {
  return useMutation({
    ...options,
    mutationFn: (v) =>
      mutationFn(v).then((res) => {
        if (res.status === 'error') {
          console.error(res.error)
          toast({ variant: 'error', title: 'Something went wrong', children: res.error, delay: 20_000 })
          throw res.error
        }
        return res.data
      }),
  })
}
