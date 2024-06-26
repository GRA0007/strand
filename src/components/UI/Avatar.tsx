import { AvatarFallback, AvatarImage, Avatar as Root } from '@radix-ui/react-avatar'
import { Slot, Slottable } from '@radix-ui/react-slot'
import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { Children } from 'react'
import { cn } from '../../utils/cn'
import { TooltipContent } from './Tooltip'

export const Avatar = ({
  emailHash,
  name,
  size = 40,
  className,
  ...props
}: {
  emailHash: string
  name: string
  /** The size of the image to load (should be at least 2x the pixel size) */
  size?: number
  className?: string
} & (
  | {
      tooltip?: false
    }
  | {
      tooltip: true
      email: string
    }
)) => {
  const avatar = (
    <Root className={cn('relative flex shrink-0 overflow-hidden rounded-full h-6 w-6 text-[.6rem]', className)}>
      <AvatarImage
        src={`https://gravatar.com/avatar/${emailHash}?s=${size}&d=404`}
        className="aspect-square h-full w-full object-cover"
      />
      <AvatarFallback className="flex h-full w-full items-center justify-center rounded-full bg-surface font-semibold">
        {getInitials(name)}
      </AvatarFallback>
    </Root>
  )

  if (!props.tooltip) return avatar

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Slottable>{avatar}</Slottable>
      </TooltipTrigger>

      <TooltipContent>
        {name}
        <span className="text-foreground/60 text-[.6rem] block">{props.email}</span>
      </TooltipContent>
    </Tooltip>
  )
}

/** Returns the first 2 initials of the user's name */
const getInitials = (name: string) => {
  if (name.includes('[bot]')) return '🤖'

  return name
    .split(' ')
    .map((p) => Array.from(p)[0])
    .slice(0, 2)
    .join('')
    .toLocaleUpperCase()
}

export const AvatarStack = ({ children, className }: { children: React.ReactNode; className?: string }) => {
  const avatars = getStackAvatars(children)
  return (
    <div className={cn('h-6 w-6', className, 'relative border-none')}>
      <div className="flex group absolute">
        {avatars.map((child, i) => (
          <Slot
            // biome-ignore lint/suspicious/noArrayIndexKey: avatars won't change order
            key={i}
            className={cn(
              className,
              'relative',
              i > 0 && 'group-hover:ml-1 transition-[margin]',
              i === 0 && 'z-[3]',
              i === 1 && 'z-[2] ml-[-1.1rem]',
              i === 2 && 'z-[1] -ml-5',
            )}
          >
            {child}
          </Slot>
        ))}
      </div>
    </div>
  )
}

const getStackAvatars = (children: React.ReactNode) => {
  const avatars = Children.toArray(children)
  return avatars.length > 3 ? [...avatars.slice(0, 2), <MoreAvatars key="more" count={avatars.length - 2} />] : avatars
}

const MoreAvatars = ({ count, className }: { count: number; className?: string }) => {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Slottable>
          <div
            className={cn(
              'h-6 w-6 bg-surface rounded-full text-[.6rem] flex items-center justify-center font-semibold',
              className,
            )}
          >
            +{count}
          </div>
        </Slottable>
      </TooltipTrigger>

      <TooltipContent>+{count} more</TooltipContent>
    </Tooltip>
  )
}
