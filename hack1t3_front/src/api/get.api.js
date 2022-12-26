import axios from 'axios';
import { serverUrl } from './serverUrl';

export const getCsvList = (setData) => {
  axios.get(`${serverUrl}/api/get_csv_files_list`)
    .then((response) => setData(response))
    .catch((error) => console.log(error))
}

export const getCsvTable = (uuid, setData) => {
  axios.get(`${serverUrl}/api/get_table/${uuid}`)
    .then((response) => setData(response.data))
    .catch((error) => console.log(error))
}
