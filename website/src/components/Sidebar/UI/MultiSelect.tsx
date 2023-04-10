import React, { FunctionComponent, ReactElement } from 'react'
import Select from 'react-select'
import FilterValueType from '../../../interfaces/filters/filterValueType'

const reactSelectStyles = {
  control: theme => ({
    ...theme,
    backgroundColor: '#fff',
    color: '#333',
    fontSize: '14px',
    border: '1px solid #333',
    outline: 'none',
    padding: '0px 0px',
    borderRadius: '2px',
    boxSizing: 'border-box',
    boxShadow: 'none'
  }),

  menu: theme => ({
    ...theme,
    border: '1px solid #aaa;',
    borderRadius: '4px',
    marginTop: '0px',
    borderTopLeftRadius: 0,
    borderTopRightRadius: 0
  }),

  option: (theme, { isDisabled, isFocused }) => ({
    ...theme,
    color: isFocused ? '#fff' : '#333',
    backgroundColor: isFocused ? '#F49827' : 'transparent',
    cursor: isDisabled ? 'not-allowed' : 'default',
    ':active': {
      backgroundColor: '#F49827'
    }
  }),

  multiValue: theme => ({
    ...theme,
    backgroundColor: '#F49827'
  }),

  multiValueLabel: styles => ({
    ...styles,
    color: '#fff'
  }),

  multiValueRemove: styles => ({
    ...styles,
    color: '#fff',
    ':hover': {
      color: '#487cef'
    }
  }),

  indicatorsContainer: styles => ({
    ...styles,
    display: 'none'
  })
}

interface IProps {
  options: string[]
  isMulti: boolean
  onChange: (values: FilterValueType) => void
  placeholder?: string
  name: string
  value?: (string | number)[]
}

const MultiSelect: FunctionComponent<IProps> = ({
  options,
  isMulti = true,
  onChange,
  placeholder = '',
  name,
  value = []
}): ReactElement => {
  const selectOptions = options.map(opt => ({
    label: opt,
    value: opt
  }))

  let newValues: { label: string; value: string }[] = []
  if (value && value.length >= 1) {
    newValues = value.map((v: string) => ({
      label: v,
      value: v
    }))
  }
  return (
    <Select
      isMulti={isMulti}
      name={name}
      placeholder={placeholder}
      value={newValues}
      options={selectOptions}
      getOptionLabel={option => option.label}
      getOptionValue={option => option.value}
      onChange={values => {
        if (!values) return onChange(null)
        if (Array.isArray(values))
          return onChange(values.map(value => value.value))
        return onChange(null)
      }}
      styles={reactSelectStyles}
    />
  )
}

export default MultiSelect
