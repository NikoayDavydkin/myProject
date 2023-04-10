import { makeStyles } from '@material-ui/core/styles'

const useStyles = makeStyles(theme => ({
  colorWheelBox: {
    background: theme.mainColor.main
  },

  colorWheelCanvasBox: {
    backgroundColor: '#fff'
  },

  colorWheelCanvas: {
    cursor: 'pointer',
    zindex: '1',
    height: '100%',
    width: '100%',
    aspectRatio: '1'
  },

  colorWheelPicker: {
    borderRadius: '100%',
    backgroundColor: 'white'
  },

  sliderInput: {
    display: 'inline-block',
    width: '30px',
    margin: '0px 0px 0px 3px',
    color: '#333',
    fontSize: '12px',
    border: '1px solid #fff',
    outline: 'none',
    padding: '2px 3px',
    borderRadius: '2px',
    boxSizing: 'border-box'
  },
}))

export default useStyles
