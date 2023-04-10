import React, {
  FunctionComponent,
  ReactElement,
  useState,
  useEffect
} from 'react'
import Slider from '@material-ui/core/Slider'
import { withStyles } from '@material-ui/core/styles'
import FilterValueType from '../../../interfaces/filters/filterValueType'

const CustomeSlider = withStyles({
  root: {
    color: '#F49827',
    padding: '0px',
    height: 10,
    width: '93%'
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
  active: {
    backgroundColor: '#285f8f',
    boxShadow: 'none'
  },
  track: {
    height: 10
  },
  rail: {
    color: '#80b4e6',
    opacity: 1,
    height: 10,
    borderRadius: '4px'
  },
  mark: {
    display: 'none'
  }
})(Slider)

interface IProps {
  value: FilterValueType
  min: number
  max: number
  marks?: string[]
  onChange: (value: FilterValueType) => void
  id: string
  name?: string
  unit?: string
}

const MinMaxSlider: FunctionComponent<IProps> = ({
  value,
  min,
  max,
  marks,
  onChange,
  id,
  name,
  unit
}): ReactElement => {
  const [state, setState] = useState<FilterValueType>(value)
  useEffect(() => {
    setState(value)
  }, [value])

  const markValues = []
  const values = []
  if (marks) {
    const markMultiplier = Math.ceil(100 / marks.length)
    marks.forEach((mark, index) => {
      markValues.push({
        value: (index + 1) * markMultiplier,
        mark: Number(mark)
      })
    })

    markValues.forEach(mark => {
      if (state.length >= 1 && mark.mark === state[0]) {
        values[0] = mark.value
      }
    })

    markValues.forEach(mark => {
      if (state.length >= 1 && mark.mark === state[1]) {
        values[1] = mark.value
      }
    })
  }

  return (
    <>
      <label
        style={{
          marginBottom: '2px',
          display: 'block'
        }}
      >
        {name && (
          <>
            {name}: {state.length >= 0 ? state[0] : ''} -{' '}
            {state.length >= 0 ? state[1] : ''} {unit && <>{unit}</>}
          </>
        )}
      </label>
      <div style={{ marginLeft: '7px' }}>
        {marks ? (
          <CustomeSlider
            value={values as number[]}
            id={id}
            onChange={(e, values) => {
              e.preventDefault()
              const selectedValues = []
              markValues.forEach(mark => {
                if (mark.value === values[0]) selectedValues[0] = mark.mark
              })
              markValues.forEach(mark => {
                if (mark.value === values[1]) selectedValues[1] = mark.mark
              })
              setState(selectedValues)
              // onChange(selectedValues)
            }}
            onChangeCommitted={() => {
              onChange(state)
            }}
            marks={markValues}
            min={markValues[0].value}
            max={markValues[markValues.length - 1].value}
            step={null}
          />
        ) : (
          <CustomeSlider
            value={state as number[]}
            id={id}
            onChange={(e, values: number[]) => {
              setState(values)
            }}
            min={min}
            max={max}
            onChangeCommitted={() => {
              onChange(state)
            }}
          />
        )}
      </div>
    </>
  )
}

export default MinMaxSlider
