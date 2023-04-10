import getAttributes from '../utils/getAttributes'
import IFilter from '../interfaces/filters/filters'
import IFetchedFilter from '../interfaces/filters/fetchedFilter'

const getURLDisplayAttributes = (
  filters: { [key: string]: IFilter },
  filtersData: { [key: string]: IFetchedFilter }
): any => {
  const attributes = getAttributes(filters, filtersData)
  const attributesObj = {}
  attributes.forEach(attribute => {
    attributesObj[attribute.attribute] = { ...attribute, attribute: undefined }
  })

  return attributesObj
}

export default getURLDisplayAttributes
