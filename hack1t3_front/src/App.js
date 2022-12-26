import React, { useState } from "react";
import './App.css';
import { ModalWindow } from "./components/export.components";
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from "react-router-dom";
import { NotFoundPage, CsvList, CsvTable, Test3 } from "./pages/export.pages";
// TEST
function App() {
  const [open, setOpen] = useState(false);
  const [content, setContent] = useState(null);
  const [csvInfo, setCsvInfo] = useState(null);

  return (
    <div className="App">
      <ModalWindow open={open} setOpen={setOpen}>{content}</ModalWindow>
      <Router>
        <Routes>
          <Route path="*" element={<Navigate to="/404" replace />} />
          <Route path="/404" element={<NotFoundPage />} />
          <Route path="/" element={<Navigate to="/CsvList" />} />
          <Route path='/CsvList' element={<CsvList setCsvInfo={setCsvInfo} csvInfo={csvInfo} />} />
          <Route path='/CsvTable' element={<CsvTable setContent={setContent} setOpen={setOpen} open={open} csvInfo={csvInfo} />} />
          <Route path='/test3' element={<Test3 />} />
        </Routes>
      </Router>
    </div>
  );
}

export default App;
