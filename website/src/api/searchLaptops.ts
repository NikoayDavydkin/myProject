import axios, { CancelTokenSource } from 'axios'
import dealTechBaseInstance from './dealTechBaseInstance'

const getSearchInstance = () => {
  let token: CancelTokenSource = null

  return async function (query: string, { variables } = { variables: {} }) {
    if (token) {
      token.cancel()
    }
    token = axios.CancelToken.source()
    try {
      const res = await dealTechBaseInstance({
        cancelToken: token.token,
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        data: {
          query,
          variables
        }
      })
      return res.data.data.search
    } catch (err) {
      if (axios.isCancel(err)) {
        return {
          isCancel: true
        }
      }
    }
  }
}

const searchLaptops = getSearchInstance()

export async function search(
  query: string | string[],
  category: string,
  sortOrder: string,
  attributes: { [key: string]: any }[],
  page: number,
  pageSize: number,
  price: { min: number; max: number }
): Promise<any> {
  let newAttributes = []
  for (let i = 0; i < attributes.length; i++) {
    newAttributes[i] = { ...attributes[i] }
    if (typeof attributes[i].min !== 'undefined') {
      newAttributes[i].min = String(attributes[i].min)
    }
    if (typeof attributes[i].max !== 'undefined') {
      newAttributes[i].max = String(attributes[i].max)
    }
  }
  const data = await searchLaptops(
`query search($query: String, $category: UUID, $sortOrder: UUID, $filters: [SearchAttributeInput!], $page: Int, $pageSize: Int, $price: MinMaxPriceInput) {
       search(query: $query, category: $category, sortOrder: $sortOrder, filters: $filters, page: $page, pageSize: $pageSize, price: $price) {
   content {
     __typename
     ... on Search {
       debug
       numProducts
       products {
         id
         title
         imageUrl
         selectOffer {
           price
           shipping
           url
         }
         details {
           text
         }
         imageUrl
         seller
         buyUrl
         updated
       }
     }
   }
   filters {
     attribute
     name
     filterType
     attributeValues
     units
   }
   sortOrders {
     id
     name
   }
 }
}`,
    {
      variables: {
        query,
        category,
        sortOrder,
        filters: newAttributes,
        page,
        pageSize,
        price
      }
    }
  )
  return data
}
