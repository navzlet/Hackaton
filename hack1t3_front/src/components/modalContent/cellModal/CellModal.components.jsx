import React, { useState } from "react";
import { newCellValuePost } from "../../../api/post.api";
import "./CellModal.scss";
import IconButton from '@mui/material/IconButton';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import Button from '@mui/material/Button';
import { CopyToClipboard } from 'react-copy-to-clipboard';

export const CellModal = ({ setOpen, cellValue, firstValue, setEdited }) => {
  const [inputValue, setInputValue] = useState(null);

  const newCellData = {
    csvFileUuid: cellValue.csvFileUuid,
    columnName: cellValue.columnName,
    pdfRowUuid: cellValue.pdfRowUuid,
    newValue: null
  }

  return (
    <div className="CellModal">
      <div className="CellModal_titleContainer">
        <h3 className="CellModal_title">Оригинал</h3>
        <CopyToClipboard text={cellValue.value}
          onCopy={() => this.setState({ copied: true })}>
          <IconButton
            aria-label="copy"
            className="CellModal_titleBtn"
            size="small"
          >
            <ContentCopyIcon />
          </IconButton>
        </CopyToClipboard>
      </div>
      <div className="CellModal_originalText">{cellValue.value}</div>
      <hr className="CellModal_line"></hr>
      <div>
        <div className="CellModal_titleContainer">
          <h3 className="CellModal_title">исправление</h3>
          <CopyToClipboard text={cellValue.newValue}
            onCopy={() => {
              if (cellValue.newValue !== null) {
                this.setState({ copied: true })
              } else {
                alert('Исправлений нету!')
              }
            }}>
            <IconButton
              aria-label="copy"
              className="CellModal_titleBtn"
              size="small"
            >
              <ContentCopyIcon />
            </IconButton>
          </CopyToClipboard>
        </div>
        <textarea
          className="newValue_Textarea"
          placeholder={
            cellValue.newValue === null ?
              "Изменений не было"
              : cellValue.newValue
          }
          onChange={(event) => setInputValue(event.target.value)}
        />
      </div>
      <div className="buttons">
        <div className="mainButtons">
          <Button variant="contained" onClick={() => {
            cellValue.newValue = inputValue;
            newCellData.newValue = inputValue;
            setEdited(true);
            newCellValuePost(newCellData, setOpen, cellValue, inputValue)
          }}>сохранить исправление</Button>
          <Button variant="contained" onClick={() => {
            if (cellValue.newValue !== null) {
              cellValue.newValue = null;
              setEdited(false);
              newCellValuePost(newCellData, setOpen, cellValue, inputValue);
            } else {
              alert('Исправления отсутсвуют!')
            }
          }}>отменить исправление</Button>
        </div>
        <Button variant="outlined" onClick={() => { setOpen(false); }}>закрыть</Button>
      </div>
    </div >
  );
};
