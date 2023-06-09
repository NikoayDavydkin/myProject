import { ApolloClient, createHttpLink, gql, InMemoryCache } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';

const httpLink = createHttpLink({
    uri: process.env.NEXT_PUBLIC_BONFIRE,
  });
  
const authLink = setContext((_, { headers }) => {
    // get the authentication token from local storage if it exists
    
    // return the headers to the context so httpLink can read them
    return {
      headers: {
        ...headers,
      }
    }
});
  
const AuthClient = new ApolloClient({
    link: authLink.concat(httpLink),
    cache: new InMemoryCache()
});
export default AuthClient;
