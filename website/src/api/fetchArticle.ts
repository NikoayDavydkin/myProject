import { AxiosResponse } from 'axios'
import strapiInstance from './dealTechStrapiInstance'

const fetchArticle = (id: string | string[]): Promise<AxiosResponse> => {
  const query = `query search($category: ID) {
    search(category: $category) {
      content {
        __typename
        ... on Product {
          title
          body {
            __typename
            ... on Video {
              url
            }
            ... on RichParagraph {
              title
              imageUrl
              richText
              specs
              pros
              cons
              link
              price
              updated
            }
  
            ... on RichText {
              text
            }
          }
          sources {
              author
              date
              title
              link
          }
          currentTime
        }
      }
    }
  }`

  const variables = {
    category: id
  }

  return strapiInstance({
    method: 'POST',
    data: {
      query,
      variables
    }
  })
}

export default fetchArticle
