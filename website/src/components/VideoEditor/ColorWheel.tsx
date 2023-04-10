import React, {
  FunctionComponent,
  ReactElement,
  useState,
  useEffect,
  useRef
} from 'react'
import { withStyles } from '@material-ui/core/styles'
import Grid from '@material-ui/core/Grid'
import Slider from '@material-ui/core/Slider'

import useStyles from '../../../src/styles/color-wheel'
import { HSVtoRGB } from '../../../src/utils/colors'

const CustomSlider = withStyles({
  root: {
    color: '#F49827',
    height: 10,
    width: '86%',
    marginLeft: '16px'
  },
  thumb: {
    height: '1.2em',
    width: '1.5em',
    borderRadius: '3px',
    backgroundColor: '#fff',
    border: '1px solid #fff',
    '&:hover': {
      backgroundColor: '#487cef',
      opacit: 1,
      boxShadow: 'none'
    }
  },
  rail: {
    opacity: 1,
    height: 10
  },
  mark: {
    display: 'none'
  }
})(Slider)

interface ColorWheelSliderState {
  label: string
  value: number
}

const ColorWheelSlider: FunctionComponent<ColorWheelSliderState> = ({
  label,
  value = 0,
}): ReactElement => {
  const classes = useStyles()

  const [state, setState] = useState<ColorWheelSliderState>({
    label,
    value,
  })

  return (
    <div>
      <div style={{ display: 'flex'}}>
        <label
          style={{
            display: 'block',
            marginLeft: '10px',
            color: '#fff'
          }}
        >
          {state.label + ': '}
        </label>
        <form /*onSubmit={submitHandler}*/>
          <input
            value={state.value}
            className={classes.sliderInput}
            /*onChange={event => setPrice(event.target.value)}*/
          >
          </input>
        </form>
      </div>
      <CustomSlider
        onChange={(e, value) => {
          let newState = state
          newState.value = value as number
          setState(newState)
        }}
      />
    </div>
  )
}

 interface ColorWheelState {
  hue: number
  saturation: number
  value: number
  x: number
  y: number
  red: number
  green: number
  blue: number
}


const ColorWheel: FunctionComponent<ColorWheelState> = ({
  hue = 0,
  saturation = 0,
  value = 0,
  x = 0,
  y = 0,
  red = 0,
  blue = 0,
  green = 0
}): ReactElement => {
  const classes = useStyles()

  const canvasRef = useRef(null)

  const [state, setState] = useState<ColorWheelState>({
    hue,
    saturation,
    value,
    x,
    y,
    red,
    green,
    blue
  })

  const draw = () => {
    const canvas = canvasRef.current
    const context = canvas.getContext('2d')
    canvas.width = canvas.clientWidth
    canvas.height = canvas.clientHeight
    const imageData = context.createImageData(canvas.width, canvas.height)
    console.log(canvas.width)
    console.log(canvas.height)
    const data = imageData.data
    for (var x = 0; x < canvas.width; x++) {
      for (var y = 0; y < canvas.height; y++) {
        let x_scaled = (x - canvas.width / 2) / (canvas.width / 2)
        let y_scaled = (y - canvas.height / 2) / (canvas.height / 2)
        let radius = Math.sqrt(x_scaled * x_scaled + y_scaled * y_scaled)
        let theta = Math.atan2(x_scaled, y_scaled)
        let degrees = ((theta * 180) / Math.PI + 180) % 360
        let { r, g, b } = HSVtoRGB(degrees, radius, 1)
        data[y * canvas.width * 4 + x * 4] = r
        data[y * canvas.width * 4 + x * 4 + 1] = g
        data[y * canvas.width * 4 + x * 4 + 2] = b
        let alpha = 255
        let pixels = (radius * canvas.width) / 2 - canvas.width / 2
        if (pixels >= -1) {
          if (pixels <= 1) {
            alpha = Math.round(255 * (1 - (pixels + 1) / 2))
          } else {
            alpha = 0
          }
        }
        data[y * canvas.width * 4 + x * 4 + 3] = alpha
      }
    }
    context.putImageData(imageData, 0, 0)
  }

  useEffect(() => {
    draw()
    canvasRef.current.addEventListener('resize', draw)
    return () => canvasRef.current.removeEventListener('resize', draw)
  })

  const size = 300

  return (
    <Grid container spacing={2} className={classes.colorWheelBox}>
      <Grid item xs={12}>
        <div className={classes.colorWheelCanvasBox}>
          <canvas ref={canvasRef} className={classes.colorWheelCanvas} />
          <div
            className={classes.colorWheelPicker}
            style={{
              top: y * size,
              left: x * size,
              width: size / 15,
              height: size / 15,
              border: `${size / 150}px solid black`
            }}
          />
        </div>
      </Grid>
      <Grid item xs={12}>
        <Grid container>
          <Grid item xs={12}>
            <ColorWheelSlider label={"Hue"} value={0}/>
          </Grid>
          <Grid item xs={12}>
            <ColorWheelSlider label={"Saturation"} value={0}/>
          </Grid>
          <Grid item xs={12}>
            <ColorWheelSlider label={"Value"} value={0}/>
          </Grid>
          <Grid item xs={12}>
            <ColorWheelSlider label={"Red"} value={0}/>
          </Grid>
          <Grid item xs={12}>
            <ColorWheelSlider label={"Green"} value={0}/>
          </Grid>
          <Grid item xs={12}>
            <ColorWheelSlider label={"Blue"} value={0}/>
          </Grid>
        </Grid>
      </Grid>
    </Grid>
  )
}

export default ColorWheel
