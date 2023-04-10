import { FunctionComponent, ReactElement, useEffect } from 'react'
import { GetServerSidePropsContext } from 'next'
import useStyles from '../src/styles/articleDetail'
import ArticleSection from '../src/components/ArticleSection/ArticleSection'
import fetchArticle from '../src/api/fetchArticle'
import IArticleDetail from '../src/interfaces/detailedArticle'

interface IProps {
  article: IArticleDetail
}

const DetailPage: FunctionComponent<IProps> = ({ article }): ReactElement => {
  const classes = useStyles()
  useEffect(() => {
    window.scrollTo(0, 0)
  }, [])
  return (
    <div className='wrapper'>
      <div className={classes.main}>
        <h1 className={classes.title}>{article.title}</h1>
        {article.body.length !== 0 &&
          article.body.map((section, index) => (
            <ArticleSection key={index} section={section} />
          ))}
        {article.sources && <ArticleSection sources={article.sources} />}
      </div>
    </div>
  )
}

export async function getServerSideProps(
  context: GetServerSidePropsContext
): Promise<{
  props: { article: IArticleDetail }
}> {
  const { id } = context.query

  const res = await fetchArticle(id)

  return { props: { article: res.data.data.search.content } }
}

export default DetailPage
