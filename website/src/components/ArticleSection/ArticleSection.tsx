import { FunctionComponent, ReactElement } from 'react'
import ReactPlayer from 'react-player/youtube'
import ReactMarkdown from 'markdown-to-jsx'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheck, faTimesOctagon } from '@fortawesome/pro-solid-svg-icons'
import Button from '@material-ui/core/Button'
import Grid from '@material-ui/core/Grid'

import classes from './articleSection.module.scss'
import IArticleBody from '../../interfaces/articleDetailBody'
import CONTENTTYPES from '../../interfaces/contentTypes'
import ISource from '../../interfaces/sources'

interface IProps {
  section?: IArticleBody
  sources?: ISource[]
}

const ArticleSection: FunctionComponent<IProps> = ({
  section,
  sources
}): ReactElement => {
  if (section) {
    let content: ReactElement = null
    if (section.__typename === CONTENTTYPES.Video) {
      content = (
        <div className={classes.videoBox}>
          <ReactPlayer
            width='100%'
            height='100%'
            url={section.url}
            className={classes.player}
          />
        </div>
      )
    } else if (section.__typename === CONTENTTYPES.RichParagraph) {
      let imageSrc = section.imageUrl.startsWith('http')
        ? section.imageUrl
        : process.env.NEXT_PUBLIC_IMAGE + section.imageUrl
      content = (
        <div className={classes.detailBox}>
          <div className={classes.titleBox}>
            <img
              src={imageSrc}
            />
            <h2 className={classes.secondryHeading}>{section.title}</h2>
          </div>

          <ReactMarkdown
            options={{
              overrides: {
                h2: {
                  props: {
                    className: classes.secondryHeading
                  }
                },
                h3: {
                  props: {
                    className: classes.thirdHeading
                  }
                },
                ul: {
                  props: {
                    className: classes.articleUl
                  }
                },
                p: {
                  props: {
                    className: classes.body
                  }
                },
                span: {
                  props: {
                    className: classes.body
                  }
                }
              }
            }}
          >
            {section.richText}
          </ReactMarkdown>
          <Grid container spacing={1} className={classes.prosConsBox}>
            <Grid item md={6}>
              <ul style={{ listStyleType: 'none' }}>
                {section.pros.map((text, index) => (
                  <li key={index}>
                    <FontAwesomeIcon
                      icon={faCheck}
                      size='lg'
                      style={{ color: 'green' }}
                    />
                    &nbsp;{text}
                  </li>
                ))}
              </ul>
            </Grid>
            <Grid item md={6}>
              <ul style={{ listStyleType: 'none' }}>
                {section.cons.map((text, index) => (
                  <li key={index}>
                    <FontAwesomeIcon
                      icon={faTimesOctagon}
                      size='lg'
                      style={{ color: 'red' }}
                    />
                    &nbsp;{text}
                  </li>
                ))}
              </ul>
            </Grid>
          </Grid>
          <Button
            variant='contained'
            color='primary'
            fullWidth
            href={section.link}
            target='_blank'
          >
            View on Amazon
          </Button>
        </div>
      )
    } else if (CONTENTTYPES.RichText) {
      content = (
        <div>
          <ReactMarkdown
            options={{
              overrides: {
                h2: {
                  props: {
                    className: classes.secondryHeading
                  }
                },
                h3: {
                  props: {
                    className: classes.thirdHeading
                  }
                },
                ul: {
                  props: {
                    className: classes.articleUl
                  }
                },
                p: {
                  props: {
                    className: classes.body
                  }
                }
              }
            }}
          >
            {section.text}
          </ReactMarkdown>
        </div>
      )
    }
    if (null == content) {
      content = <div>ERROR</div>
    }
    return <div>{content}</div>
  } else if (sources) {
    return (
      <div className={classes.detailBox}>
        <div className={classes.titleBox}>
          <h2>Sources</h2>
        </div>

        {sources.map((source, index) => (
          <p key={index} className={classes.source}>
            {source.author}
            {source.date && <> ({source.date})</>}
            {source.title && <> {source.title}</>}
            {source.link && <a href={source.link}> {source.link}</a>}
          </p>
        ))}
      </div>
    )
  }
}

export default ArticleSection
