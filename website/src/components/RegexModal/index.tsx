import { gql, useLazyQuery, useMutation } from "@apollo/client";
import React , { useEffect, useState }  from "react";
import { Button, Modal, Table } from "react-bootstrap";
import { useCookies } from "react-cookie";

interface Regex {
    name: string| null;
    regex: string | null;
    replacement: string | null
}
interface Props   {
    handleClose: () => any
    show: boolean
    regexId: string
    regexList: Regex[],
    handleSave: () => void
}

const CREATE_REGEX = gql`
    mutation ($fields:[RegexFieldInput]!, $token:ID!, $id:ID!) {
        createRegex(fields:$fields,id:$id, token:$token){
            regex {
            stringPattern
            id
            }
        }
    }
`
const CHECK_REGEX = gql`
  query ($fields:[RegexFieldInput]!){
    regexStringPattern(fields: $fields) {
      regex
      error
    }
  }
`;
const RegexModal:React.FC<Props> = ({ show, handleClose, regexId, regexList, handleSave})  => {  
    const [regexes, setRegex] = useState<Regex[]>([]);
    const cookies = localStorage.getItem('token');
    useEffect(() => {
      (async ()  => {
        await setRegex([...regexList]);
        checkRegex();
      })();
        
    },[])
    const [createNewRegex] = useMutation(CREATE_REGEX);
    const [getRegexString,{data}] = useLazyQuery(CHECK_REGEX);
    const handleRegexNameChange = ( event:  React.ChangeEvent<HTMLInputElement>) => {
      var regex = regexes[event.target.name];
      regex  = {
          ...regex,
          name: event.target.value,
      }
      const index = event.target.name;
      setRegex(prevList => {
          const tempList = [...prevList];
          tempList[index] = regex;
          return tempList;
      })
    }
    const handleRegexChange = ( event:  React.ChangeEvent<HTMLInputElement>) => {
        var regex = regexes[event.target.name];
      regex  = {
          ...regex,
          regex: event.target.value,
      }
      const index = event.target.name;
      setRegex(prevList => {
          const tempList = [...prevList];
          tempList[index] = regex;
          return tempList;
      })
    }
    const handleRegexReplacementChange = ( event:  React.ChangeEvent<HTMLInputElement>) => {
        var regex = regexes[event.target.name];
      regex  = {
          ...regex,
          replacement : event.target.value,
      }
      const index = event.target.name;
      setRegex(prevList => {
          const tempList = [...prevList];
          tempList[index] = regex;
          return tempList;
      })
    }
    const addNewRegex = () => {
      setRegex(prevList => [...prevList,{name:'',regex:'',replacement:''}])
      
    }
    const removeRegex = (id: number ) => {
      setRegex(prevList => prevList.filter((regex, index) => {
        return (index !== id)
      }))
    }
   

    const checkRegex = async() => {
      try { 
        const customRegexList = regexes.map(regex => {
          return  { 
            name: regex.name ? regex.name: '',
            regex: regex.regex ? regex.regex : '',
            replacement: regex.replacement? regex.replacement : ''
          };
        
        });
        await getRegexString({variables:{fields:customRegexList}});
      }
      catch(e) {
      }
    }
    const saveRegex = async () => {
        if(regexes.length === 0) {
          return;
        }
        const customRegexList = regexes.map(regex => {
          return  { 
            name: regex.name ? regex.name: '',
            regex: regex.regex ? regex.regex : '',
            replacement: regex.replacement? regex.replacement : ''
          };
        
        });
        await createNewRegex({
            variables:{fields:customRegexList,id:regexId, token:cookies},
        })
        handleSave();
    }

    const moveUp = (index:number) => {
      if(index === 0) {
        return ;
      }
      const swapElement1 = regexes[index];
      const swapElement2 = regexes[index - 1];
      const tempList = [...regexes];
      tempList[index] = swapElement2;
      tempList[index-1] = swapElement1;
      setRegex(tempList);


    }

    const moveDown = (index:number) => {
      if(index === regexes.length - 1) {
        return;
      }
      const swapElement1 = regexes[index];
      const swapElement2 = regexes[index + 1];
      const tempList = [...regexes];
      tempList[index] = swapElement2;
      tempList[index + 1] = swapElement1;
      setRegex(tempList);
    }


    
    return (
      <>
        
        <Modal show={show} onHide={handleClose} size="xl">
          <Modal.Header closeButton>
            <Modal.Title>Add Edit Regex</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <p>RegexString :{data ? data.regexStringPattern.regex : ''}</p>
            <Table striped bordered hover  variant="light" >
              <tbody>
                <tr>
                  <td>ID</td>
                  <td>{regexId}</td>
                  <td colSpan={4}></td>
                </tr>
                <tr>
                  <td colSpan={6}>Fields</td>
                </tr>
                <tr >
                  <td >name</td>
                  <td >regex</td>
                  <td >replacement</td>
                  <td colSpan={3}>Operations</td>
                </tr>
                  {regexes.map((regex, index) => {
                    return (
                    <tr key={index}>
                      <td>
                        <input type="text" value={regex.name ? regex.name : ''} onChange={handleRegexNameChange} name={'' + index} style={{border:"none"}}
                          onBlur={checkRegex}
                        />
                      </td>
                      <td>
                        <input type="text" value={regex.regex ? regex.regex : ''} onChange={handleRegexChange} name={''+index} style={{border:"none"}}
                          onBlur={checkRegex}
                        />
                      </td>
                      <td>
                        <input type="text" value={regex.replacement? regex.replacement: ''} onChange={handleRegexReplacementChange} name={'' +index} style={{border:"none"}}
                          onBlur={checkRegex}
                        />
                      </td>
                      <td>
                        <Button variant="secondary" onClick={() => moveUp(index)}>
                          MoveUp
                        </Button>
                      </td>
                      <td>
                        <Button variant="secondary" onClick={() => moveDown(index)}>
                          MoveDown
                        </Button>
                      </td>
                      <td>
                        <Button variant="danger" onClick={() => removeRegex(index)}>
                          Delete
                        </Button>
                      </td>
                    </tr>
                  )})}
              </tbody>
            </Table>
                    {data ? <p style={{color:'red'}}>{data.regexStringPattern.error}</p> : null}
            
            <Button variant="secondary" onClick={addNewRegex}>
              Add
            </Button>
          </Modal.Body>
          <Modal.Footer>
            <Button variant="secondary" onClick={handleClose}>
              Close
            </Button>
            <Button variant="primary" onClick={saveRegex}>
              Save Changes
            </Button>
          </Modal.Footer>
        </Modal>
      </>
    );
  }

export default RegexModal;
  
