import IFilter from '../interfaces/filters/filters'
import IFetchedFilter from '../interfaces/filters/fetchedFilter'
import FILTER_TYPES from '../interfaces/filters/filterTypes'
import { stringify as uuidStringify, parse } from 'uuid'

const getAttributes = (
  filters: { [key: string]: IFilter },
  filtersData: { [key: string]: IFetchedFilter }
): IFilter[] => {
  const attributes = []
  const names = Object.keys(filters)
  names.forEach(key => {
    const filter = filters[key]
    if (filter.type === FILTER_TYPES.multiSelect) {
      if (filter.value) {
        attributes.push({
          attribute: uuidStringify(parse(filter.attribute)),
          values: filter.value
        })
      }
    } else if (
      filter.type === FILTER_TYPES.minMaxSlider ||
      filter.type === FILTER_TYPES.minMaxContinousSlider ||
      filter.type === FILTER_TYPES.minMaxContinousPriceSlider
    ) {
      const sliderMinValue = Number(filtersData[filter.name].attributeValues[0])
      const sliderMaxValue = Number(
        filtersData[filter.name].attributeValues[
          filtersData[filter.name].attributeValues.length - 1
        ]
      )

      if (
        filter.value[0] !== sliderMinValue ||
        filter.value[1] !== sliderMaxValue
      ) {
        const filterObj = {
          attribute: uuidStringify(parse(filter.attribute))
        }

        if (filter.value[0] !== sliderMinValue) {
          filterObj['min'] = filter.value[0]
        }

        if (filter.value[1] !== sliderMaxValue) {
          filterObj['max'] = filter.value[1]
        }

        attributes.push(filterObj)
      }
    } else if (filter.type === FILTER_TYPES.minMaxStringSlider) {
      const sliderMinValue = filtersData[filter.name].attributeValues[0]
      const sliderMaxValue =
        filtersData[filter.name].attributeValues[
          filtersData[filter.name].attributeValues.length - 1
        ]
      if (
        filter.value[0] !== sliderMinValue ||
        filter.value[1] !== sliderMaxValue
      ) {
        const filterObj = {
          attribute: uuidStringify(parse(filter.attribute))
        }

        if (filter.value[0] !== sliderMinValue) {
          filterObj['min'] = filter.value[0]
        }

        if (filter.value[1] !== sliderMaxValue) {
          filterObj['max'] = filter.value[1]
        }

        attributes.push(filterObj)
      }
    }
  })

  return attributes
}

export default getAttributes
