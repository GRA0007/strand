import { TooltipProvider } from '@radix-ui/react-tooltip'
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { Toolbar } from './components/Toolbar'

export const App = () => {
  return (
    <TooltipProvider disableHoverableContent delayDuration={300}>
      <Toolbar />
      <PanelGroup direction="horizontal">
        <Panel defaultSize={20}>
          <PanelGroup direction="vertical">
            <Panel className="bg-surface rounded-lg">local</Panel>
            <PanelResizeHandle className="h-4" />
            <Panel className="bg-surface rounded-lg">remote</Panel>
          </PanelGroup>
        </Panel>
        <PanelResizeHandle className="w-4" />
        <Panel className="bg-surface rounded-lg">tree</Panel>
        <PanelResizeHandle className="w-4" />
        <Panel defaultSize={30}>
          <PanelGroup direction="vertical">
            <Panel className="bg-surface rounded-lg" defaultSize={15}>
              commit message
            </Panel>
            <PanelResizeHandle className="h-2" />
            <Panel>changes</Panel>
          </PanelGroup>
        </Panel>
      </PanelGroup>
    </TooltipProvider>
  )
}
