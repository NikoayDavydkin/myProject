import {
  useEffect,
  useState,
  ReactElement,
  Fragment,
  FunctionComponent,
  useRef
} from 'react'
import { Theme } from '@material-ui/core/styles'
import { search } from '../src/api/searchLaptops'
import Router from 'next/router'
import Grid from '@material-ui/core/Grid'
import Button from '@material-ui/core/Button'
import IconButton from '@material-ui/core/IconButton'
import SwipeableDrawer from '@material-ui/core/SwipeableDrawer'
import Typography from '@material-ui/core/Typography'
import useMediaQuery from '@material-ui/core/useMediaQuery'
import Pagination from '@material-ui/lab/Pagination'
import Paper from '@material-ui/core/Paper'
import Box from '@material-ui/core/Box'
import Head from 'next/head'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faRedditSquare } from '@fortawesome/free-brands-svg-icons'

import Skeleton from 'react-loading-skeleton'
import { useTheme } from '@material-ui/styles'
import MoreVertIcon from '@material-ui/icons/MoreVert'

import fetchSortOrders from '../src/api/fetchSortOrders'
import fetchFilters from '../src/api/fetchFilters'
import fetchRedditMarkup from '../src/api/fetchRedditMarkup'
import useStyles from '../src/styles/search'
import Sidebar from '../src/components/Sidebar/Sidebar'
import SingleSelect from '../src/components/Sidebar/UI/SingleSelect'
import IFetchedFilter from '../src/interfaces/filters/fetchedFilter'
import IFetchedRedditProductMarkup from '../src/interfaces/fetchedRedditProductMarkup'
import FilterValueType from '../src/interfaces/filters/filterValueType'
import IFilter from '../src/interfaces/filters/filters'
import FILTER_TYPES from '../src/interfaces/filters/filterTypes'
import ISortOrder from '../src/interfaces/filters/sortOrders'
import isEmpty from 'lodash/isEmpty'
import merge from 'lodash/merge'

import { GetServerSidePropsContext } from 'next'
import getAttributes from '../src/utils/getAttributes'
import getURLDisplayAttributes from '../src/utils/getURLDisplayAttributes'
import Sponsored from '../src/components/Sponsored/Sponsored'
import AmazonPrice from '../src/components/AmazonPrice/AmazonPrice'

export async function getServerSideProps(
  context: GetServerSidePropsContext
): Promise<{
  props: {
    query: {
      q: string | string[]
      filters?: string | string[]
      page?: string | string[]
      sort_order?: string | string[]
    }
  }
}> {
  return {
    props: {
      query: {
        q: context.query.q ? context.query.q : '',
        filters: context.query.filters ? context.query.filters : null,
        page: context.query.page ? context.query.page : '1',
        sort_order: context.query.sort_by ? context.query.sort_by : ''
      }
    }
  }
}

export default function Search({
  query: params,
  searchingProducts,
  setSearchingProducts,
  searchQuery
}: {
  query: {
    q: string | string[]
    filters?: string | string[]
    page?: string | string[]
    sort_order?: string | string[]
  }
  searchingProducts: boolean
  setSearchingProducts: (value: boolean) => void
  searchQuery: string
}): ReactElement {
  /** All the state for this component */
  const [loadingFilters, setLoadingFilters] = useState<boolean>(true)
  const [loadingSortOrders, setLoadingSortOrders] = useState<boolean>(true)
  // All the filters that wana show on the page fetched from api
  const [filtersData, setFiltersData] = useState<{
    [key: string]: IFetchedFilter
  } | null>(null)

  const [productsCount, setProductsCount] = useState<number>(null)

  // User selected filters
  const [filters, setFilters] = useState<{ [key: string]: IFilter }>(null)

  // Array of all the filters fetched from the api
  const [sortOrders, setSortOrders] = useState<ISortOrder[]>([])

  // ID of the selected sort
  const [selectedSortOrder, setSeletedSortOrder] = useState<ISortOrder>(null)

  // Current Page
  const [selectedPage, setSelectedPage] = useState<number>(1)

  // No # of pages fetched from the api
  const [pageCount, setPageCount] = useState(1)

  // State for the drawer
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)
  const iOS = process.browser && /iPad|iPhone|iPod/.test(navigator.userAgent)

  /** Consuming HOOKS */
  const classes = useStyles()
  // For reading the query from the URL

  const { q: query } = params

  //Consumes the material-ui theme
  const theme: Theme = useTheme()

  // Media query for mobile and desktop screen
  const isBigScreen = useMediaQuery(theme.breakpoints.up('sm'))

  const isFirstRun = useRef(true)

  ///////////////////////////////////////////////////////////
  function handleFiltersChange(values: IFilter) {
    setSelectedPage(1)
    setFilters({
      ...filters,
      [values.name]: {
        ...filters[values.name],
        ...values
      }
    })
    setSearchingProducts(true)
  }

  useEffect(() => {
    if (isFirstRun.current) {
      isFirstRun.current = false
      return
    }
    if (!isEmpty(filters)) {
      let displayQuery = {}

      if (!isEmpty(filtersData)) {
        displayQuery = getURLDisplayAttributes(filters, filtersData)
      }

      const filtersAttr = {}
      const newQueryArray = []

      if (!isEmpty(displayQuery)) {
        filtersAttr['c'] = JSON.stringify(displayQuery)
      }

      if (selectedPage > 1) {
        filtersAttr['a'] = selectedPage
      }

      if (selectedSortOrder && selectedSortOrder.name !== 'Featured') {
        filtersAttr['b'] = selectedSortOrder.id
      }

      Object.keys(filtersAttr)
        .sort()
        .forEach(key => {
          if (key === 'a') {
            newQueryArray.push(`page=${selectedPage}`)
          } else if (key === 'b') {
            newQueryArray.push(`sort_by=${selectedSortOrder.id}`)
          } else if (key === 'c') {
            newQueryArray.push(`filters=${JSON.stringify(displayQuery)}`)
          }
        })
      if (searchQuery !== query && searchQuery.length > 0) {
        if (newQueryArray.length === 0) {
          Router.replace(encodeURI(`?q=${searchQuery}`))
        } else {
          Router.replace(
            encodeURI(`?q=${searchQuery}&${newQueryArray.join('&')}`)
          )
        }
      } else {
        if (newQueryArray.length === 0) {
          Router.replace(encodeURI(`?q=${query}`))
        } else {
          Router.replace(encodeURI(`?q=${query}&${newQueryArray.join('&')}`))
        }
      }
    }
  }, [searchingProducts])

  function toggleDrawer(open: boolean) {
    setIsDrawerOpen(open)
  }
  useEffect(() => {
    setSelectedPage(Number(params.page))
  }, [])

  useEffect(() => {
    const sortOrder = params.sort_order
    if (sortOrder) {
      if (sortOrders.length > 0) {
        const order = sortOrders.find(od => od.id === sortOrder)
        setSeletedSortOrder(order)
      }
    }
  }, [sortOrders])

  // Fetch filters from the api
  useEffect(() => {
    fetchFilters().then(res => {
      setLoadingFilters(false)
      // Array of filters fetched from an api
      const filtersFetched: IFetchedFilter[] = res.data.data.search.filters

      // Push the price filter
      filtersFetched.push({
        attribute: '3b0e2207-50c8-4d44-81af-0aa02c8790f0',
        name: 'Price',
        attributeValues: ['0', '20000'],
        units: '$',
        filterType: FILTER_TYPES.minMaxContinousPriceSlider
      })

      // Change the filters to key value pairs
      const filtersData: { [key: string]: IFetchedFilter } = {}
      filtersFetched.forEach(filter => {
        filtersData[filter.name] = { ...filter }
      })

      // select the filters initailly -> user selected filters state
      const filterKeys: { [key: string]: IFilter } = {}

      // Type of the filter for each filter
      let values: FilterValueType

      // Loop over the filter and find the inital filters state value
      filtersFetched.forEach(filter => {
        values = []
        if (filter.filterType === FILTER_TYPES.multiSelect) {
          values = null
        } else if (
          filter.filterType === FILTER_TYPES.minMaxSlider ||
          filter.filterType === FILTER_TYPES.minMaxContinousSlider ||
          filter.filterType === FILTER_TYPES.minMaxContinousPriceSlider
        ) {
          values[0] = Number(filtersData[filter.name].attributeValues[0])
          values[1] = Number(
            filtersData[filter.name].attributeValues[
              filtersData[filter.name].attributeValues.length - 1
            ]
          )
        } else if (filter.filterType === FILTER_TYPES.minMaxStringSlider) {
          values[0] = filtersData[filter.name].attributeValues[0]
          values[1] =
            filtersData[filter.name].attributeValues[
              filtersData[filter.name].attributeValues.length - 1
            ]
        }
        filterKeys[filter.name] = {
          name: filter.name,
          value: values,
          type: filter.filterType,
          attribute: filter.attribute,
          unit: filter.units
        }
      })

      const queryFiltersString: string = params.filters as string

      let queryFiltersObj: {
        [key: string]: {
          min?: string | number
          max?: string | number
          values: FilterValueType
        }
      } = {}
      const filterFromQuery = {}
      if (queryFiltersString) {
        queryFiltersObj = JSON.parse(decodeURI(queryFiltersString))
        Object.keys(queryFiltersObj).forEach(attribute => {
          const filterDataQuery =
            filtersData[
              Object.keys(filtersData).find(name => {
                const filter = filtersData[name]
                return filter.attribute === attribute
              })
            ]
          const queryFilter = queryFiltersObj[attribute]
          let values = []
          if (
            FILTER_TYPES.minMaxContinousPriceSlider ===
              filterDataQuery.filterType ||
            FILTER_TYPES.minMaxContinousSlider === filterDataQuery.filterType ||
            FILTER_TYPES.minMaxSlider === filterDataQuery.filterType
          ) {
            const minValue = Number(filterDataQuery.attributeValues[0])
            const maxValue = Number(
              filterDataQuery.attributeValues[
                filterDataQuery.attributeValues.length - 1
              ]
            )
            if (queryFilter.min) {
              values.push(queryFilter.min)
            } else {
              values.push(minValue)
            }

            if (queryFilter.max) {
              values.push(queryFilter.max)
            } else {
              values.push(maxValue)
            }
          } else if (FILTER_TYPES.multiSelect === filterDataQuery.filterType) {
            values = queryFilter.values
          } else if (FILTER_TYPES.minMaxStringSlider) {
            const minValue = filterDataQuery.attributeValues[0]
            const maxValue =
              filterDataQuery.attributeValues[
                filterDataQuery.attributeValues.length - 1
              ]

            if (queryFilter.min) {
              values.push(queryFilter.min)
            } else {
              values.push(minValue)
            }

            if (queryFilter.max) {
              values.push(queryFilter.max)
            } else {
              values.push(maxValue)
            }
          }
          filterFromQuery[filterDataQuery.name] = {
            name: filterDataQuery.name,
            type: filterDataQuery.filterType,
            attribute: filterDataQuery.attribute,
            unit: filterDataQuery.units,
            value: values
          }
        })
      }
      const newFilters = merge(filterKeys, filterFromQuery)
      setFilters(newFilters)
      setFiltersData(filtersData)
      setSearchingProducts(true)
    })
  }, [])

  // fetch sort options
  useEffect(() => {
    fetchSortOrders().then(res => {
      setLoadingSortOrders(false)
      const sortOrders: ISortOrder[] = res.data.data.search.sortOrders
      setSeletedSortOrder(sortOrders[0])
      setSortOrders(sortOrders)
    })
  }, [])

  // check if there are filters to display
  const sidebarContent = (
    <Sidebar
      loading={loadingFilters}
      filters={filters}
      filtersData={filtersData}
      onFilterChange={handleFiltersChange}
      onSidebarClose={toggleDrawer}
    />
  )

  let resultsInfo = <Skeleton width={202} height={25} />

  if (productsCount && !searchingProducts) {
    if (selectedPage === 1) {
      resultsInfo = (
        <Typography variant='subtitle1' component='p'>
          {productsCount} results
        </Typography>
      )
    } else {
      resultsInfo = (
        <Typography variant='subtitle1' component='p'>
          Page {selectedPage} of {productsCount} results
        </Typography>
      )
    }
  }

  return (
    <Fragment>
      <Head>
        <title>DealTech | Search for Laptops, Computers, and Parts</title>
        <meta
          name='description'
          content='Search for Laptops, Computers, and Parts'
        />
      </Head>
      <div>
        <main className={classes.main}>
          {/* check if sort odrers array is empty */}
          <div style={{ display: 'flex' }}>
            <Grid container spacing={2}>
              <Grid item sm={4} md={3}></Grid>
              <Grid item xs={12} sm={8} md={9}>
                <div className={classes.SearchInfoBar}>
                  {resultsInfo}
                  {/*<Button
                    variant='contained'
                    color='primary'
                    href={`https://www.google.com/search?q=${query}`}
                    target='_blank'
                  >
                    Search Google
                  </Button>
                  <Button
                    variant='contained'
                    color='primary'
                    href={`https://www.youtube.com/search?q=${query}`}
                    target='_blank'
                  >
                    Search YouTube
                  </Button>
                  <Button
                    variant='contained'
                    color='primary'
                    href={`https://www.amazon.com/s?k=${query}`}
                    target='_blank'
                  >
                    Search Amazon
                  </Button>
                  <Button
                    variant='contained'
                    color='primary'
                    href={`https://www.ebay.com/sch/i.html?_nkw=${query}`}
                    target='_blank'
                  >
                    Search Ebay
                  </Button>*/}
                  {!loadingSortOrders ? (
                    <SingleSelect
                      name='sort'
                      options={sortOrders}
                      value={selectedSortOrder}
                      onChange={value => {
                        setSeletedSortOrder(value)
                        setSearchingProducts(true)
                      }}
                    />
                  ) : (
                    <Skeleton width={202} height={40} />
                  )}
                </div>
              </Grid>
            </Grid>
          </div>

          {/* Block Reclama*/}
          <div style={{ float: 'right' }}>
            <Sponsored />
          </div>

          {/* show the toggle drawer button */}
          {!isBigScreen && (
            <div>
              <Button
                color='primary'
                onClick={() => toggleDrawer(true)}
                startIcon={<MoreVertIcon />}
              >
                Filters
              </Button>
            </div>
          )}

          <div>
            <Grid container spacing={2}>
              <Grid item sm={4} md={3}>
                {isBigScreen && <>{sidebarContent}</>}
                {!isBigScreen && (
                  <SwipeableDrawer
                    open={isDrawerOpen}
                    onClose={() => setIsDrawerOpen(false)}
                    onOpen={() => setIsDrawerOpen(true)}
                    disableBackdropTransition={!iOS}
                    disableDiscovery={iOS}
                  >
                    {sidebarContent}
                  </SwipeableDrawer>
                )}
              </Grid>
              <Grid item xs={12} sm={8} md={9}>
                {filters && filtersData && sortOrders && (
                  <>
                    <ProductGrids
                      setSearching={setSearchingProducts}
                      searching={searchingProducts}
                      searchQuery={searchQuery}
                      query={query}
                      filters={filters}
                      filtersData={filtersData}
                      selectedSort={selectedSortOrder}
                      selectedPage={selectedPage}
                      setPageCount={setPageCount}
                      setProductsCount={setProductsCount}
                    ></ProductGrids>
                    <div
                      style={{
                        display: 'flex',
                        justifyContent: 'center',
                        marginTop: '30px'
                      }}
                    >
                      <Pagination
                        count={pageCount}
                        color='primary'
                        page={selectedPage}
                        boundaryCount={4}
                        siblingCount={4}
                        onChange={(e, pageNumber) => {
                          window.scrollTo(0, 0)
                          setSelectedPage(pageNumber)
                          setSearchingProducts(true)
                        }}
                      />
                    </div>
                  </>
                )}
              </Grid>
            </Grid>
          </div>
        </main>
      </div>
    </Fragment>
  )
}

interface IProductGridProps {
  query: string | string[]
  searchQuery: string
  filters: { [key: string]: IFilter }
  filtersData: { [key: string]: IFetchedFilter }
  selectedSort: ISortOrder
  selectedPage: number
  setPageCount: (value: number) => void
  searching: boolean
  setSearching: (value: boolean) => void
  setProductsCount: (value: number) => void
}

const ProductGrids: FunctionComponent<IProductGridProps> = ({
  query,
  searchQuery,
  filters,
  filtersData,
  selectedSort,
  selectedPage,
  setPageCount,
  searching,
  setSearching,
  setProductsCount
}): ReactElement => {
  const [searchResponse, setSearchResponse] = useState(null)
  const [loadingProducts, setLoadingProducts] = useState<boolean>(true)

  useEffect(() => {
    if (searching) {
      setLoadingProducts(true)

      const attributes = getAttributes(filters, filtersData)
      const newAttributes = attributes.filter(
        attr => attr.attribute !== '3b0e2207-50c8-4d44-81af-0aa02c8790f0'
      )
      let sortOrder: string = null
      if (selectedSort) {
        sortOrder = selectedSort.id
      }

      let minPrice = Number(filters['Price'].value[0])
      let maxPrice = Number(filters['Price'].value[1])
      if (Object.is(minPrice, -0)) minPrice = 0
      if (Object.is(maxPrice, -0)) maxPrice = 0
      const price = {
        min: minPrice * 100,
        max: maxPrice * 100
      }

      let q = query
      if (query !== searchQuery && searchQuery.length > 0) {
        q = searchQuery
      }
      const category = 'dfe0c6a8-3b02-41d5-8eab-5375ba4bc063'
      const page = selectedPage
      const pageSize = 50
      search(q, category, sortOrder, newAttributes, page, pageSize, price).then(
        res => {
          if (res && !res.isCancel) {
            console.log(res.content.debug)
            console.log(res)
            setPageCount(Math.floor((res.content.numProducts + pageSize - 1) / pageSize))
            setSearchResponse(res)
            setProductsCount(res.content.numProducts)
          } else {
            console.log("Error fetching content")
          }
          setLoadingProducts(false)
          setSearching(false)
        }
      )
    }
  }, [searching])

  if (loadingProducts) {
    return (
      <Grid container spacing={2}>
        {Array.from(Array(16).keys()).map(key => (
          <ProductItem key={key} loading={loadingProducts} />
        ))}
      </Grid>
    )
  } else if (
    !loadingProducts &&
    searchResponse &&
    searchResponse.content.products.length === 0
  ) {
    return (
      <>
        <Typography variant='h5' align='center' component='p'>
          No Products Found
        </Typography>
        <Typography variant='body1' align='center' component='p'>
          Please try your search again using fewer keywords or filters
        </Typography>
      </>
    )
  } else {
    return (
      <div>
        <Grid container spacing={2}>
          {searchResponse.content.products.map((product, index) => (
            <ProductItem key={index} product={product} />
          ))}
        </Grid>
      </div>
    )
  }
}

//  Purpose of this component is  to correctly size the every list item within the grid component
interface IProductItemProps {
  product?: any
  loading?: boolean
}

const ProductItem: FunctionComponent<IProductItemProps> = ({
  product,
  loading
}): ReactElement => {
  return (
    <Grid item xs={12} sm={6} md={4} lg={3} style={{ display: 'flex' }}>
      <Paper
        variant='outlined'
        style={{
          display: 'flex',
          flexDirection: 'column',
          width: '100%'
        }}
      >
        {!loading ? (
          <>
            <ProductPhoto product={product} />
            <ProductTitle title={product.title} />
            <ul style={{ listStyleType: 'none', padding: '16px' }}>
              {product.details.map((detail, index) => (
                <li key={index}>{detail.text}</li>
              ))}
            </ul>
            <div
              style={{
                marginTop: 'auto',
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center'
              }}
            >
              <AmazonPrice offer={product.selectOffer} updated={product.updated}/>
              <ProductButtons product={product}></ProductButtons>
            </div>
          </>
        ) : (
          <>
            <Skeleton
              height={175}
              style={{
                borderRadius: '0px',
                lineHeight: '20px'
              }}
            />
            <div style={{ padding: '16px' }}>
              <Skeleton height={16} count={3} />
              <div style={{ marginTop: '1rem' }} />
              <Skeleton count={9} />
            </div>
          </>
        )}
      </Paper>
    </Grid>
  )
}

//  Purpose  of this component  is to correctly position within the parent component
const ProductPhoto = ({ product }) => {
  const [hasLoaded, setHasLoaded] = useState(false)
  const classes = useStyles()
  useEffect(() => {
    const image = new Image()
    image.src = product.imageUrl
    image.onload = () => {
      setHasLoaded(true)
    }
  }, [])

  return (
    <div className={classes.ProductPhoto}>
      {hasLoaded ? (
        <div
          className={classes.ProductPhotoImage}
          style={{ backgroundImage: `url(${product.imageUrl})` }}
        ></div>
      ) : (
        <Skeleton height={175}></Skeleton>
      )}
      {/* Photo goes here */}
    </div>
  )
}

const ProductTitle = ({ title }) => {
  const classes = useStyles()
  return (
    <div className={classes.ProductDetails}>
      <span>
        <b>{title}</b>
      </span>
    </div>
  )
}

const ProductButtons = ({ product }) => {
  const classes = useStyles()
  return (
    <div className={classes.ButtonGroup}>
      <div className={classes.OfferGroup}>
        <a
          href={product.buyUrl}
          rel='noreferrer'
          target='_blank'
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '3px 22px',
            fontSize: '0.8125rem',
            color: '#3f51b5',
            border: '1px solid rgba(63, 81, 181, 0.5)',
            borderRadius: '25px',
            textDecoration: 'none'
          }}
        >
          View on {product.seller}
        </a>
        <div>
          <FontAwesomeIcon
            icon={faRedditSquare}
            size="2x"
            style={{ color: '#FF4500' }}
            onClick={() => {
              fetchRedditMarkup(product.id).then(res => {
                const markup: IFetchedRedditProductMarkup = res.data.data.productMarkup;
                navigator.clipboard.writeText(markup.text);
                alert('Reddit markdown copied')
              })
            }}
          />
        </div>
      </div>
      <Box m={2}></Box>
    </div>
  )
}
