import { useEffect, useState } from 'react'
import { ToastState, type ToastType } from '.'
import { Toast } from './Toast'

export const Toaster = () => {
  const [toasts, setToasts] = useState<ToastType[]>([])

  useEffect(
    () =>
      ToastState.subscribe((toast) => {
        // Delete toast
        if (typeof toast === 'string') {
          return setToasts((toasts) => toasts.filter((t) => t.id !== toast))
        }
        // Add toast
        setToasts((toasts) => [...toasts, toast])
      }),
    [],
  )

  return (
    <div className="fixed bottom-14 right-4 pointer-events-none flex flex-col gap-2">
      {toasts.map((toast) => (
        <Toast key={toast.id} {...toast} />
      ))}
    </div>
  )
}
