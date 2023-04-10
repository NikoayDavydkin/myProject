import IArticleBody from './articleDetailBody'
import ISource from './sources'

interface IArticleDetail {
  title: string
  body: IArticleBody[]
  sources: ISource[]
}

export default IArticleDetail
