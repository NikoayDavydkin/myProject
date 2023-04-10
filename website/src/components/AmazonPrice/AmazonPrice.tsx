import React from 'react'
import { withStyles } from '@material-ui/core/styles'
import { Theme } from '@material-ui/core/styles'
import Button from '@material-ui/core/Button'
import Tooltip from '@material-ui/core/Tooltip'
import { useState } from 'react'
import IOffer from '../../interfaces/offer'
import classes from './amazonPrice.module.scss'

interface IProps {
  offer: IOffer
  updated: string
}

const AmazonPrice: React.FunctionComponent<IProps> = ({
  offer,
  updated
}): React.ReactElement => {
  if (null === offer || null === updated) {
    return <span>Price not available</span>
  } else {
    var elapsed = 'forever'
    let duration = (Date.now() - Date.parse(updated)) / 1000
    if (duration < 60) {
      elapsed = '<1 min'
    } else if (duration == 60) {
      elapsed = '1 min'
    } else if (duration < 60 * 60) {
      elapsed = `${Math.trunc(duration / 60)} minutes`
    } else if (duration < 120 * 60) {
      elapsed = '1 hour'
    } else {
      elapsed = `${Math.trunc(duration / 60 / 60)} hours`
    }
    return (
      <div className={classes.priceBox}>
        <p className={classes.price}>
          {`$${(Number(offer.price + offer.shipping) / 100).toFixed(2)}`}
        </p>
        <p className={classes.updated}>{`as of ${elapsed} ago`}</p>
        <div className={classes.link}>
          {'More Info'}
          <span className={classes.tooltip}>
            {
              'Product prices and availability are accurate as of the date/time indicated and are subject to change. Any price and availability information displayed on Amazon at the time of purchase will apply to the purchase of this product.'
            }
          </span>
        </div>
      </div>
    )
  }
}

export default AmazonPrice
