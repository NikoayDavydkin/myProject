import { useRouter } from 'next/router'
import Link from 'next/link'
import useClasses from './styles'

interface IProps {
  isMobile?: boolean
  toggleMenu?: () => void
}

const NavLinks: React.FunctionComponent<IProps> = ({
  isMobile = false,
  toggleMenu
}): React.ReactElement => {
  const classes = useClasses()
  const router = useRouter()
  const { pathname } = router
  return (
    <>
      {!isMobile && (
        <>
          <li className={pathname === '/' ? classes.activeDesktop : undefined}>
            <Link href='/'>
              <a>Home</a>
            </Link>
          </li>
          <li
            className={pathname === '/articles' ? classes.activeDesktop : undefined}
          >
            <Link href='/articles'>
              <a>Articles</a>
            </Link>
          </li>
          <li
            className={
              pathname === '/support' ? classes.activeDesktop : undefined
            }
          >
            <Link href='/support'>
              <a>Support</a>
            </Link>
          </li>
        </>
      )}

      {isMobile && (
        <>
          <li>
            <Link href='/'>
              <a
                onClick={toggleMenu}
                className={pathname === '/' ? classes.activeMobile : undefined}
              >
                Home
              </a>
            </Link>
          </li>
          <li>
            <Link href='/articles'>
              <a
                onClick={toggleMenu}
                className={
                  pathname === '/articles' ? classes.activeMobile : undefined
                }
              >
                Articles
              </a>
            </Link>
          </li>
          <li>
            <Link href='/support'>
              <a
                onClick={toggleMenu}
                className={
                  pathname === '/support' ? classes.activeMobile : undefined
                }
              >
                Support
              </a>
            </Link>
          </li>
        </>
      )}
    </>
  )
}

export default NavLinks
