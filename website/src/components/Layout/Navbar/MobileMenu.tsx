import { makeStyles } from '@material-ui/core/styles'
import NavLinks from './NavLinks'

const useStyles = makeStyles(theme => ({
  root: {
    width: '100%',
    position: 'absolute',
    top: '55px',
    left: '0px',
    backgroundColor: theme.dark.main,
    color: theme.light.main,
    margin: '0px',
    padding: '0px',
    listStyleType: 'none',
    zIndex: 999,

    '& li': {
      '&:not(:last-child)': {
        marginBottom: theme.spacing(1)
      },
      '& a': {
        '&:link': {
          color: theme.light.main,
          fontSize: '1.1em',
          textDecoration: 'none',
          display: 'block',
          padding: theme.spacing(1, 2, 1, 2)
        },
        '&:visited': {
          color: theme.light.main,
          fontSize: '1.1em',
          textDecoration: 'none'
        },
        '&:hover': {
          color: theme.light.main,
          fontSize: '1.1em',
          textDecoration: 'none'
        }
      }
    }
  }
}))

interface IProps {
  open: boolean
  toggleMenu: () => void
}

const MobileMenu: React.FunctionComponent<IProps> = (
  props
): React.ReactElement => {
  const classes = useStyles()
  const { open, toggleMenu } = props
  return (
    <>
      {open && (
        <ul className={classes.root}>
          <NavLinks toggleMenu={toggleMenu} isMobile />
        </ul>
      )}
    </>
  )
}

export default MobileMenu
