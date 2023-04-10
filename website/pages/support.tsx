import { useState } from 'react'
import { Formik } from 'formik'
import * as Yup from 'yup'
import TextField from '@material-ui/core/TextField'
import Card from '@material-ui/core/Card'
import Button from '@material-ui/core/Button'
import useStyles from '../src/styles/support'
import Typography from '@material-ui/core/Typography'
import CircularProgress from '@material-ui/core/CircularProgress'
import Email from '../src/lib/smtp'
import Link from '@material-ui/core/Link'
import Router from 'next/router'

const Support: React.FunctionComponent = (): React.ReactElement => {
  const [loading, setLoading] = useState<boolean>(false)
  const classes = useStyles()

  const initialValues = {
    name: '',
    email: '',
    message: ''
  }

  const validationSchema = Yup.object({
    name: Yup.string().required('name is required'),
    email: Yup.string()
      .email('invalid email address')
      .required('email is required'),
    message: Yup.string()
      .min(8, 'message must be at least 8 characters long')
      .required('message is required')
  })

  const handleSubmit = values => {
    const { name, email, message } = values
    setLoading(true)
    Email.send({
      Host: 'smtp.gmail.com',
      Username: 'dealtech64',
      Password: 'support@dealtech',
      To: 'support@dealtech.com',
      From: 'dealtech64@gmail.com',
      Subject: `${name} sent you a message`,
      Body: `Name: ${name} <br /> Email: ${email} <br /> Message: ${message}`
    }).then(message => {
      if (message === 'OK') {
        setLoading(false)
        Router.push('/form-fill-thanks')
      }
    })
  }

  return (
    <div className={classes.root}>
      <Card className={classes.card}>
        <div className={classes.titleBox}>
          <Typography variant='h5'>Get in Touch</Typography>
          {/*<Typography variant='body2' style={{ marginTop: '10px' }}>
            Have an inquiry or some feedback for us? Fill out the form below to
            contact our team.
          </Typography>*/}
          <Typography variant='body2' style={{ marginTop: '10px' }}>
            Have an inquiry or some feedback for us?
          </Typography>
        </div>
        <Formik
          initialValues={initialValues}
          validationSchema={validationSchema}
          onSubmit={handleSubmit}
        >
          {({
            handleBlur,
            handleChange,
            values,
            errors,
            touched,
            handleSubmit
          }) => (
            <form onSubmit={handleSubmit}>
            {/*<div className={classes.inputBox}>
                <TextField
                  size='small'
                  value={values.name}
                  onChange={handleChange}
                  onBlur={handleBlur}
                  name='name'
                  label='Your Name'
                  helperText={
                    !!touched.name && !!errors.name ? errors.name : ''
                  }
                  error={!!touched.name && !!errors.name}
                  fullWidth
                  variant='outlined'
                />
              </div>
              <div className={classes.inputBox}>
                <TextField
                  size='small'
                  value={values.email}
                  onChange={handleChange}
                  onBlur={handleBlur}
                  name='email'
                  label='Your Email'
                  helperText={
                    !!touched.email && !!errors.email ? errors.email : ''
                  }
                  error={!!touched.email && !!errors.email}
                  fullWidth
                  variant='outlined'
                />
              </div>
              <div className={classes.inputBox}>
                <TextField
                  size='small'
                  value={values.message}
                  onChange={handleChange}
                  onBlur={handleBlur}
                  name='message'
                  label='Your Message'
                  helperText={
                    !!touched.message && !!errors.message ? errors.message : ''
                  }
                  error={!!touched.message && !!errors.message}
                  multiline
                  rows={6}
                  fullWidth
                  variant='outlined'
                />
              </div>
              <div className={classes.inputBox}>
                <Button
                  variant='contained'
                  color='primary'
                  fullWidth
                  type='submit'
                  disabled={
                    !!errors.message ||
                    !!errors.email ||
                    !!errors.name ||
                    values.name === '' ||
                    values.email === '' ||
                    values.message === '' ||
                    loading
                  }
                >
                  {loading && (
                    <CircularProgress
                      size={14}
                      style={{ marginRight: '5px' }}
                    />
                  )}
                  Submit
                </Button>
              </div>*/}
              <Typography>
                Email us at |{' '}
                <Link href='mailto:support@dealtech.com'>
                  support@dealtech.com
                </Link>
              </Typography>
              <Typography>
                Or write us |{' '}
                  P.O. Box 50802, Mesa AZ, 85208
              </Typography>
            </form>
          )}
        </Formik>
      </Card>
    </div>
  )
}

export default Support
