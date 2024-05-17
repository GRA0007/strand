import { twMerge } from 'tailwind-merge'

/** Merge tailwind classes */
export const cn = (...classes: (string | false | null | undefined)[]) => twMerge(classes.filter(Boolean))
