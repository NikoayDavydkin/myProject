import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  root: {
    minHeight: 'calc(100vh - 137px)',
    paddingTop: theme.spacing(3),
    paddingLeft: theme.spacing(3),
    paddingRight: theme.spacing(3),
  },

  videoEditorBox: {
    height: 'calc(100vh - 161px)',
  },

  sideBar: {
    width: '16%',
    float: 'left'
  },

  editorBox: {
    width: '84%',
    height: '100%',
    float: 'left',
    paddingLeft: theme.spacing(2)
  },

  viewItemBox: {
    background: 'url(images/color_wheel.svg)',
    backgroundSize: 'contain',
    backgroundRepeat: 'no-repeat',
    flex: 'auto'
  },

  viewerItem: {
    width: '50%',
    height: '50%',
    float: 'left',
    background: 'url(images/color_wheel.svg)',
    backgroundSize: 'contain',
    backgroundRepeat: 'no-repeat',
  },

  timelineItemBox: {
    width: '100%',
    height: '50%',
    float: 'left',
    paddingTop: theme.spacing(1)
  },

  timelineItem: {
    width: '100%',
    height: '100%',
    background: theme.mainColor.main
  }
}))

export default useStyles
