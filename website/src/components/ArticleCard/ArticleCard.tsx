import { makeStyles } from '@material-ui/core/styles'
import { useRouter } from 'next/router'
import Skeleton from 'react-loading-skeleton'
import IArticle from '../../interfaces/article'
import Link from 'next/link'

interface IProps {
  article: IArticle
}

const useStyles = makeStyles(theme => ({
  card: {
    backgroundColor: theme.palette.background.default,
    boxShadow: '0px 1px 1px rgba(0, 0, 0, 0.1)',
    borderRadius: '3px'
  },

  image_top: {},

  card_body: {
    padding: theme.spacing(3, 1.5, 3, 1.5)
  },
  title: {
    color: theme.dark.main700,
    fontSize: '18px',
    fontWeight: 500,
    marginBottom: '4px'
  },
  modified: {
    fontSize: '12px',
    color: theme.dark.main100
  },

  summary: {
    color: theme.dark.main500,
    lineHeight: '180%'
  },

  button: {
    border: 'none',
    outline: 'none',
    color: '#fff',
    backgroundColor: theme.mainColor.main,
    padding: '4px 7px',
    marginLeft: '5px',
    cursor: 'pointer',
    fontSize: '.8rem'
  }
}))

const ArticleCard: React.FunctionComponent<IProps> = (
  { article } = { article: null }
): React.ReactElement => {
  const classes = useStyles()
  const router = useRouter()
  return (
    <div className={classes.card}>
      <div className={classes.image_top}>
        {article ? (
          <Link href={`${article.id}`}>
            <a>
              <img
                src={article.image}
                style={{
                  width: '100%',
                  borderTopLeftRadius: '3px',
                  borderTopRightRadius: '3px'
                }}
              />
            </a>
          </Link>
        ) : (
          <Skeleton
            height={200}
            style={{
              borderRadius: '0px',
              borderTopLeftRadius: '3px',
              borderTopRightRadius: '3px',
              lineHeight: '20px'
            }}
          />
        )}
      </div>
      <div className={classes.card_body}>
        <h4 className={classes.title}>
          {article ? article.title : <Skeleton />}
        </h4>
        <p className={classes.modified}>
          {article ? article.lastModified : <Skeleton width='20%' />}
        </p>
        <p className={classes.summary}>
          {article ? (
            <>
              {article.summary}...
              <button
                className={classes.button}
                onClick={() => {
                  router.push(`${article.id}`)
                }}
              >
                Read more
              </button>
            </>
          ) : (
            <Skeleton count={10} />
          )}
        </p>
      </div>
    </div>
  )
}

export default ArticleCard
