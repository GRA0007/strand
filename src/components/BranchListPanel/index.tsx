import { FolderIcon, FolderOpenIcon, GitBranchIcon } from 'lucide-react'
import { useMemo } from 'react'
import { Panel } from 'react-resizable-panels'
import { cn } from '../../utils/cn'

type Item = {
  hash: string
  path: string[]
  children: React.ReactNode
}

type BranchListPanelProps = {
  title: string
  icon: React.ReactNode
  actions?: React.ReactNode
  items: Item[]
} & Omit<React.ComponentProps<typeof Panel>, 'children'>

export const BranchListPanel = ({ title, icon, actions, className, items, ...props }: BranchListPanelProps) => {
  const tree = useMemo(() => itemsToTree(items), [items])

  return (
    <Panel className={cn('bg-surface rounded-md flex flex-col', className)} {...props}>
      <header className="border-foreground/20 border-b flex items-center h-8 pl-2 pr-1 gap-1 text-foreground/70">
        {icon}
        <h2 className="text-sm font-semibold mr-auto ml-0.5">{title}</h2>

        {actions}
      </header>

      <div className="text-sm overflow-y-auto flex-1">
        <Items tree={tree} level={0} />
      </div>
    </Panel>
  )
}

const Items = ({ tree, level }: { tree: Tree; level: number }) => {
  return Object.entries(tree).map(([name, item]) =>
    isItem(item) ? (
      <BranchItem key={item.path.join('/')} item={item} level={level} />
    ) : (
      <FolderItem key={name} name={name} tree={item} level={level} />
    ),
  )
}

const isItem = (item: Item | Tree): item is Item => Array.isArray(item.path)

const FolderItem = ({ name, tree, level }: { name: string; tree: Tree; level: number }) => {
  return (
    <details open>
      <summary
        className="flex items-center gap-1.5 hover:bg-foreground/10 active:bg-foreground/20 h-6 select-none cursor-default list-none [&::-webkit-details-marker]:hidden pr-1.5"
        style={{ paddingLeft: `calc(var(--spacing-1_5) + calc(var(--spacing-4) * ${level}))` }}
      >
        <FolderIcon className="h-4 w-4 opacity-60 [details[open]>summary>&]:hidden shrink-0" />
        <FolderOpenIcon className="h-4 w-4 opacity-60 hidden [details[open]>summary>&]:block shrink-0" /> {name}
      </summary>
      <Items tree={tree} level={level + 1} />
    </details>
  )
}

const BranchItem = ({ item, level }: { item: Item; level: number }) => {
  return (
    <div
      className="flex items-center gap-1.5 hover:bg-foreground/10 active:bg-foreground/20 h-6 select-none cursor-default pr-1.5 text-ellipsis overflow-hidden whitespace-nowrap"
      style={{ paddingLeft: `calc(var(--spacing-1_5) + calc(var(--spacing-4) * ${level}))` }}
    >
      <GitBranchIcon className="h-4 w-4 opacity-60 shrink-0" />
      {item.children}
    </div>
  )
}

type Tree = {
  [key in string]: Tree | Item
}

// TODO: Migrate this processing to rust
const itemsToTree = (items: Item[]): Tree => {
  // biome-ignore lint/suspicious/noExplicitAny: Allow nested object
  const setNestedProperty = (obj: Record<string, any>, path: string[], value: unknown) => {
    const [head, ...rest] = path

    if (!(head in obj)) obj[head] = {}

    if (rest.length) setNestedProperty(obj[head], rest, value)
    else obj[head] = value
  }

  const tree = {}
  for (const item of items) {
    setNestedProperty(tree, item.path, item)
  }
  return tree
}
