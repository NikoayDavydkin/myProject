import React from "react";
import { Button, Modal } from "react-bootstrap";

interface Props  {
    handleClose: () => void;
    handleDelete : (id: string) => void ;
    show:boolean,
    id: string
    
}

const  DeleteModal:React.FC<Props> = ({handleClose, handleDelete, show, id}) =>  {
    
  
    return (
      <>
  
        <Modal show={show} onHide={handleClose}>
          <Modal.Body>Are you sure you want to delete ?</Modal.Body>
          <Modal.Footer>
            <Button variant="secondary" onClick={handleClose}>
              No
            </Button>
            <Button variant="primary" onClick={() => handleDelete(id)}>
              Yes
            </Button>
          </Modal.Footer>
        </Modal>
      </>
      
    );
  }

export default DeleteModal;
  