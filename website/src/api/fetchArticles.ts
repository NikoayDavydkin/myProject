import dealTechInstance from './dealTechStrapiInstance'
import { AxiosResponse } from 'axios'

const fetchArticles = (): Promise<AxiosResponse> => {
  const query = `query search($category: ID) {
  search(category: $category) {
    content {
      __typename
      ... on Search {
        products {
          id
          title
          summary
          imageUrl
          seller
          created
          updated
          defaultSortOrder
          selectOffer {
            price
            url
          }
          details {
            id
            text
            value
            max
            color
          }
        }
        currentTime
      }
    }
  }
  }`

  const variables = {
    category: "02ba33c2-d364-41cf-ab96-ce46cb6821ca"
  }

  return dealTechInstance({
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    data: {
      query,
      variables
    }
  })
}

export { fetchArticles }
