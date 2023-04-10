import {  gql, useLazyQuery, useMutation, useQuery } from '@apollo/client'
import { Delete } from '@material-ui/icons';
import { useRouter } from 'next/router';
import React, { useEffect, useState } from 'react'
import { Button, Table } from 'react-bootstrap'
import context from 'react-bootstrap/esm/AccordionContext';
import { useCookies } from 'react-cookie';
import Loader from 'react-loader-spinner';
import { v4 as uuidv4 } from 'uuid';
import AdminClient from '../../client/AdimClient';
import DeleteModal from '../DeleteModal';
import RegexModal from '../RegexModal';

const GET_REGEXES = gql `
  query {
    regexes {
      id
      fields {
        name
        regex
        replacement
      }
    }
  }
`
interface Fields {

  name: string| null;
  regex: string | null;
  replacement: string | null
}

interface Regex {
  id: string,
  fields: {
    name: string | null;
    regex: string | null;
    replacement: string | null
  }[]
}
interface State{
  show: boolean
  regexId: string
  regexList: Fields[]
}
const DELETE_REGEX = gql`
  mutation($id:ID!, $token:ID!){
    deleteRegex(id:$id, token:$token) {
      id
    }
  }
`
const RegexTable:React.FC = () => {
    
    const  [getRegexesList, {loading, data, error}]= useLazyQuery(GET_REGEXES,{
      fetchPolicy: "network-only"
    } ) ;
    const router = useRouter();
    const [cookies] = useCookies(['token']);
    const [shouldExecute, executeQuery] = useState(true)
    const [deleteRegexData] = useMutation(DELETE_REGEX);
    const [deleteModalState, setDeleteModal] = useState(false);
    const [deleteId, setDeleteId] = useState('');
    const [modalState, setModalState] = useState<State>({
      show: false,
      regexId:'',
      regexList:[]
    })
    const [regexList, setRegexList] = useState<Regex[]>([]);
    const [toggleQuery, setToggleQuery] = useState(true);

    const addRegexFields = () => {
      setModalState(state => {
        return {
          show: true,
          regexId: uuidv4(),
          regexList:[]
        }
      })
    }
    const editRegexFields =  (regexId: string, newFieldsList: Fields[]) => {
      setModalState(state => {
        return {
          show: true,
          regexId: regexId,
          regexList:[...newFieldsList]
        }
      })
    }

    const handleDelete = async (id:string) => {
      try {
        await deleteRegexData({variables:{id, token:localStorage.getItem('token')}});
        setRegexList(regexList => regexList.filter(regex => regex.id !== id));
        setDeleteModal(false);
      }
      catch(e) {
        console.log(e);
      }
    };
    const closeDeleteModal =  () => {
      setDeleteModal(false);
    }
    const handleClose =  () => {
      setModalState(state => {
        return {
          ...state,
          show: false,
        }
      })
      
    }

    const openDeleteModal = async (id: string) =>{
      await setDeleteId(id);
      await setDeleteModal(true)
    }


    const handleSave  = () => {
      handleClose()
      executeQuery(true);
      setToggleQuery(toggle => !toggle);
    }

    useEffect(() => {
      AdminClient.query(
        {
          query: gql`
            query {
              me {
                id
              }
            }
          `
        },
      ).catch(e => router.replace('/admin'));
    },[])
    useEffect(() => {
      const getRegexes= async () => {
        try {
          await getRegexesList();
        }
        catch(e) {

        }
      }
      getRegexes();
    },[toggleQuery])
    if(loading) {
      return (
        <Loader
         type="ThreeDots"
         color="white"
         height={100}
         width={100}
         timeout={3000} //3 secs
 
      />
      )
    }
    if(error) {
      return (
        <div>Error</div>
      )
    }
    if(data && shouldExecute) {
      const {regexes} = data;
      setRegexList(regexes);
      executeQuery(false);
    }
    return (
        <div style={{width:"100%"}}>
            <Button variant="primary" style={{margin: "1% 40% 1% 40%"}} onClick={addRegexFields}>Add Regex</Button>
            {modalState.show ? <RegexModal 
                  handleSave={handleSave}
                  show={modalState.show} 
                  handleClose={handleClose} 
                  regexId={modalState.regexId}
                  regexList={modalState.regexList}
              /> : null}
              <DeleteModal 
                show={deleteModalState}
                handleClose={closeDeleteModal}
                id={deleteId} 
                handleDelete = {handleDelete}             
              />
            <Table striped bordered hover variant="dark" style={{width:"100%", borderRadius: "10px", outline:"none", }}>
                <thead>
                    <tr >
                        <th  rowSpan={2}>Id</th>
                        <th colSpan={3}>Fields</th>
                        <th  colSpan={2} rowSpan={2}>Operation</th>
                    </tr>
                    <tr>
                        <th>Name</th>
                        <th>Regex</th>
                        <th>Replacement</th>
                        
                    </tr>
                </thead >
                <tbody>
                   {regexList.map((regex, regexIndex) => {
                       const {fields} = regex;
                       return fields.length ? fields.map((field, index) => (
                           <tr key={regex.id + field.name + field.replacement + field.regex }>
                                {index === 0 ? <td rowSpan={fields.length} >{regex.id}</td> : null}
                                <td>{field.name}</td>
                                <td>{field.regex}</td>
                                <td>{field.replacement?.replace('"', '')}</td>
                                {index === 0 ? 
                                    <td rowSpan={fields.length}>
                                      <Button variant="primary" 
                                       onClick={() => editRegexFields(regex.id ,fields)}
                                      >
                                        Edit
                                      </Button></td>: null}
                                {index === 0 ? 
                                    <td rowSpan={fields.length}>
                                      <Button variant="danger" 
                                        onClick={() => openDeleteModal(regex.id)}
                                      >Delete</Button>
                                    </td>: null}

                            </tr>
                       )): 
                       (
                         <tr>
                           <td>{regex.id}</td>
                           <td></td>
                           <td></td>
                           <td></td>
                           <td>
                              <Button variant="primary" 
                                  onClick={() => editRegexFields(regex.id ,[])}
                              >
                                Edit
                              </Button>
                           </td>
                           <td>
                              <Button variant="danger" 
                                onClick={() => handleDelete(regex.id)}
                              >Delete</Button>
                           </td>
                         </tr>
                       )
                   })}
                </tbody>
            </Table>
        </div>
    )
}

export default RegexTable
