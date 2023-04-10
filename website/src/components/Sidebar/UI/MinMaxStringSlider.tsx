import { ReactElement, FunctionComponent, useState, useEffect } from 'react'
import { withStyles } from '@material-ui/core/styles'
import Slider from '@material-ui/core/Slider'
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
  id: string
  value: FilterValueType
  min: number
  max: number
  onChange: (e: any, values: any) => void
  marks: string[]
  name: string
  unit: string
}

const MinMaxStringSlider: FunctionComponent<IProps> = ({
  id,
  value,
  min,
  max,
  onChange,
  marks,
  name,
  unit
}): ReactElement => {
  const [state, setState] = useState<FilterValueType>([])
  useEffect(() => {
    setState(value)
  }, [value])
  const markValues = []
  for (let i = 0; i < marks.length; i++) {
    markValues.push({
      value: i * 10,
      mark: marks[i]
    })
  }

  const values = []
  markValues.forEach(markValue => {
    if (markValue.mark === state[0]) {
      values[0] = markValue.value
    }
  })

  markValues.forEach(markValue => {
    if (markValue.mark === state[1]) {
      values[1] = markValue.value
    }
  })

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
        <CustomeSlider
          value={values}
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
          }}
          onChangeCommitted={e => {
            onChange(e, state)
          }}
          marks={markValues}
          min={min}
          max={max}
          step={null}
        />
      </div>
    </>
  )
}

export default MinMaxStringSlider
