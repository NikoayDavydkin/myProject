import IArticle from '../../interfaces/article'
import ArticleCard from '../ArticleCard/ArticleCard'
import classes from './articlesGrid.module.scss'
import Masonry from 'react-masonry-css'

interface IProps {
  articles: IArticle[]
}

const ArticlesGrid: React.FunctionComponent<IProps> = ({
  articles
}): React.ReactElement => {
  return (
    <Masonry
      breakpointCols={{
        default: 4,
        1280: 3,
        800: 2,
        600: 1
      }}
      className={classes.grid}
      columnClassName={classes.grid_item}
    >
      {articles.length >= 1
        ? articles.map(article => (
            <ArticleCard key={article.id} article={article} />
          ))
        : [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map(number => (
            <ArticleCard key={number} article={null} />
          ))}
    </Masonry>
  )
}

export default ArticlesGrid
