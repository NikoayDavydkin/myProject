import useStyles from './styles'
import { useRouter } from 'next/router'

const Footer: React.FunctionComponent = (): React.ReactElement => {
  const classes = useStyles()
  const router = useRouter()
  const { pathname } = router
  return (
    <footer className={classes.footer}>
      <div className={classes.copy}>Powered by </div>
      <img className={classes.logo} alt='dealTech logo' src='/logoSmall.svg' />
    </footer>
  )
}

export default Footer
