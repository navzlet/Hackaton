import React, { useState, useEffect } from 'react';
import { getCsvList } from '../../api/get.api';
import './csvList.pages.scss';
import { Link } from "react-router-dom";
import CircularProgress from '@mui/material/CircularProgress';

export const CsvList = ({ setCsvInfo }) => {
  const [data, setData] = useState(null);

  useEffect(() => {
    getCsvList(setData);
  }, [])

  return (
    <div className='container'>
      <h3 className='title'>Выберите файл для проверки</h3>

      {data !== null ?
        data.data.length !== 0 ?
          <div className="list_container">
            <ul className='csvList'>
              {data.data.map((csvInfo, index) => {
                return (
                  <Link key={index} to={'/CsvTable'} className='csvListLink'>
                    <li className='csvListItem'
                      onClick={() => {
                        setCsvInfo(csvInfo);
                        localStorage.setItem('csvInfo', JSON.stringify(csvInfo));
                      }}>{csvInfo.name}</li>
                  </Link>
                )
              })}
            </ul>
          </div>
          :
          <span className='nullFiles'>Файлов не найдено :(</span>
        :
        <div className='loadingContainer'>
          <CircularProgress />
        </div>
      }
    </div>

  );
}
