import axios from "axios";
import { serverUrl } from "./serverUrl";

export const newCellValuePost = (
  newCellData,
  setOpen,
  cellValue,
  inputValue
) => {
  axios
    .post(`${serverUrl}/api/update_cell_new_value`, newCellData)
    .then(() => {
      setOpen(false);
      alert('Успешно!')
    })
    .catch((error) => alert(error));
};

export const verifyRowPost = (rowPostData, setConfirm, confirm, rowInfo) => {
  axios
    .post(`${serverUrl}/api/update_verified_row`, rowPostData)
    .then(() => {
      setConfirm(confirm ? false : true);
      rowInfo.verified = rowInfo.verified ? false : true;
    })
    .catch((error) => alert(error));
};
