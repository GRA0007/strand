import { TooltipContent as Content, TooltipPortal } from '@radix-ui/react-tooltip'
import { forwardRef } from 'react'
import { cn } from '../../utils/cn'

export const TooltipContent = forwardRef<
  React.ElementRef<typeof Content>,
  React.ComponentPropsWithoutRef<typeof Content>
>(({ className, sideOffset = 4, collisionPadding = 4, ...props }, ref) => (
  <TooltipPortal>
    <Content
      ref={ref}
      sideOffset={sideOffset}
      collisionPadding={collisionPadding}
      className={cn(
        'z-50 overflow-hidden border border-foreground/10 rounded-md bg-surface px-2 py-1 text-xs text-foreground/90 shadow-md animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2',
        className,
      )}
      {...props}
    />
  </TooltipPortal>
))
