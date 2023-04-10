import { FunctionComponent, ReactElement } from 'react'
import Card from '@material-ui/core/Card'
import Typography from '@material-ui/core/Typography'
import HomeIcon from '@material-ui/icons/Home'
import Button from '@material-ui/core/Button'
import Router from 'next/router'
import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  root: {
    backgroundColor: theme.palette.background.default,
    width: '100%',
    height: 'calc(100vh - 137px)',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center'
  },

  card: {
    padding: theme.spacing(3),
    textAlign: 'center'
  },

  bigHead: {
    fontSize: '15rem',
    lineHeight: '100%',
    '& span': {
      color: theme.palette.primary.main
    },
    fontWeight: 500,
    ['@media (max-width: 600px)']: {
      fontSize: '8rem'
    }
  },

  smallHead: {
    fontSize: '4rem',
    fontWeight: 400,
    marginBottom: '1rem',
    ['@media (max-width: 600px)']: {
      fontSize: '2rem'
    }
  },

  body: {
    fontSize: '1.6rem',
    fontWeight: 300,
    marginBottom: theme.spacing(4)
  }
}))

const NotFound: FunctionComponent = (): ReactElement => {
  const classes = useStyles()
  return (
    <div className={classes.root}>
      <Card className={classes.card}>
        <Typography variant='h1' component='p' className={classes.bigHead}>
          4<span>0</span>4
        </Typography>
        <Typography variant='h1' component='p' className={classes.smallHead}>
          Oops!!
        </Typography>
        <Typography variant='body1' component='p' className={classes.body}>
          The page doesn&apos;t exist or is unavailable.
        </Typography>
        <Button
          variant='contained'
          color='primary'
          disableElevation
          size='large'
          startIcon={<HomeIcon />}
          onClick={() => Router.replace('/')}
        >
          Go Back To Home
        </Button>
      </Card>
    </div>
  )
}

export default NotFound
