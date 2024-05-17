import { TooltipProvider } from '@radix-ui/react-tooltip'
import { CircleDashedIcon, CircleIcon, PaintbrushIcon, PlusIcon } from 'lucide-react'
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { BranchListPanel } from './components/BranchListPanel'
import { IconButton } from './components/IconButton'
import { Toolbar } from './components/Toolbar'

export const App = () => {
  return (
    <TooltipProvider disableHoverableContent delayDuration={300}>
      <Toolbar />
      <PanelGroup direction="horizontal">
        <Panel defaultSize={20}>
          <PanelGroup direction="vertical">
            <BranchListPanel
              icon={<CircleIcon className="h-4 w-4" />}
              title="Local"
              actions={
                <IconButton tooltip="Clean merged branches" size="sm">
                  <PaintbrushIcon />
                </IconButton>
              }
            >
              local branches
            </BranchListPanel>

            <PanelResizeHandle className="h-4" />

            <BranchListPanel
              icon={<CircleDashedIcon className="h-4 w-4" />}
              title="Remote"
              actions={
                <IconButton tooltip="Add remote" size="sm">
                  <PlusIcon />
                </IconButton>
              }
            >
              remote branches
            </BranchListPanel>
          </PanelGroup>
        </Panel>

        <PanelResizeHandle className="w-4" />

        <Panel className="bg-surface rounded-lg">tree</Panel>

        <PanelResizeHandle className="w-4" />

        <Panel defaultSize={30}>
          <PanelGroup direction="vertical">
            <Panel className="bg-surface rounded-lg rounded-b-none" defaultSize={15}>
              <div className="overflow-y-auto h-full px-3 py-2">
                <div>commit message</div>
                <div className="text-xs pt-2">commit description</div>
              </div>
            </Panel>

            <PanelResizeHandle className="h-4 bg-surface rounded-b-lg flex items-center justify-center group">
              <div className="bg-foreground/20 h-1 w-[40%] rounded-full group-hover:bg-foreground/30 group-active:bg-foreground/40" />
            </PanelResizeHandle>

            <Panel>
              <div className="text-xs mt-2">
                <span className="text-foreground/60">Authored</span> 4 July 2024 at 8:47 PM
              </div>
            </Panel>
          </PanelGroup>
        </Panel>
      </PanelGroup>
    </TooltipProvider>
  )
}
