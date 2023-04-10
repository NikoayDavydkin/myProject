import React, {
  FunctionComponent,
  ReactElement,
  useState,
  useEffect,
  useRef
} from 'react'
import { withStyles } from '@material-ui/core/styles'
import Slider from '@material-ui/core/Slider'

import useStyles from '../src/styles/video-editor'
import ColorWheel from '../src/components/VideoEditor/ColorWheel'

const VideoEditor: React.FunctionComponent = (): React.ReactElement => {
  const [loading, setLoading] = useState<boolean>(false)
  const classes = useStyles()

  const initialValues = {
    name: '',
    email: '',
    message: ''
  }

  return (
    <div className={classes.root}>
      <div className={classes.videoEditorBox}>
        <div className={classes.sideBar}>
          <ColorWheel
            hue={0}
            saturation={0}
            value={0}
            x={0}
            y={0}
            red={0}
            blue={0}
            green={0}
          />
        </div>
        <div className={classes.editorBox}>
          <div className={classes.viewerItem} />
          <div className={classes.viewerItem} />
          <div className={classes.timelineItemBox}>
            <div className={classes.timelineItem} />
          </div>
        </div>
      </div>
    </div>
  )
}

export default VideoEditor
