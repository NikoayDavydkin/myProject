import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  footer: {
    width: '100%',
    height: '55px',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: theme.dark.main,
    color: theme.light.main,
    marginTop: '25px'
  },

  copy: {
    fontSize: '1em',
    marginRight: '3px'
  },

  logo: {
    height: '25px',
    ['@media (max-width: 490px)']: {
      height: '20px'
    }
  }
}))

export default useStyles
