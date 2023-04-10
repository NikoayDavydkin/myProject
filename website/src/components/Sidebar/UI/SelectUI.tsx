import { FunctionComponent, ReactElement } from 'react'
import {} from '@material-ui/core/Input'
import { makeStyles } from '@material-ui/styles'
import MinMaxSlider from './MinMaxSlider'
import MinMaxStringSlider from './MinMaxStringSlider'
import MultiSelect from './MultiSelect'
import PriceSlider from './PriceSlider'
import { Theme } from '@material-ui/core'
import IFetchedFilter from '../../../interfaces/filters/fetchedFilter'
import IFilter from '../../../interfaces/filters/filters'
import FilterValueType from '../../../interfaces/filters/filterValueType'
import FILTER_TYPES from '../../../interfaces/filters/filterTypes'

const useStyles = makeStyles((theme: Theme) => ({
  inputGroup: {
    marginBottom: theme.spacing(3)
  },

  label: {
    marginBottom: '2px',
    display: 'block'
  }
}))

interface IProps {
  filterType: IFetchedFilter
  filter: IFilter
  onFilterChange: (value: IFilter) => void
}

const SelectUI: FunctionComponent<IProps> = ({
  filterType,
  filter,
  onFilterChange
}): ReactElement => {
  const classes = useStyles()
  return (
    <div className={classes.inputGroup}>
      {filterType.filterType === FILTER_TYPES.multiSelect && (
        <>
          <label className={classes.label}>
            {filterType.name} {filter.unit && <>{filter.unit}</>}
          </label>

          <MultiSelect
            isMulti
            options={filterType.attributeValues}
            name={filterType.name}
            value={filter.value}
            onChange={(values: FilterValueType) => {
              onFilterChange({
                ...filter,
                value: values
              })
            }}
            placeholder='select ...'
          />
        </>
      )}

      {filterType.filterType === FILTER_TYPES.minMaxSlider && (
        <MinMaxSlider
          name={filter.name}
          unit={filter.unit}
          id={filterType.attribute}
          value={filter.value}
          marks={filterType.attributeValues}
          min={Number(filterType.attributeValues[0])}
          max={Number(
            filterType.attributeValues[filterType.attributeValues.length - 1]
          )}
          onChange={(values: FilterValueType) => {
            onFilterChange({
              ...filter,
              value: values
            })
          }}
        />
      )}

      {filterType.filterType === FILTER_TYPES.minMaxContinousSlider && (
        <MinMaxSlider
          name={filter.name}
          unit={filter.unit}
          id={filterType.attribute}
          value={filter.value}
          min={Number(filterType.attributeValues[0])}
          max={Number(
            filterType.attributeValues[filterType.attributeValues.length - 1]
          )}
          onChange={(values: number[]) => {
            onFilterChange({
              ...filter,
              value: values
            })
          }}
        />
      )}

      {filterType.filterType === FILTER_TYPES.minMaxStringSlider && (
        <MinMaxStringSlider
          name={filter.name}
          unit={filter.unit}
          id={filterType.attribute}
          value={filter.value}
          min={0}
          max={10 * (filterType.attributeValues.length - 1)}
          marks={filterType.attributeValues}
          onChange={(e, values) => {
            onFilterChange({
              ...filter,
              value: values
            })
          }}
        />
      )}

      {filterType.filterType === FILTER_TYPES.minMaxContinousPriceSlider && (
        <PriceSlider
          filter={filter}
          filterType={filterType}
          onChange={(values: number[]) => {
            onFilterChange({
              ...filter,
              value: values
            })
          }}
          value={filter.value}
        />
      )}
    </div>
  )
}

export default SelectUI
