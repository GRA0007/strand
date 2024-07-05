import { atom } from 'jotai'

export const selectedCommitHashAtom = atom<string | null>(null)

export const selectedFileIdAtom = atom<string | null>(null)

export const calculateFileId = (commitHash: string, path: string) => `${commitHash}-${path}`
