import { FunctionComponent, ReactElement, useState } from 'react'
import SearchIcon from '@material-ui/icons/Search'
import { useRouter } from 'next/router'
import useStyles from './styles'

interface IProps {
  setSearchingProducts: (value: boolean) => void
  getQuery: (value: string) => void
}

const Searchbar: FunctionComponent<IProps> = ({
  setSearchingProducts,
  getQuery
}): ReactElement => {
  const router = useRouter()
  let initialQuery = ''
  if (router.query.q) initialQuery = router.query.q as string

  const [query, setQuery] = useState<string>(initialQuery)
  const classes = useStyles()

  const routeToSearch = () => {
    if (router.pathname !== '/') {
      router.push(`/?q=${query.trim()}`)
    } else {
      setSearchingProducts(true)
      getQuery(query.trim()===''? ' ' : query.trim())
    }
  }

  return (
    <div className={classes.searchBox}>
      <span className={classes.searchIconBox}>
        <SearchIcon className={classes.searchIcon} />
      </span>
      <div className={classes.inputBox}>
        <input
          type='text'
          placeholder='Search for Laptops...'
          value={query}
          onChange={e => setQuery(e.target.value)}
          onKeyDown={e => {
            if (e.key === 'Enter') routeToSearch()
          }}
        />
      </div>
    </div>
  )
}

export default Searchbar
