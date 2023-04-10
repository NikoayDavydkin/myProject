interface IFetchedArticle {
  id: string
  title: string
  summary: string
  imageUrl: string
  updated: string
  created: string
  defaultSortOrder: bigint
}

export default IFetchedArticle
