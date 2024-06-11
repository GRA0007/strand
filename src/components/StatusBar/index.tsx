import { useEffect, useState } from 'react'
import { Panel, PanelResizeHandle } from 'react-resizable-panels'
import { events } from '../../bindings'

export const StatusBar = () => {
  const [recentCommand, setRecentCommand] = useState('')
  useEffect(() => {
    const unlisten = events.gitCommandEvent.listen(({ payload }) => {
      setRecentCommand(payload)
    })
    return () => {
      unlisten.then((f) => f())
    }
  }, [])

  return (
    <>
      <PanelResizeHandle className="h-4 [&:has(+div[data-panel-size='0.0'])]:h-0" />

      <Panel
        defaultSize={5}
        collapsible
        minSize={5}
        maxSize={5}
        className="bg-surface rounded-lg flex items-center font-mono text-sm px-3"
      >
        {recentCommand}
      </Panel>
    </>
  )
}
