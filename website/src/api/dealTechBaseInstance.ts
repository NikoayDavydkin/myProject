import axios from 'axios'

const dealtech = axios.create({
  baseURL: process.env.NEXT_PUBLIC_BONFIRE
})

export default dealtech
