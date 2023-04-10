interface IArticle {
  id: string
  title: string
  summary: string
  image: string
  lastModified: string
  created_at: string
  updated_at: string
  default_sort_order: bigint
}

export default IArticle
