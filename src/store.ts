import { useQuery } from '@tanstack/react-query'
import { Store } from '@tauri-apps/plugin-store'

const store = new Store('store.bin')

export const useStore = <TValue>(key: string, defaultValue: TValue | null = null) => {
  const { data: value } = useQuery({
    queryKey: ['store', key],
    queryFn: () => store.get<TValue>(key) ?? defaultValue,
    refetchOnWindowFocus: false,
  })

  const setValue = async (newValue: TValue) => {
    await store.set(key, newValue)
    await store.save()
  }

  return [value, setValue] as const
}
