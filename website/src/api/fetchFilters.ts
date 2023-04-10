import { AxiosResponse } from 'axios'
import dealTechBaseInstance from './dealTechBaseInstance'

const fetchFilters = (): Promise<AxiosResponse> => {
  const query = `query {
          search(category: "dfe0c6a8-3b02-41d5-8eab-5375ba4bc063") {
            filters {
              attribute
              name
              filterType
              attributeValues
              units
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

export default fetchFilters
