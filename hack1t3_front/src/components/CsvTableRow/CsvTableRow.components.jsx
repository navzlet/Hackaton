import React, { useState } from "react";
import TableRow from "@mui/material/TableRow";
import TableCell from "@mui/material/TableCell";
import Checkbox from "@mui/material/Checkbox";
import { CsvTableCell } from "../export.components";
import "./CsvTableRow.components.scss";
import { verifyRowPost } from "../../api/post.api";
import IconButton from '@mui/material/IconButton';
import PictureAsPdfIcon from '@mui/icons-material/PictureAsPdf';

const label = { inputProps: { "aria-label": "Checkbox demo" } };

export function CsvTableRow({ rowInfo, rowData, setOpen, setContent }) {
  const [confirm, setConfirm] = useState(rowInfo.verified);
  const [edited, setEdited] = useState(false);

  if (edited === false) {
    for (let i = 0; i < rowData.length; i++) {
      if (rowData[i].newValue !== null) { setEdited(true) }
    }
  }
  return (
    <TableRow className={rowInfo.verified ? "checked" : null}>
      <TableCell sx={{
        position: "sticky",
        left: 0,
        zIndex: 1,
        backgroundColor: 'white'
      }}
      >
        {
          edited === true ? <span>изменён</span> : null
        }
        <Checkbox
          onChange={() => {
            const rowPostData = {
              uuid: rowInfo.uuid,
              verified: rowInfo.verified ? false : true,
            };
            verifyRowPost(rowPostData, setConfirm, confirm, rowInfo);
          }}
          {...label}
          checked={rowInfo.verified ? true : false}
        />
        <IconButton
          aria-label="copy"
          size="small"
          onClick={() => {

          }}>
          <a href={`${rowInfo.pdfUrl}`} target="_blank">
            <PictureAsPdfIcon />
          </a>
        </IconButton>
      </TableCell>
      {
        rowData.map((cellData, index) => {
          return (
            <CsvTableCell
              key={index}
              cellData={cellData}
              setContent={setContent}
              setOpen={setOpen}
              setEdited={setEdited}
            />
          );
        })
      }
    </TableRow >
  );
}
