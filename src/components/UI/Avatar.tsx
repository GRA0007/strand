import { AvatarFallback, AvatarImage, Avatar as Root } from '@radix-ui/react-avatar'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { Children } from 'react'
import { cn } from '../../utils/cn'
import { TooltipContent } from './Tooltip'

export const Avatar = ({
  emailHash,
  name,
  email,
  size = 40,
  className,
}: { emailHash: string; name: string; email: string; size?: number; className?: string }) => {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Root className={cn('relative flex shrink-0 overflow-hidden rounded-full h-5 w-5 text-[.6rem]', className)}>
          <AvatarImage
            src={`https://gravatar.com/avatar/${emailHash}?s=${size}&d=404`}
            className="aspect-square h-full w-full object-cover"
          />
          <AvatarFallback className="flex h-full w-full items-center justify-center rounded-full bg-surface font-semibold">
            {getInitials(name)}
          </AvatarFallback>
        </Root>
      </TooltipTrigger>

      <TooltipContent>
        {name}
        <span className="text-foreground/60 text-[.6rem] block">{email}</span>
      </TooltipContent>
    </Tooltip>
  )
}

const getInitials = (name: string) => {
  if (name.includes('[bot]')) return '🤖'

  return name
    .split(' ')
    .map((p) => p[0])
    .join('')
    .toLocaleUpperCase()
}

export const AvatarStack = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="flex gap-1 flex-row-reverse justify-end group w-6">
      {Children.toArray(children)
        .reverse()
        .map((child, i) => (
          // biome-ignore lint/suspicious/noArrayIndexKey: avatars won't change order
          <div key={i} className={cn(i > 0 && 'w-1 group-hover:w-6 transition-[width]')}>
            {child}
          </div>
        ))}
    </div>
  )
}
