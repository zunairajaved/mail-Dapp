import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { CssBaseline, ThemeProvider } from '@mui/material';
import { Provider } from 'react-redux';
// Import Navbar from the Navbar component
import { Navbar } from './components/Navbar';
import {Signin} from './pages/loggedout/Signin';
import { Main } from './pages/loggedin';
import theme from './theme';
import { store } from './store';

function App() {
  return (
    <Provider store={store}>
      <Router>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Routes>
        <Route path='/' element={<Signin/>} />
        <Route path='/mail' element={<Main/>} />
      </Routes>
     
      <Navbar />
    </ThemeProvider>
    </Router>
    </Provider>
  );
}

export default App;