import { Tooltip, TooltipTrigger } from '@radix-ui/react-tooltip'
import { cn } from '../../utils/cn'
import { TooltipContent } from '../Tooltip'

type ButtonProps = {
  tooltip: React.ReactNode
} & React.ComponentProps<'button'>

export const IconButton = ({ type = 'button', className, tooltip, ...props }: ButtonProps) => {
  return (
    <Tooltip>
      <TooltipTrigger
        type={type}
        className={cn(
          'h-8 w-8 rounded flex items-center justify-center cursor-default hover:enabled:bg-foreground/10 active:enabled:bg-foreground/20 disabled:opacity-50 [&_svg]:h-[18px] [&_svg]:w-[18px]',
          className,
        )}
        {...props}
      />
      <TooltipContent>{tooltip}</TooltipContent>
    </Tooltip>
  )
}
