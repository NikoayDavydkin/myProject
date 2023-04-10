import { useState } from 'react'
import useMediaQuery from '@material-ui/core/useMediaQuery'
import MenuIcon from '@material-ui/icons/Menu'
import Link from 'next/link'
import { useRouter } from 'next/router'
import MobileMenu from './MobileMenu'
import NavLinks from './NavLinks'
import Searchbar from '../../Searchbar/Searchbar'
import useStyles from './styles'

interface IProps {
  setSearchingProducts: (value: boolean) => void
  getQuery: (value: string) => void
}

const Navbar: React.FunctionComponent<IProps> = ({
  setSearchingProducts,
  getQuery
}): React.ReactElement => {
  const [openMenu, setOpenMenu] = useState<boolean>(false)
  const classes = useStyles()
  const { pathname } = useRouter()
  const isMobile = useMediaQuery('(max-width: 650px)')

  const toggleOpenMenu = () => {
    setOpenMenu(!openMenu)
  }

  return (
    <nav className={classes.nav}>
      <div className='wrapper'>
        <div className={classes.navInner}>
          <div className={classes.navLogo}>
            <Link href='/'>
              <a>
                <img src='/logoSmall.svg' alt='dealtech logo' />
              </a>
            </Link>
          </div>
          <Searchbar
            setSearchingProducts={setSearchingProducts}
            getQuery={getQuery}
          />
          {isMobile && (
            <>
              <div className={classes.menuIconBox}>
                <MenuIcon
                  className={classes.menuIcon}
                  onClick={toggleOpenMenu}
                />
              </div>
              <MobileMenu open={openMenu} toggleMenu={toggleOpenMenu} />
            </>
          )}
          {!isMobile && (
            <ul className={classes.navbarNavDesktop}>
              <NavLinks />
            </ul>
          )}
        </div>
      </div>
    </nav>
  )
}

export default Navbar
