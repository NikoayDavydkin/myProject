import {  ApolloProvider } from '@apollo/client';
import Grid from '@material-ui/core/Grid/Grid';
import React from 'react'
import { Nav, Navbar } from 'react-bootstrap';
import RegexTable from '../../src/components/RegexTable';
import RegexClient from '../../src/client/RegexClient';
import { useRouter } from 'next/router';

import 'bootstrap/dist/css/bootstrap.min.css'





export const Regexes = () => {
    const router = useRouter();
    
    return (
        <ApolloProvider client={RegexClient}>
        <div style={{backgroundColor: '#343a40', minHeight:"800px", width:"100%"}}>
            <Navbar collapseOnSelect expand="lg" bg="light" variant="light">
                <Navbar.Brand>REGEX ADMIN</Navbar.Brand>
                <Navbar.Toggle aria-controls="responsive-navbar-nav" />
                <Navbar.Collapse id="responsive-navbar-nav">
                    
                    <Nav>
                    <Nav.Link onClick={() => router.push("/admin/cpuParsing")}>Cpu Tests</Nav.Link>
                    <Nav.Link onClick={() => router.replace("/admin/regexes")}>Regex</Nav.Link>
                    </Nav>
                </Navbar.Collapse>
            </Navbar>
            
            <Grid container style={{padding:20}}>
                
                     <RegexTable /> 
            </Grid>

        </div>
        </ApolloProvider>
    )
}

export default Regexes;