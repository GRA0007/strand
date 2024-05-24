import { TooltipProvider } from '@radix-ui/react-tooltip'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useRef } from 'react'
import { type ImperativePanelHandle, Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { Branches } from './components/Branches'
import { Toolbar } from './components/Toolbar'

const queryClient = new QueryClient()

export const App = () => {
  const leftPanelRef = useRef<ImperativePanelHandle>(null)
  const rightPanelRef = useRef<ImperativePanelHandle>(null)

  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider disableHoverableContent delayDuration={300}>
        <Toolbar />
        <PanelGroup direction="horizontal">
          <Panel defaultSize={20} ref={leftPanelRef} minSize={10} collapsible>
            <Branches />
          </Panel>

          <PanelResizeHandle
            className="w-4 [div[data-panel-size='0.0']+&]:w-0"
            onDoubleClick={() => leftPanelRef.current?.resize(20)}
          />

          <Panel className="bg-surface rounded-lg" minSize={30}>
            tree
          </Panel>

          <PanelResizeHandle className="w-4" onDoubleClick={() => rightPanelRef.current?.resize(30)} />

          <Panel defaultSize={30} minSize={10} ref={rightPanelRef}>
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
    </QueryClientProvider>
  )
}
