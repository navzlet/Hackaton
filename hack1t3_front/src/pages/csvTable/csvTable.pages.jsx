import React, { useEffect, useState } from "react";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";
import "./csvTable.pages.scss";
import { getCsvTable } from "../../api/get.api";
import { CsvTableRow } from "../../components/export.components";

export function CsvTable({ setOpen, setContent, csvInfo }) {
  const [data, setData] = useState(undefined);

  useEffect(() => {
    getCsvTable(
      localStorage.getItem("csvInfo") === null
        ? csvInfo.uuid
        : JSON.parse(localStorage.getItem("csvInfo")).uuid,
      setData
    );
  }, []);
  return (
    <>
      {data !== undefined ? (
        <TableContainer component={Paper} sx={{ maxHeight: document.documentElement.scrollHeight }} className="TableContainer">
          <Table stickyHeader aria-label="sticky table" size="small">
            <TableHead>
              <TableRow>
                <TableCell align="left"
                  sx={{
                    position: "sticky",
                    left: 0,
                    zIndex: 3,
                    backgroundColor: 'white'
                  }}
                >Проверенно</TableCell>
                {data.headers.map((title, index) => (
                  <TableCell key={index} align="left">
                    {title}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody
              sx={{
                overflow: 'hidden',
              }}
            >
              {data.rowsInfo.map((rowInfo, index) => (
                <CsvTableRow
                  key={index}
                  rowInfo={rowInfo}
                  rowData={data.data[index]}
                  setContent={setContent}
                  setOpen={setOpen}
                />
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      ) : null
      }
    </>
  );
}
