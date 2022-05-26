import React from 'react';
import { useSelector } from 'react-redux';
import { Route, Navigate } from 'react-router';

export function PrivateRoute({component: Component, ...rest}) {
  const wallet = useSelector(state => state.account.wallet);
  return (
    <Route {...rest} render={props => {
      if (!wallet || !wallet.connected) {
        // not logged in so redirect ot signin page
        return <Navigate to={{ pathname: '/'}} />
      }
      // logged in so return component
      return <Component {...props} />
    }} />
  );
}
