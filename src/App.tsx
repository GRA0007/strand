import { TooltipProvider } from '@radix-ui/react-tooltip'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useAtomValue } from 'jotai'
import { useRef } from 'react'
import { type ImperativePanelHandle, Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels'
import { Branches } from './components/Branches'
import { CommitDetails } from './components/CommitDetails'
import { FileDiff } from './components/FileDiff'
import { Graph } from './components/Graph'
import { StatusBar } from './components/StatusBar'
import { Toaster } from './components/Toaster/Toaster'
import { Toolbar } from './components/Toolbar'
import { selectedFileIdAtom } from './ui-state'
import { cn } from './utils/cn'

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

  const selectedFileId = useAtomValue(selectedFileIdAtom)

  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider disableHoverableContent delayDuration={300}>
        <Toolbar />
        <PanelGroup direction="vertical">
          <Panel>
            <PanelGroup direction="horizontal">
              <Panel minSize={50} className="relative">
                <PanelGroup
                  direction="horizontal"
                  className={cn(
                    'origin-left transition-all',
                    selectedFileId && 'scale-95 pointer-events-none opacity-75 blur-[1px]',
                  )}
                >
                  <Panel defaultSize={30} ref={leftPanelRef} minSize={15} collapsible>
                    <Branches />
                  </Panel>

                  <PanelResizeHandle
                    className="w-4 [div[data-panel-size='0.0']+&]:w-0"
                    onDoubleClick={() => leftPanelRef.current?.resize(30)}
                  />

                  <Panel className="bg-surface rounded-md" minSize={30}>
                    <Graph />
                  </Panel>
                </PanelGroup>

                <FileDiff />
              </Panel>

              <PanelResizeHandle className="w-4" onDoubleClick={() => rightPanelRef.current?.resize(30)} />

              <Panel defaultSize={30} minSize={10} ref={rightPanelRef}>
                <CommitDetails />
              </Panel>
            </PanelGroup>
          </Panel>

          <StatusBar />
        </PanelGroup>

        <Toaster />
      </TooltipProvider>
    </QueryClientProvider>
  )
}
