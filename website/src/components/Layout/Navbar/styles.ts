import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  nav: {
    backgroundColor: theme.dark.main,
    width: '100%',
    height: '55px',
    color: theme.light.main,
    position: 'fixed',
    top: 0,
    left: 0,
    zIndex: 999,

    '&::after': {
      content: '""',
      display: 'block',
      width: '100%',
      height: '3px',
      backgroundColor: theme.mainColor.main,
      position: 'absolute',
      bottom: 0,
      left: 0
    }
  },

  navLogo: {
    display: 'flex',
    height: '55px',
    alignItems: 'center',

    '& img': {
      height: '25px',
      ['@media (max-width: 490px)']: {
        height: '20px'
      }
    }
  },

  navInner: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center'
  },

  navbarNavDesktop: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '55px',
    listStyle: 'none',
    '& li': {
      position: 'relative',
      '&:not(:last-child)': {
        marginRight: theme.spacing(3)
      },

      '& a': {
        color: theme.light.main,
        textDecoration: 'none',
        fontSize: '1.1em',
        transition: 'all 0.3s',

        '&:hover': {
          color: theme.mainColor.main
        }
      }
    }
  },

  activeDesktop: {
    '&::after': {
      content: '""',
      display: 'block',
      width: '100%',
      height: '2px',
      backgroundColor: theme.mainColor.main,
      position: 'absolute'
    }
  },

  activeMobile: {
    backgroundColor: theme.mainColor.main
  },

  menuIconBox: {
    height: '55px',
    display: 'flex',
    alignItems: 'center'
  },

  menuIcon: {
    color: theme.light.main,
    fontSize: '34px'
  }
}))

export default useStyles
