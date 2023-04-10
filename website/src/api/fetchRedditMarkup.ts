import { AxiosResponse } from 'axios'
import redditBotInstance from './dealTechRedditBotInstance'

const fetchRedditMarkup = (id: string | string[]): Promise<AxiosResponse> => {
  const query = `query productMarkup($id: ID) {
    productMarkup(id: $id) {
      text
    }
  }`

  const variables = {
    id: id
  }

  return redditBotInstance({
    method: 'POST',
    data: {
      query,
      variables
    }
  })
}

export default fetchRedditMarkup
