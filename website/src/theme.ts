// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { Theme } from '@material-ui/core/styles/createMuiTheme'
import { createMuiTheme, ThemeOptions } from '@material-ui/core/styles'

declare module '@material-ui/core/styles/createMuiTheme' {
  interface Theme {
    light: {
      main: React.CSSProperties['color']
      main500: React.CSSProperties['color']
      main400: React.CSSProperties['color']
    }
    dark: {
      main: React.CSSProperties['color']
      main100: React.CSSProperties['color']
      main600: React.CSSProperties['color']
      main700: React.CSSProperties['color']
      main500: React.CSSProperties['color']
    }
    mainColor: {
      main: React.CSSProperties['color']
    }
    secondryColor: {
      main: React.CSSProperties['color']
    }
  }
  // allow configuration using `createMuiTheme`
  interface ThemeOptions {
    light?: {
      main: React.CSSProperties['color']
      main400: React.CSSProperties['color']
      main500: React.CSSProperties['color']
    }
    dark?: {
      main: React.CSSProperties['color']
      main100: React.CSSProperties['color']
      main700: React.CSSProperties['color']
      main600: React.CSSProperties['color']
      main500: React.CSSProperties['color']
    }
    mainColor?: {
      main: React.CSSProperties['color']
    }
    secondryColor?: {
      main: React.CSSProperties['color']
    }
  }
}

function createMyTheme(options: ThemeOptions) {
  return createMuiTheme({
    ...options
  })
}

const theme = createMyTheme({
  palette: {
    primary: {
      main: '#487cef'
    },
    secondary: {
      main: '#e2641e'
    }
  },
  light: {
    main: '#d7d7d7',
    main500: '#DFE1E5',
    main400: '#F2F2F2'
  },
  dark: {
    main: '#212121',
    main100: '#9AA0A6',
    main700: '#333',
    main600: '#5F6368',
    main500: '#777777'
  },
  mainColor: {
    main: '#487cef'
  },

  secondryColor: {
    main: '#e2641e'
  }
})

export default theme
