import { FunctionComponent, ReactElement } from 'react'
import DoneIcon from '@material-ui/icons/Done'
import Typography from '@material-ui/core/Typography'
import Button from '@material-ui/core/Button'
import useStyles from '../src/styles/thanks'
import Router from 'next/router'

const Thanks: FunctionComponent = (): ReactElement => {
  const classes = useStyles()

  return (
    <div className={classes.root}>
      <div className={classes.iconBox}>
        <DoneIcon />
      </div>
      <Typography variant='h3'>Thank you</Typography>
      <Typography variant='h4' className={classes.bodySec}>
        We Received Your Request.
      </Typography>
      <Typography variant='body1' className={classes.body}>
        One of our experts will contact you as soon as possible.
      </Typography>
      <Button
        variant='contained'
        color='primary'
        disableElevation
        onClick={() => Router.replace('/')}
      >
        Go to Home
      </Button>
    </div>
  )
}

export default Thanks
