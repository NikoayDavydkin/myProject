import Grid from '@material-ui/core/Grid/Grid';
import React, { useState } from 'react'
import { Button, Nav, Navbar } from 'react-bootstrap';
import CPUTable from '../CPUTable';
// import RegexModal from '../components/Table/RegexModal';
import RegexTable from '../RegexTable';
const pages = [
     "CPU TESTS",
     "REGEXES"
];
export const CPUParsing = () => {
    const [activePage, setActivePage] = useState("CPU TESTS")
    return (
        <div style={{backgroundColor: '#343a40', height:"100%", width:"100%"}}>
            <Navbar collapseOnSelect expand="lg" bg="light" variant="light">
                <Navbar.Brand>REGEX ADMIN</Navbar.Brand>
                <Navbar.Toggle aria-controls="responsive-navbar-nav" />
                <Navbar.Collapse id="responsive-navbar-nav">
                    
                    <Nav>
                    <Nav.Link onClick={() => setActivePage('CPU TESTS')}>Cpu Tests</Nav.Link>
                    <Nav.Link onClick={() => setActivePage('REGEXES')}>Regex</Nav.Link>
                    </Nav>
                </Navbar.Collapse>
            </Navbar>
            <Grid container style={{padding:20}}>
                
                        { activePage === pages[0] ? <CPUTable />: <RegexTable /> }
            </Grid>
        </div>
    )
}

export default CPUParsing;