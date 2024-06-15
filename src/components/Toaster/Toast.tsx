import { CircleAlertIcon, CircleCheckIcon, InfoIcon, TriangleAlertIcon, XIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import type { ToastType } from '.'
import { useIsDocumentHidden } from '../../utils/useIsDocumentHidden'

export const Toast = ({ variant, title, children, action, onClose, delay = 5_000 }: Omit<ToastType, 'id'>) => {
  const isDocumentHidden = useIsDocumentHidden()
  const [isMouseOver, setIsMouseOver] = useState(false)

  useEffect(() => {
    if (delay === 0) return
    let timeoutId: ReturnType<typeof setTimeout>

    const startTimer = () => {
      timeoutId = setTimeout(() => {
        onClose()
      }, delay)
    }

    // Reset timer if hovered or document is no longer focused
    if (!isMouseOver && !isDocumentHidden) startTimer()

    return () => clearTimeout(timeoutId)
  }, [isDocumentHidden, onClose, delay, isMouseOver])

  return (
    <div
      className="bg-surface border border-foreground/10 p-2 rounded-md flex gap-2 items-center text-sm w-[300px] shadow-lg relative group pointer-events-auto"
      onMouseEnter={() => setIsMouseOver(true)}
      onMouseLeave={() => setIsMouseOver(false)}
    >
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
        onClick={() => onClose()}
      >
        <XIcon className="h-3 w-3" />
      </button>
    </div>
  )
}
