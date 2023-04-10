import React, { useEffect, useState } from 'react'
import {Button, Table} from 'react-bootstrap'
import 'bootstrap/dist/css/bootstrap.min.css';
import { gql, useLazyQuery, useMutation, useQuery } from '@apollo/client';
import Loader from 'react-loader-spinner';
import { useRouter } from 'next/router';
import { useCookies } from 'react-cookie';
import { v4 as uuidv4 } from 'uuid';
import RegexModal from '../RegexModal';
import AdminClient from '../../client/AdimClient';
import DeleteModal from '../DeleteModal';



interface Fields {

  name: string| null;
  regex: string | null;
  replacement: string | null
}
interface CpuTest {
  cpu: string
  count: number
  matches: {
    regex:{
      id: string
      stringPattern:string
      fields:Fields[]
    },
    product: {
      attributes :{
        key: string,
        name: string,
        value:[string]
      }[]
    }
  }[]
}


const GET_CPU_TEST = gql`
  query {
    
    cpuParsingTests {
      cpu
      count
      matches {
        regex {
          id
          stringPattern
          fields {
            name
            regex
            replacement
          }
        }
        product {
          attributes {
            key
            name
            value
          }
        }
      }
    }
  }

`

const DELETE_REGEX = gql`
  mutation($id:ID!, $token:ID!){
    deleteRegex(id:$id, token:$token) {
      id
    }
  }
`

interface State{
  show: boolean
  regexId: string
  regexList: Fields[]
}
const CPUTable = () => {
  const router = useRouter();
  const [cookies] = useCookies(['token']);
  const [shouldExecute, executeQuery] = useState(true)
  const [modalState, setModalState] = useState<State>({
    show: false,
    regexId:'',
    regexList:[]
  })

  const [deleteModalState, setDeleteModal] = useState(false);
  const [deleteId, setDeleteId] = useState('');
  const [deleteRegexData] = useMutation(DELETE_REGEX);
  const [getCPUList,{loading , data, error, refetch}] = useLazyQuery(GET_CPU_TEST,{
    fetchPolicy:"network-only"
  });
  const [cpuParsingTestList, setCpuParsingTestList] = useState<CpuTest[]>([]);
  const addRegexFields = () => {
      setModalState(state => {
        return {
          show: true,
          regexId: uuidv4(),
          regexList:[]
        }
      })
  }
  const handleDelete = async (id:string) => {
    try {
      
      await deleteRegexData({variables:{id, token:localStorage.getItem('token')}});
    
      await refetch();
      closeDeleteModal();
      executeQuery(true);
      
    }
    catch(e) {
      console.log(e);
    }
  };

  const editRegexFields =  (regexId: string, newFieldsList: Fields[]) => {
    setModalState(state => {
      return {
        show: true,
        regexId: regexId,
        regexList:[...newFieldsList]
      }
    })
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
  const handleSave = () => {
    handleClose();
    refetch();
    executeQuery(true);
  }
  const closeDeleteModal =  () => {
    setDeleteModal(false);
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
      }
    ).catch(e => router.replace('/admin'));
  },[])
  useEffect(() => {
    const getRegexes= async () => {
      try {
        await getCPUList();
      }
      catch(e) {
      }
    }
    getRegexes();
  },[executeQuery])
  if(loading) {
    return( 
      <Loader
        type="ThreeDots"
        color="white"
        height={100}
        width={100}
        timeout={3000} //3 secs

    />
    );
  }
  
  if(error) {
    return (
      <div>Error</div>
    )
  }
  if(data && shouldExecute) {
    const {cpuParsingTests} = data;
    setCpuParsingTestList(cpuParsingTests)
    executeQuery(false);
  }
    return (
        <div style={{width:"100%"}}>
            <Button variant="primary" style={{margin: "1% 40% 1% 40%"}} onClick={addRegexFields}>Add Regex</Button>
            {modalState.show ? <RegexModal 
                  handleSave = {handleSave}
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
            <Table striped bordered hover variant="dark" style={{width:"100%", borderRadius: "10px", outline:"none"}}>
                <thead>
                    <tr >
                        <th rowSpan={2} >CPU</th>
                        <th rowSpan={2}>Count</th>
                        <th rowSpan={2}>RegexID</th>
                        <th colSpan={3}>Regex Fields</th>
                        <th rowSpan={2} colSpan={2}>Operation</th>
                        <th colSpan={3}>Product</th>
                    </tr>
                    <tr>
                        <th>Name</th>
                        <th>Regex</th>
                        <th>Replacement</th>
                        <th>Name</th>
                        <th>Value</th>
                    </tr>
                    
                </thead>
                <tbody>
                    {cpuParsingTestList.map((cpuParsingTest,cpuIndex) => {
                        
                        return  cpuParsingTest.matches.length ? cpuParsingTest.matches.map((cpuMatch) => {

                            const {regex: {fields:regexFields}, product:{attributes:productAttributes}} = cpuMatch;

                            if ( regexFields.length > productAttributes.length) {
                              return regexFields.map((regexField, index) => (
                                
                                <tr key={cpuMatch.regex.id+regexField.regex + regexField.replacement + regexField.name}>
                                  {index === 0 ? <td rowSpan={regexFields.length} >{cpuParsingTest.cpu}</td>: null}
                                  {index === 0 ? <td rowSpan={regexFields.length}>{cpuParsingTest.count}</td>: null}
                                  {index === 0 ? <td rowSpan={regexFields.length}>{ cpuMatch.regex.id}</td>: null}

                                  <td>{regexField.name}</td>
                                  <td>{regexField.regex}</td>
                                  <td>{regexField.replacement}</td>
                                  {index === 0 ? <td rowSpan={regexFields.length}>
                                    <Button variant="primary"
                                        onClick={() => editRegexFields(cpuMatch.regex.id ,regexFields)}
                                    
                                    >Edit</Button>
                                  </td>: null}
                                  
                                  {index === 0 ? <td rowSpan={regexFields.length}>
                                    <Button 
                                      variant="danger"
                                      onClick={() => openDeleteModal(cpuMatch.regex.id)}
                                    >Delete</Button>
                                  </td>: null}
                                  <td>{productAttributes[index] !== undefined ? productAttributes[index].name : ''}</td>
                                  <td>{productAttributes[index] !== undefined ? productAttributes[index].value : ''}</td>

                                </tr>
                                )
                              )
                            }
                            else if(productAttributes.length!==0){
                              return productAttributes.map((productAttribute, index) => (
                                <tr>
                                    {index === 0 ? <td rowSpan={productAttributes.length} >{cpuParsingTest.cpu}</td>: null}
                                    {index === 0 ? <td rowSpan={productAttributes.length}>{cpuParsingTest.count}</td>: null}
                                    {index === 0 ? <td rowSpan={productAttributes.length}>{cpuMatch.regex.id}</td>: null}

                                    <td>{regexFields[index]!== undefined ? regexFields[index].name : ''}</td>
                                    <td>{regexFields[index]!== undefined ? regexFields[index].regex : ''}</td>
                                    <td>{regexFields[index]!== undefined ? regexFields[index].replacement : ''}</td>
                                    {index === 0 ? <td rowSpan={productAttributes.length}>
                                      <Button variant="primary"
                                        onClick={() => editRegexFields(cpuMatch.regex.id,regexFields)}
                                      
                                      >Edit</Button>
                                    </td>: null}
                                    {index === 0 ? <td rowSpan={productAttributes.length}>
                                      <Button 
                                        variant="danger"
                                        onClick={() => openDeleteModal(cpuMatch.regex.id)}
                                      >Delete</Button>
                                    </td>: null}
                                    <td>{productAttribute.name}</td>
                                    <td>{productAttribute.value[0]}</td>

                                </tr>
                              ))
                            }
                            else {
                              return (
                                <tr>
                                  <td >{cpuParsingTest.cpu}</td>
                                  <td >{cpuParsingTest.count}</td>
                                  <td >{cpuMatch.regex.id}</td>
                                  <td>''</td>
                                  <td>''</td>
                                  <td> ''</td>
                                  <td>
                                  <Button variant="primary"
                                        onClick={() => editRegexFields(cpuMatch.regex.id,regexFields)}
                                      
                                  >Edit</Button>
                                  </td>
                                  <td>
                                  <Button 
                                      variant="danger"
                                      onClick={() => openDeleteModal(cpuMatch.regex.id)}
                                    >Delete</Button>
                                  </td>
                                  <td>'</td>
                                  <td>''</td>
                                  <td>''</td>
                                </tr>
                              )
                            }
                            
                        })
                        : (
                          <tr>
                            <td >{cpuParsingTest.cpu}</td>
                            <td>{cpuParsingTest.count}</td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                          </tr>
                        )
                    })}
                </tbody>
            </Table>
        </div>
    )
}

export default CPUTable;
