import { makeStyles, Theme } from '@material-ui/core/styles'

const useStyles = makeStyles((theme: Theme) => ({
  searchBox: {
    width: '45%',
    height: '34px',
    psotion: 'relative',
    display: 'flex',
    alignItems: 'center',
    ['@media (max-width: 650px)']: {
      width: '100%',
      margin: '0px 30px'
    },
    ['@media (max-width: 550px)']: {
      margin: '0px 10px'
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
      borderRadius: '3px',
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

      ['@media (max-width: 950px)']: {
        fontSize: '12px'
      },
      ['@media (max-width: 650px)']: {
        fontSize: '11px'
      }
    }
  }
}))

export default useStyles
