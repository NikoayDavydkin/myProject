import React from 'react'
import Select from 'react-select'

const reactSelectStyles = {
  control: theme => ({
    ...theme,
    color: '#333',
    fontSize: '14px',
    outline: 'none',
    padding: '0px 0px',
    boxSizing: 'border-box',
    width: '200px',
    backgroundColor: '#fff',
    border: '1px solid #212121',
    borderRadius: 'none',
    boxShadow: 'none',
    '&:hover': {
      border: '1px solid #212121'
    },

    '&:active': {
      border: '1px solid #212121'
    }
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
    backgroundColor: isFocused ? '#212121' : 'transparent',
    cursor: isDisabled ? 'not-allowed' : 'default',
    ':active': {
      backgroundColor: '#212121'
    }
  })
}

const SingleSelect = ({ options, onChange, value, name }) => (
  <Select
    name={name}
    value={value}
    options={options}
    getOptionLabel={option => option.name}
    getOptionValue={option => option.id}
    onChange={onChange}
    styles={reactSelectStyles}
  />
)

export default SingleSelect
