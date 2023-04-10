import React, { useState } from 'react'
import 'bootstrap/dist/css/bootstrap.min.css'
import LoginForm from '../../src/components/LoginForm';
import {  ApolloProvider } from '@apollo/client';
import AuthClient from '../../src/client/AuthClient';




const Login:React.FC = () => {
    return (
        
        <ApolloProvider client={AuthClient}>
            <LoginForm />
        </ApolloProvider>
            

    )
}

export default Login
