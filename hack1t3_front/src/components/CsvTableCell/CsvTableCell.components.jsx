import React, { useState } from "react";
import './CsvTableCell.components.scss';
import TableCell from "@mui/material/TableCell";
import { CellModal } from "../export.components";

export function CsvTableCell({ cellData, rowKey, setOpen, setContent, setEdited }) {
  const [cellValue, setCellValue] = useState(cellData);

  if (rowKey !== "id" && rowKey !== "checked") {
    return (
      <>
        <TableCell
          className={
            cellValue.newValue !== null ? "TableCell_changed" : null
          }
          onClick={() => {
            setOpen(true);
            setContent(
              <CellModal
                firstValue={cellValue.value}
                cellValue={cellValue}
                setCellValue={setCellValue}
                setOpen={setOpen}
                setEdited={setEdited}
              />
            );
          }}
        >
          {cellValue.newValue === null ? cellValue.value : cellValue.newValue}
        </TableCell>
      </>
    );
  }
}
