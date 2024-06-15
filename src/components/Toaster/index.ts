export type ToastType = {
  variant: 'info' | 'warning' | 'error' | 'success'
  title: string
  children?: React.ReactNode
  action?: { label: string; onClick: () => void }
  /**
   * Set to 0 to disable auto-dismiss
   * @default 5000 // (5 seconds)
   */
  delay?: number

  id: string
  onClose: () => void
}

type Subscriber = (toast: ToastType | string) => void

class ToastObserver {
  subscribers: Subscriber[]

  constructor() {
    this.subscribers = []
  }

  subscribe = (subscriber: Subscriber) => {
    this.subscribers.push(subscriber)

    return () => {
      // Clean up subscriber
      this.subscribers.splice(this.subscribers.indexOf(subscriber), 1)
    }
  }

  publish = (data: ToastType | string) => {
    for (const subscriber of this.subscribers) subscriber(data)
  }

  addToast = (toast: Omit<ToastType, 'id' | 'onClose'>) => {
    const id = `${new Date().valueOf().toString()}-${Math.floor(Math.random() * 100)}`
    const data = {
      id,
      onClose: () => {
        this.publish(id)
      },
      ...toast,
    }
    this.publish(data)
  }
}

export const ToastState = new ToastObserver()

export const toast = ToastState.addToast
