import { gql, useMutation } from '@apollo/client';
import { useRouter } from 'next/router';
import React, { useState } from 'react'
import { Button, Form } from 'react-bootstrap'


const LOGIN_USER = gql`
   mutation loginUser($userName: String!, $password:String!) {
    login(input:{identifier:$userName, password:$password}) {
      jwt
      user{
        username
        id
        email
      }
    }
  }
` 

const LoginForm: React.FC = () => {
  const router = useRouter();
  const [ loginUser, {data, error}] = useMutation(LOGIN_USER);
    const [userName, setUserName] = useState('');
    const [password, setPassword] = useState('');

    const handleUserName = (event: React.ChangeEvent<HTMLInputElement>) => {
        setUserName(event.target.value);

    }
    const handlePassword = (event: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(event.target.value);
    }

    const handleSubmit = async (event: React.FormEvent<HTMLElement>)=> {
        event.preventDefault();
        try {

          await loginUser({variables: {
            userName :userName,
            password: password
          }});

        }
        catch(e) {
          console.log('Error ' , e);
        }
      
    }
    if(data)  {
      const {login} = data;
      localStorage.setItem('token', login.jwt);
      router.push('/admin/cpuParsing');
    }

    if(error) {
    }
    return (
        <div style={{ padding:"310px 30%"}}>
          <div style={{ width: 300, margin:"0 20%"}}>
            <Form onSubmit={handleSubmit}>
              <Form.Group controlId='formBasicEmail'>
                <Form.Label>Username</Form.Label>
                <Form.Control 
                    type='text' 
                    placeholder='Enter email' 
                    value={userName}
                    onChange={handleUserName}
                />
              </Form.Group>
    
              <Form.Group controlId='formBasicPassword'>
                <Form.Label>Password</Form.Label>
                <Form.Control type='password' placeholder='Password' value={password}
                onChange={handlePassword}
                
                />
              </Form.Group>
    
              <Button 
                variant='success' 
                type='submit'
                >
                Log In
              </Button>
            </Form>
          </div>
        </div>
      )
}

export default LoginForm;
