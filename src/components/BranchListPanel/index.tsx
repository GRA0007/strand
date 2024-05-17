import { Panel } from 'react-resizable-panels'
import { cn } from '../../utils/cn'

type BranchListPanelProps = {
  title: string
  icon: React.ReactNode
  actions?: React.ReactNode
} & React.ComponentProps<typeof Panel>

export const BranchListPanel = ({ title, icon, actions, className, children, ...props }: BranchListPanelProps) => {
  return (
    <Panel className={cn('bg-surface rounded-lg', className)} {...props}>
      <header className="border-foreground/20 border-b flex items-center h-8 pl-2 pr-1 gap-1 text-foreground/70">
        {icon}
        <h2 className="text-sm font-semibold mr-auto ml-0.5">{title}</h2>

        {actions}
      </header>

      <div className="px-2 py-1 text-xs">{children}</div>
    </Panel>
  )
}
