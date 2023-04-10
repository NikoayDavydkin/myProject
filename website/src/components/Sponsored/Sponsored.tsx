import React from 'react';
import { withStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core/styles';
import Button from '@material-ui/core/Button';
import Tooltip from '@material-ui/core/Tooltip';
import {useState} from 'react'





const HtmlTooltip = withStyles((theme: Theme) => ({
  tooltip: {
    backgroundColor: '#f5f5f9',
    color: 'rgba(0, 0, 0, 0.87)',
    maxWidth: 300,
    fontSize: theme.typography.pxToRem(15),
    border: '1px solid #dadde9',
  },
}))(Tooltip);

export default function Sponsored() {

  const [state, setState] = useState(false)


  return (
    <div>
      <HtmlTooltip
        title={state ? 
          <React.Fragment>
            As an Amazon Associate, DealTech earns from qualifying purchases. DealTech may recieve a commission if you purchase something through one of our links. Certain content that appears on DealTech comes from Amazon. This content is provided ‘as is’ and is subject to change or removal at any time.
          </React.Fragment> : null
        }
      >
        <Button onClick = {() => {setState(state ? false : true)}}>
         <div>Sponsored</div> 
         <i className="fas fa-briefcase"></i>
         <img style = {{width : '17px', margin: '3px'}} src = {'info.png'}/>
         </Button>
      </HtmlTooltip>
    </div>
  );
}
