import axios from 'axios'

const redditBot = axios.create({
  baseURL: process.env.NEXT_PUBLIC_REDDIT_BOT
})

export default redditBot
