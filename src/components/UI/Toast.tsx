import { CircleAlertIcon, CircleCheckIcon, InfoIcon, TriangleAlertIcon, XIcon } from 'lucide-react'

export const Toast = ({
  variant,
  title,
  children,
  action,
}: {
  variant: 'info' | 'warning' | 'error' | 'success'
  title: string
  children?: React.ReactNode
  action?: { label: string; onClick: () => void }
}) => {
  return (
    <div className="bg-surface border border-foreground/10 p-2 rounded-md flex gap-2 items-center text-sm w-[300px] shadow-lg relative group">
      <div className="flex flex-col gap-1 flex-1">
        <div className="flex gap-2 leading-tight">
          <div className="shrink-0 self-baseline">
            {variant === 'info' && <InfoIcon className="h-4 w-4 text-info" />}
            {variant === 'warning' && <TriangleAlertIcon className="h-4 w-4 text-warn" />}
            {variant === 'error' && <CircleAlertIcon className="h-4 w-4 text-error" />}
            {variant === 'success' && <CircleCheckIcon className="h-4 w-4 text-success" />}
          </div>

          {title}
        </div>

        {children && <div className="text-xs text-foreground/60">{children}</div>}
      </div>

      {action && (
        <button
          type="button"
          className="bg-foreground hover:bg-foreground/90 active:bg-foreground/80 text-background text-xs font-semibold rounded px-1 py-0.5"
          onClick={action.onClick}
        >
          {action.label}
        </button>
      )}

      <button
        type="button"
        className="absolute -top-1.5 -right-1.5 h-5 w-5 bg-inherit border-inherit border rounded-full flex items-center justify-center shadow opacity-0 invisible pointer-events-none transition-[opacity,visibility] group-hover:visible group-hover:opacity-100 group-hover:pointer-events-auto"
      >
        <XIcon className="h-3 w-3" />
      </button>
    </div>
  )
}
