import { makeStyles, Theme } from '@material-ui/core/styles'

const useStyles = makeStyles((theme: Theme) => ({
  articles: {
    backgroundColor: '#F5F6FA'
  },

  header: {
    display: 'flex',
    justifyContent: 'flex-end',
    paddingTop: theme.spacing(3)
  },

  sortDropdown: {
    width: '320px',
    height: '35px',
    marginRight: '10px',
    display: 'flex',
    alignItems: 'center',
    '& span': {
      marginRight: theme.spacing(0.5)
    }
  },

  sortDropdown__comp: {
    width: '230px',
    marginRight: '10px'
  },

  searchWrapper: {
    width: '230px',
    height: '38px',
    background: 'red',
    display: 'flex',
    alignItems: 'center',
    position: 'relative'
  },

  searchIcon: {
    position: 'absolute',
    left: '13px',
    fill: theme.dark.main100,
    width: '20px',
    height: '20px'
  },

  searchInput: {
    display: 'block',
    width: '100%',
    height: '100%',
    padding: '0px 42px',
    border: `1px solid ${theme.dark.main}`,
    fontSize: '16px',
    color: theme.dark.main700,
    outline: 'none'
  },

  main: {
    marginTop: '30px'
  }
}))

export default useStyles
