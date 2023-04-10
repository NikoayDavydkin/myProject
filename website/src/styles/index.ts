import { makeStyles, Theme } from '@material-ui/core/styles'

const useStyles = makeStyles((theme: Theme) => ({
  root: {
    backgroundColor: theme.palette.background.default,
    width: '100%',
    height: 'calc(100vh - 112px)',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center'
  },

  centered: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    width: '100%'
  },

  logoBox: {
    marginBottom: theme.spacing(3.5),
    ['@media (max-width: 490px)']: {
      marginBottom: theme.spacing(2.5)
    },

    '& img': {
      height: '8rem',
      ['@media (max-width: 650px)']: {
        height: '7rem'
      },
      ['@media (max-width: 490px)']: {
        height: '6rem'
      }
    }
  },

  searchBox: {
    width: '584px',
    height: '46px',
    psotion: 'relative',
    display: 'flex',
    alignItems: 'center',
    ['@media (max-width: 650px)']: {
      width: 'calc(100% - 40px)'
    }
  },

  searchIconBox: {
    position: 'absolute',
    backgroundColor: theme.palette.background.default,
    margin: 0,
    padding: 0,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    marginLeft: '13px'
  },

  searchIcon: {
    width: '20px',
    height: '20px',
    fill: theme.dark.main100
  },

  inputBox: {
    height: '100%',
    width: '100%',
    '& input': {
      display: 'block',
      width: '100%',
      height: '100%',
      backgroundColor: theme.palette.background.default,
      border: `1px solid ${theme.light.main500}`,
      borderRadius: '24px',
      outline: 'none',
      padding: '0px 40px',
      fontSize: '16px',
      fontFamily: 'Roboto',
      color: theme.dark.main700,

      '&:hover': {
        boxShadow: '0 1px 6px rgba(32,33,36,.28)',
        borderColor: 'rgba(223,225,229,0)'
      },

      '&:focus': {
        boxShadow: '0 1px 6px rgba(32,33,36,.28)',
        borderColor: 'rgba(223,225,229,0)'
      },

      ['@media (max-width: 650px)']: {
        fontSize: '14px'
      }
    }
  },

  buttonGroup: {
    marginTop: theme.spacing(3),
    display: 'flex'
  },

  googleButton: {
    width: '127px',
    height: '36px',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    borderRadius: '4px',
    backgroundColor: theme.light.main400,
    border: '1px solid',
    borderColor: theme.light.main400,
    color: theme.dark.main600,
    textTransform: 'capitalize',
    fontSize: '16px',
    fontWeight: 400,

    '&:hover': {
      boxShadow: '0 1px 1px rgba(0,0,0,0.1)',
      backgroundImage: '-webkit-linear-gradient(top,#f8f8f8,#f1f1f1)',
      backgroundColor: '#f8f8f8',
      border: '1px solid #c6c6c6'
    }
  },

  dialogPaper: {
    width: '820px',
    height: '450px',
    position: 'relative',
    overflowY: 'hidden',
    margin: '0px',

    ['@media(max-width: 690px)']: {
      width: '100vw',
      height: '30vh'
    },
    ['@media(min-width: 1440px)']: {
      width: '960px',
      height: '530px'
    }
  },

  player: {
    position: 'absolute',
    top: '0px',
    left: '0px',
    width: '100%',
    height: '100%',
    overflowY: 'hidden'
  }
}))

export default useStyles
