import {
  SelectContent as Content,
  SelectItem as Item,
  ItemIndicator,
  ItemText,
  SelectIcon,
  SelectPortal,
  SelectTrigger as Trigger,
} from '@radix-ui/react-select'
import { CheckIcon, ChevronDownIcon } from 'lucide-react'
import { forwardRef } from 'react'
import { cn } from '../../utils/cn'

export const SelectTrigger = forwardRef<
  React.ElementRef<typeof Trigger>,
  React.ComponentPropsWithoutRef<typeof Trigger>
>(({ className, children, ...props }, ref) => {
  return (
    <Trigger
      ref={ref}
      className={cn(
        'flex gap-2 items-center justify-between rounded-md border border-foreground/10 hover:bg-foreground/10 px-2 py-1 text-sm placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1',
        className,
      )}
      {...props}
    >
      {children}
      <SelectIcon asChild>
        <ChevronDownIcon className="h-3 w-3 opacity-50" />
      </SelectIcon>
    </Trigger>
  )
})

export const SelectContent = forwardRef<
  React.ElementRef<typeof Content>,
  React.ComponentPropsWithoutRef<typeof Content>
>(({ className, position = 'popper', ...props }, ref) => {
  return (
    <SelectPortal>
      <Content
        ref={ref}
        className={cn(
          'relative z-50 max-h-96 py-1 min-w-[var(--radix-select-trigger-width)] overflow-hidden rounded-md border border-foreground/10 bg-surface shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2',
          position === 'popper' &&
            'data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1',
          className,
        )}
        position={position}
        {...props}
      />
    </SelectPortal>
  )
})

export const SelectItem = forwardRef<React.ElementRef<typeof Item>, React.ComponentPropsWithoutRef<typeof Item>>(
  ({ className, children, ...props }, ref) => {
    return (
      <Item
        ref={ref}
        className={cn(
          'relative flex w-full cursor-default select-none items-center rounded-xs py-1.5 pl-8 pr-2 text-sm focus:bg-foreground/10 outline-hidden data-disabled:pointer-events-none data-disabled:opacity-50',
          className,
        )}
        {...props}
      >
        <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
          <ItemIndicator>
            <CheckIcon className="h-4 w-4" />
          </ItemIndicator>
        </span>

        <ItemText>{children}</ItemText>
      </Item>
    )
  },
)
