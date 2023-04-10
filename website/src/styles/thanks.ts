import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  root: {
    minHeight: 'calc(100vh - 137px)',
    wwidth: '100%',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    flexDirection: 'column',
    textAlign: 'center'
  },

  iconBox: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    padding: theme.spacing(2),
    border: `1px solid ${theme.palette.primary.main}`,
    borderRadius: '100%',
    marginBottom: theme.spacing(2),

    '& svg': {
      color: theme.palette.primary.main,
      fontSize: '45px'
    }
  },

  body: {
    marginTop: theme.spacing(1.5),
    marginBottom: theme.spacing(2.5)
  },
  bodySec: {
    marginTop: '5px'
  }
}))

export default useStyles
