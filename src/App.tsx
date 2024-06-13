import { TooltipProvider } from '@radix-ui/react-tooltip'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useRef } from 'react'
import { type ImperativePanelHandle, Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { Branches } from './components/Branches'
import { StatusBar } from './components/StatusBar'
import { Toolbar } from './components/Toolbar'
import { Toast } from './components/UI/Toast'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: false,
    },
  },
})

export const App = () => {
  const leftPanelRef = useRef<ImperativePanelHandle>(null)
  const rightPanelRef = useRef<ImperativePanelHandle>(null)

  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider disableHoverableContent delayDuration={300}>
        <Toolbar />
        <PanelGroup direction="vertical">
          <Panel>
            <PanelGroup direction="horizontal">
              <Panel defaultSize={20} ref={leftPanelRef} minSize={10} collapsible>
                <Branches />
              </Panel>

              <PanelResizeHandle
                className="w-4 [div[data-panel-size='0.0']+&]:w-0"
                onDoubleClick={() => leftPanelRef.current?.resize(20)}
              />

              <Panel className="bg-surface rounded-md" minSize={30}>
                tree (todo)
                <Toast
                  variant="info"
                  title="Version 0.1.1 is ready to install"
                  action={{ label: 'Update', onClick: console.log }}
                />
                <Toast variant="error" title="Failed to open repository">
                  <code>cool-folder</code> is not a git repository
                </Toast>
                <Toast variant="warning" title="Large repository detected">
                  You may want to consider a sparse checkout
                </Toast>
                <Toast variant="success" title="Branch created" />
              </Panel>

              <PanelResizeHandle className="w-4" onDoubleClick={() => rightPanelRef.current?.resize(30)} />

              <Panel defaultSize={30} minSize={10} ref={rightPanelRef}>
                <PanelGroup direction="vertical">
                  <Panel className="bg-surface rounded-md rounded-b-none" defaultSize={15}>
                    <div className="overflow-y-auto h-full px-3 py-2">
                      <div>commit message (todo)</div>
                      <div className="text-xs pt-2">commit description</div>
                    </div>
                  </Panel>

                  <PanelResizeHandle className="h-4 bg-surface rounded-b-md flex items-center justify-center group">
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
          </Panel>

          <StatusBar />
        </PanelGroup>
      </TooltipProvider>
    </QueryClientProvider>
  )
}
