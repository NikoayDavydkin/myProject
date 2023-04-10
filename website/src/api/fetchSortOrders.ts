import { AxiosResponse } from 'axios'
import dealTechBaseInstance from './dealTechBaseInstance'

const fetchSortOrders = (): Promise<AxiosResponse> => {
  const query = `query {
          search(category: "dfe0c6a8-3b02-41d5-8eab-5375ba4bc063") {
            sortOrders {
              id
              name
            }
          }
        }`
  return dealTechBaseInstance({
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    data: {
      query
    }
  })
}

export default fetchSortOrders
