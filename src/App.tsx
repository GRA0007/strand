import { TooltipProvider } from '@radix-ui/react-tooltip'
import { Toolbar } from './components/Toolbar'

export const App = () => {
  return (
    <TooltipProvider disableHoverableContent delayDuration={300}>
      <Toolbar />
      <p>Hello there</p>
    </TooltipProvider>
  )
}
