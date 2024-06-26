import { atom } from 'jotai'

export const selectedCommitHashAtom = atom<string | null>(null)

export const selectedFileIdAtom = atom<string | null>(null)

export const calculateFileId = (srcHash: string | null, dstHash: string | null) => `${srcHash ?? '0'}-${dstHash ?? '0'}`
