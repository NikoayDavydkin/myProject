import { FunctionComponent, ReactElement, useState, useEffect } from 'react'
import { makeStyles, withStyles } from '@material-ui/core/styles'
import IFilter from '../../../interfaces/filters/filters'
import IFetchedFilter from '../../../interfaces/filters/fetchedFilter'
import FilterValueType from '../../../interfaces/filters/filterValueType'
import Slider from '@material-ui/core/Slider'

const useStyles = makeStyles(() => ({
  label: {
    marginBottom: '2px',
    display: 'block'
  },

  priceInput: {
    display: 'inline-block',
    width: '73px',
    margin: '0px 0px 0px 3px',
    color: '#333',
    fontSize: '12px',
    border: '1px solid #fff',
    outline: 'none',
    padding: '2px 3px',
    borderRadius: '2px',
    boxSizing: 'border-box'
  }
}))

const CustomSlider = withStyles({
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
  filter: IFilter
  filterType: IFetchedFilter
  onChange: (value: number[]) => void
  value: FilterValueType
}

const PriceSlider: FunctionComponent<IProps> = ({
  filter,
  filterType,
  onChange,
  value
}): ReactElement => {
  const classes = useStyles()
  const [state, setState] = useState<FilterValueType>(value)
  useEffect(() => {
    setState(value)
  }, [value])

  const power = 6
  const minValue = Number(filterType.attributeValues[0])
  const maxValue = Number(
    filterType.attributeValues[filterType.attributeValues.length - 1]
  )
  const step = 1 / (maxValue - minValue)

  const transform = (value: number): number => {
    return Math.round(
      ((Math.exp((power * value) / maxValue) - 1) / (Math.exp(power) - 1)) *
      maxValue
    )
  }

  const reverse = (value: number): number => {
    return (
      (1 / power) *
      Math.log(((Math.exp(power) - 1) * value) / maxValue + 1) *
      maxValue
    )
  }

  let submitHandler = (event) => {
    event.preventDefault()
    onChange([Number(state[0]), Number(state[1])])
  }

  return (
    <>
      <div style={{ display: 'flex', marginBottom: '5px' }}>
        <label className={classes.label} style={{ marginRight: '5px' }}>
          {filter.name}:
        </label>
        <div style={{ display: 'flex' }}>
          <div>{filter.unit && filter.unit}</div>
          <form onSubmit={submitHandler}>
            <input
              type='numeric'
              value={state[0]}
              className={classes.priceInput}
              onChange={event => setState([event.target.value, state[1]])}
            >
            </input>
          </form>
        </div>
        <div style={{ margin: '0px 7px' }}>to</div>
        <div style={{ display: 'flex' }}>
          <div>{filter.unit && filter.unit}</div>
          <form onSubmit={submitHandler}>
            <input
              type='numeric'
              className={classes.priceInput}
              onChange={event => setState([state[0], event.target.value])}
              value={state[1]}
            >
            </input>
          </form>
        </div>
      </div>
      <CustomSlider
        value={[reverse(state[0] as number), reverse(state[1] as number)]}
        id='price-range-slider-1234'
        onChange={(e, values: number[]) => {
          setState([transform(values[0]), transform(values[1])])
        }}
        min={minValue}
        max={maxValue}
        step={step}
        onChangeCommitted={() => {
          onChange(state as number[])
        }}
      />
    </>
  )
}

export default PriceSlider
