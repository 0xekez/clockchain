import logo from './logo.svg';
import './App.css';

import NewProcess from "./components/NewProcess"
import Runtime from "./components/Runtime"

import React from "react";
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from "react-router-dom";

export default function App() {
  return (
    <Router>
      <div>
        <Switch>
          <Route path="/newprocess">
            <NewProcess />
          </Route>
          <Route path="/runtime">
            <Runtime />
          </Route>
          <Route path="/">
            <Home />
          </Route>
        </Switch>
      </div>
    </Router>
  );
}

function Home() {
  return <h2>Home</h2>;
}

