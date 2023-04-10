import { makeStyles, Theme } from '@material-ui/core/styles'

const useStyles = makeStyles((theme: Theme) => ({
  main: {
    width: '70%',
    marginTop: theme.spacing(3),
    marginLeft: 'auto',
    marginRight: 'auto',
    ['@media (max-width: 950px)']: {
      width: '90%'
    },
    ['@media (max-width: 650px)']: {
      width: '100%'
    }
  },

  title: {
    fontSize: '3.7rem',
    textAlign: 'center',
    marginBottom: theme.spacing(3),
    color: '#333',
    fontWeight: 500,
    ['@media (max-width: 750px)']: {
      fontSize: '3rem'
    },
    ['@media (max-width: 650px)']: {
      fontSize: '2.7rem'
    },
    ['@media (max-width: 450px)']: {
      fontSize: '2rem'
    }
  }
}))

export default useStyles
