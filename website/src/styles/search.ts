import { makeStyles, Theme } from '@material-ui/core/styles'

const useStyles = makeStyles((theme: Theme) => ({
  title: {
    margin: 0,
    lineHeight: 1.15
  },

  main: {
    overflowX: 'hidden',
    padding: '30px 2%'
  },

  link: {
    textDecoration: 'none',
    color: '#f50057',
    '&:hover': {
      textDecoration: 'underline'
    }
  },

  control: {
    padding: theme.spacing(2)
  },

  ProductPhoto: {
    overflow: 'hidden',
    marginBottom: '10px'
  },

  ProductPhotoImage: {
    width: '100%',
    height: '175px',
    backgroundSize: 'contain',
    backgroundRepeat: 'no-repeat',
    backgroundPosition: 'top center'
  },

  ButtonGroup: {
    marginTop: theme.spacing(2)
  },

  OfferGroup: {
    display: 'flex',
    flexDirection: 'row',
    gap: '10px',
  },

  Button: {
    borderRadius: '25px'
  },

  ProductDetails: {
    padding: theme.spacing(2)
  },

  SearchInfoBar: {
    marginBottom: '10px',
    marginLeft: 'auto',
    display: 'flex',
    alignItems: 'center',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
    ['@media (max-width: 430px)']: {
      flexDirection: 'column-reverse',
      alignItems: 'flex-end'
    }
  }
}))

export default useStyles
