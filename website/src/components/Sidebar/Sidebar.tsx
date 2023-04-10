import { FunctionComponent, ReactElement } from 'react'
import { makeStyles, useTheme } from '@material-ui/styles'
import useMediaQuery from '@material-ui/core/useMediaQuery'
import Typography from '@material-ui/core/Typography'
import SelectUI from './UI/SelectUI'
import CloseIcon from '@material-ui/icons/Close'
import { Theme } from '@material-ui/core'
import IFilter from '../../interfaces/filters/filters'
import IFetchedFilter from '../../interfaces/filters/fetchedFilter'
import Skeleton, { SkeletonTheme } from 'react-loading-skeleton'

const useStyles = makeStyles((theme: Theme) => ({
  root: {
    backgroundColor: theme.mainColor.main,
    color: '#fff',
    padding: '20px 25px',
    position: 'relative'
  },
  inputGroup: {
    marginBottom: theme.spacing(3)
  },

  titleBox: {
    marginBottom: theme.spacing(3)
  },

  label: {
    marginBottom: '2px',
    display: 'block'
  }
}))

interface IProps {
  loading: boolean
  filters: { [key: string]: IFilter }
  filtersData: { [key: string]: IFetchedFilter }
  onFilterChange: (value: IFilter) => void
  onSidebarClose: (open: boolean) => void
}

const Sidebar: FunctionComponent<IProps> = ({
  filters,
  filtersData,
  onFilterChange,
  onSidebarClose,
  loading
}): ReactElement => {
  const classes = useStyles()
  const theme: Theme = useTheme()
  const isBigScreen = useMediaQuery(theme.breakpoints.up('sm'))

  return (
    <div className={classes.root}>
      {!isBigScreen && (
        <div
          style={{
            position: 'absolute',
            top: '10px',
            right: '20px',
            cursor: 'pointer'
          }}
          onClick={() => onSidebarClose(false)}
        >
          <CloseIcon />
        </div>
      )}
      <div className={classes.titleBox}>
        <Typography variant='h5'>Search for your laptop</Typography>
      </div>
      {filtersData &&
        filters &&
        !loading &&
        Object.keys(filtersData).map(key => {
          return (
            <SelectUI
              key={filtersData[key].attribute}
              filterType={filtersData[key]}
              filter={filters[key]}
              onFilterChange={onFilterChange}
            />
          )
        })}

      {loading && (
        <>
          <SkeletonTheme color='#dcdcdc' highlightColor='#eee'>
            <Skeleton width={150} height={10} duration={3} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
            <Skeleton width={150} height={10} />
            <Skeleton style={{ marginBottom: '1rem' }} />
          </SkeletonTheme>
        </>
      )}
    </div>
  )
}

export default Sidebar
