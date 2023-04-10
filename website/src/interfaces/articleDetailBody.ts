import CONTENTTYPES from './contentTypes'

interface IArticleBody {
  __typename: CONTENTTYPES
  url?: string
  title?: string
  imageUrl?: string
  text?: string
  richText?: string
  specs?: string[]
  pros?: string[]
  cons?: string[]
  link?: string
}

export default IArticleBody
