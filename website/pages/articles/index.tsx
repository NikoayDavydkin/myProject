import { useState, useEffect } from 'react'
import Select, { ValueType } from 'react-select'
import ArrowDropdownIcon from '@material-ui/icons/ArrowDropDown'
import SearchIcon from '@material-ui/icons/Search'
import useStyles from '../../src/styles/articles'
import { fetchArticles } from '../../src/api/fetchArticles'
import ArticlesGrid from '../../src/components/ArticlesGrid/ArticlesGrid'
import IArticle from '../../src/interfaces/article'
import IContent from '../../src/interfaces/content'
import IFetchedArticle from '../../src/interfaces/fetchedArticle'
import orderBy from 'lodash/orderBy'

const months = [
  'Jan',
  'Feb',
  'March',
  'April',
  'May',
  'June',
  'July',
  'Aug',
  'Sep',
  'Oct',
  'Nov',
  'Dec'
]

enum SORT_TYPES {
  Default = 'default',
  Last_Modified = 'lastModified',
  Publication_Date_New_to_Old = 'publicationDateNewtoOld',
  Publication_Date_Old_to_New = 'publicationDateOldToNew'
}

const sortOptions = [
  {
    value: SORT_TYPES.Default,
    label: 'Default'
  },
  {
    value: SORT_TYPES.Last_Modified,
    label: 'Last Modifed'
  },
  {
    value: SORT_TYPES.Publication_Date_New_to_Old,
    label: 'Publication Date: New to Old'
  },
  {
    value: SORT_TYPES.Publication_Date_Old_to_New,
    label: 'Publication Date: Old to New'
  }
]

const dropdownStyles = {
  control: provided => ({
    ...provided,
    backgroundColor: '#fff',
    border: '1px solid #212121',
    borderRadius: 'none',
    boxShadow: 'none',
    '&:hover': {
      border: '1px solid #212121'
    },

    '&:active': {
      border: '1px solid #212121'
    }
  }),

  option: (provided, state) => {
    const { isSelected } = state
    return {
      ...provided,
      color: isSelected ? '#fff' : '#212121',
      backgroundColor: isSelected ? '#212121' : '#fff',
      ':active': {
        backgroundColor: '#fff'
      }
    }
  }
}

const Articles: React.FunctionComponent = (): React.ReactElement => {
  const [sortOrder, setSortOrder] = useState<
    ValueType<{
      [key: string]: string
    }>
  >(sortOptions[0])
  const [query, setQuery] = useState<string>('')
  const [articles, setArticles] = useState<IArticle[]>([])
  const [loading, setLoading] = useState<boolean>(true)

  useEffect(() => {
    fetchArticles().then(res => {
      console.log("res.data", res.data)
      setLoading(false)
      const content: IContent = res.data.data.search.content
      const fetchedArticles: IFetchedArticle[] = content.products
      const newArticles: IArticle[] = []
      fetchedArticles.forEach(article => {
        const date = new Date(article.updated)
        const month = months[date.getMonth()]
        const day = date.getDay()
        const year = date.getFullYear()
        let image: string = article.imageUrl
        newArticles.push({
          id: article.id,
          title: article.title,
          summary: article.summary,
          image: `${process.env.NEXT_PUBLIC_IMAGE}${image}`,
          lastModified: `${month} ${day}, ${year}`,
          created_at: article.created,
          updated_at: article.updated,
          default_sort_order: article.defaultSortOrder
        })
      })
      setArticles(newArticles)
    }).catch(function (error) {
      console.log("Could not get strapi url '" + process.env.NEXT_PUBLIC_BONFIRE + "'")
      if (error.response) {
        // Request made and server responded
        console.log(error.response.data);
        console.log(error.response.status);
        console.log(error.response.headers);
      } else if (error.request) {
        // The request was made but no response was received
        console.log(error.request);
      } else {
        // Something happened in setting up the request that triggered an Error
        console.log('Error:', error.message);
      }
    });
  }, [])

  const classes = useStyles()

  const handleSortOrderChange = (
    value: ValueType<{
      [key: string]: string
    }>
  ) => {
    setSortOrder(value)
  }

  let newArticles = articles
  const sortBy = sortOrder as { value: string; label: string }
  if (sortBy.value === SORT_TYPES.Default)
    newArticles = orderBy(articles, 'default_sort_order', 'asc')
  else if (sortBy.value === SORT_TYPES.Last_Modified)
    newArticles = orderBy(articles, 'updated_at', 'desc')
  else if (sortBy.value === SORT_TYPES.Publication_Date_New_to_Old)
    newArticles = orderBy(articles, 'created_at', 'desc')
  else if (sortBy.value === SORT_TYPES.Publication_Date_Old_to_New)
    newArticles = orderBy(articles, 'created_at', 'asc')

  if (query.length > 0)
    newArticles = newArticles.filter(article => {
      return (
        article.title.toLocaleLowerCase().includes(query.toLowerCase()) ||
        article.summary.toLowerCase().includes(query.toLowerCase())
      )
    })

  return (
    <div className={classes.articles}>
      <div className='wrapper'>
        <header className={classes.header}>
          <Select
            instanceId='articles-sort-dropdown'
            options={sortOptions}
            defaultValue={sortOrder}
            onChange={handleSortOrderChange}
            getOptionLabel={option => option.label}
            styles={dropdownStyles}
            className={classes.sortDropdown__comp}
            components={{
              IndicatorSeparator: () => null,
              // eslint-disable-next-line react/display-name
              DropdownIndicator: () => <ArrowDropdownIcon />
            }}
          />

          <div className={classes.searchWrapper}>
            <SearchIcon className={classes.searchIcon} />
            <input
              type='text'
              className={classes.searchInput}
              placeholder='search'
              value={query}
              onChange={e => setQuery(e.target.value)}
            />
          </div>
        </header>
        <main className={classes.main}>
          {loading && <ArticlesGrid articles={articles} />}
          {!loading && articles.length === 0 && <div>No article is found</div>}
          {!loading && <ArticlesGrid articles={newArticles} />}
        </main>
      </div>
    </div>
  )
}

export default Articles
