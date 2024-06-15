import { format } from 'date-fns'

export const formatDate = (value: string, dateFormat = "d MMMM yyyy 'at' p") => {
  const date = new Date(`${value}Z`)
  return format(date, dateFormat)
}
