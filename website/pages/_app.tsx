import { useEffect, useState } from 'react'
import type { AppProps } from 'next/app'
import CssBaseline from '@material-ui/core/CssBaseline'
import { ThemeProvider } from '@material-ui/core/styles'
import theme from '../src/theme'
import { Navbar, Footer } from '../src/components/Layout'
import NProgress from 'nprogress'
import './global.scss'
import 'nprogress/nprogress.css'
import { Router } from 'next/router'

NProgress.configure({
  showSpinner: false
})

const App: React.FunctionComponent<AppProps> = ({
  Component,
  pageProps
}: AppProps): React.ReactElement => {
  const [searchingProducts, setSearchingProducts] = useState<boolean>(true)
  const [searchQuery, setSearchQuery] = useState<string>('')

  const searchingProductsToggle = (value: boolean) => {
    setSearchingProducts(value)
  }

  const updateQuery = (value: string) => {
    setSearchQuery(value)
  }

  useEffect(() => {
    Router.events.on('routeChangeStart', () => {
      NProgress.start()
    })

    Router.events.on('routeChangeComplete', () => {
      NProgress.done()
    })

    Router.events.on('routeChangeError', () => {
      NProgress.done()
    })
    return () => {
      Router.events.off('routeChangeStart', () => {
        NProgress.done()
      })

      Router.events.off('routeChangeComplete', () => {
        NProgress.done()
      })

      Router.events.off('routeChangeError', () => {
        NProgress.done()
      })
    }
  }, [])
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Navbar
        setSearchingProducts={searchingProductsToggle}
        getQuery={updateQuery}
      />
      <main style={{ paddingTop: '57px' }}>
        <Component
          {...pageProps}
          searchingProducts={searchingProducts}
          setSearchingProducts={searchingProductsToggle}
          searchQuery={searchQuery}
        />
      </main>
      <Footer />
    </ThemeProvider>
  )
}

export default App
