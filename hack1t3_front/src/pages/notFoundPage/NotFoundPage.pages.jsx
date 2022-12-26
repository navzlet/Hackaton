import React from 'react';
import { NavLink } from "react-router-dom";

export const NotFoundPage = () => {
  return (
    <div>
      404
      <NavLink to='/'>На главную</NavLink>
    </div>
  );
};
