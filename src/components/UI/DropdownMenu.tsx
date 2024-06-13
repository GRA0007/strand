import {
  DropdownMenuContent as Content,
  DropdownMenuPortal,
  DropdownMenuItem as Item,
  DropdownMenuSeparator as Separator,
} from '@radix-ui/react-dropdown-menu'
import { forwardRef } from 'react'
import { cn } from '../../utils/cn'

export const DropdownMenuContent = forwardRef<
  React.ElementRef<typeof Content>,
  React.ComponentPropsWithoutRef<typeof Content>
>(({ className, sideOffset = 4, collisionPadding = 4, ...props }, ref) => (
  <DropdownMenuPortal>
    <Content
      ref={ref}
      sideOffset={sideOffset}
      collisionPadding={collisionPadding}
      className={cn(
        'z-50 min-w-[var(--radix-dropdown-menu-trigger-width)] overflow-auto rounded-md border bg-surface border-foreground/10 py-1 shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2',
        className,
      )}
      {...props}
    />
  </DropdownMenuPortal>
))

export const DropdownMenuItem = forwardRef<React.ElementRef<typeof Item>, React.ComponentPropsWithoutRef<typeof Item>>(
  ({ className, ...props }, ref) => (
    <Item
      ref={ref}
      className={cn(
        'relative flex cursor-default select-none items-center px-2 py-1.5 text-sm outline-none focus:bg-foreground/10 data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
        className,
      )}
      {...props}
    />
  ),
)

export const DropdownMenuSeparator = forwardRef<
  React.ElementRef<typeof Separator>,
  React.ComponentPropsWithoutRef<typeof Separator>
>(({ className, ...props }, ref) => (
  <Separator ref={ref} className={cn('-mx-1 my-1 h-px bg-foreground/10', className)} {...props} />
))
