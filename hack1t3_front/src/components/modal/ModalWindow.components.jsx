import React from "react";
import Box from "@mui/material/Box";
import Modal from "@mui/material/Modal";
import "./ModalWindow.scss";

export const ModalWindow = ({ open, setOpen, children }) => {
  const handleClose = () => setOpen(false);

  return (
    <>
      <Modal
        open={open}
        onClose={handleClose}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description"
      >
        <Box className="modalBox">{children}</Box>
      </Modal>
    </>
  );
};
