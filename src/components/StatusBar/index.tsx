import { Panel, PanelResizeHandle } from 'react-resizable-panels'
import { GitCommandLog } from './GitCommandLog'
import { History } from './History'

export const StatusBar = () => {
  return (
    <>
      <PanelResizeHandle className="h-4 [&:has(+div[data-panel-size='0.0'])]:h-0" />

      <Panel defaultSize={10} collapsible minSize={10} maxSize={10} className="flex gap-4 max-h-8">
        <GitCommandLog />
        <History />
      </Panel>
    </>
  )
}
