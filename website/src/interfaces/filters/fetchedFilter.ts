import FILTER_TYPES from './filterTypes'

interface fetchedFilter {
  attribute: string
  name: string
  attributeValues: string[]
  units: string
  filterType: FILTER_TYPES
}

export default fetchedFilter
