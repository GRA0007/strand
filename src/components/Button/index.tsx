import { cn } from '../../utils/cn'

type ButtonProps = React.ComponentProps<'button'>

export const Button = ({ type = 'button', className, ...props }: ButtonProps) => {
  return (
    <button
      type={type}
      className={cn(
        'h-8 w-8 rounded flex items-center justify-center cursor-default hover:enabled:bg-white/10 active:enabled:bg-white/20 disabled:opacity-50',
        className,
      )}
      {...props}
    />
  )
}
