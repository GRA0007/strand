import { type UseMutationOptions, type UseMutationResult, useMutation } from '@tanstack/react-query'
import type { CommandError, Result } from '../bindings'
import { toast } from '../components/Toaster'

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
          console.error(res.error)
          toast({ variant: 'error', title: 'Something went wrong', children: errorMessage(res.error), delay: 20_000 })
          throw res.error
        }
        return res.data
      }),
  })
}

const errorMessage = (err: CommandError) => {
  if (err === 'Sqlx') return 'Failed to save/load data'
  if (err === 'Parse') return 'Failed to parse git output'
  if ('Git' in err) return err.Git.toString()
  if ('Other' in err) return err.Other
}
