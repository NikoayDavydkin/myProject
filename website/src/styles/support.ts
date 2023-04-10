import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  root: {
    width: 'calc(100vw - 15px)',
    minHeight: 'calc(100vh - 137px)',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    padding: theme.spacing(3),
    ['@media (max-width: 650px)']: {
      width: '100%'
    }
  },
  card: {
    padding: theme.spacing(3),
    width: '500px',
    ['@media (max-width: 650px)']: {
      width: '100%'
    }
  },

  inputBox: {
    '&:not(:last-child)': {
      marginBottom: theme.spacing(3)
    }
  },

  titleBox: {
    marginBottom: theme.spacing(3),
    textAlign: 'center'
  }
}))

export default useStyles
