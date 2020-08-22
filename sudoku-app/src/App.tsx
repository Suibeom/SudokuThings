import React from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

function SudokuBoardThing(){
  let div_style = {columnCount: 3};
  return (
    <div style={div_style}>
      <div><SudokuBlock /> <SudokuBlock /> <SudokuBlock /></div>
      <div><SudokuBlock /> <SudokuBlock /> <SudokuBlock /></div>
      <div><SudokuBlock /> <SudokuBlock /> <SudokuBlock /></div>
    </div> 
  );
  }
export class SudokuBoard extends React.Component {
  blocks: SudokuBlock[];
  constructor(props: Readonly<{}>){
    super(props);
    this.state = {};
    this.blocks = Array(9).map(()=> new SudokuBlock({}));
    }
  render(){
    let div_style = {columnCount: 3};
    return (
      <div style={div_style}>
        <div><SudokuBlock /> <SudokuBlock /> <SudokuBlock /></div>
        <div><SudokuBlock /> <SudokuBlock /> <SudokuBlock /></div>
        <div><SudokuBlock /> <SudokuBlock /> <SudokuBlock /></div>
      </div> 
    );
  }
}

export class SudokuBlock extends React.Component{
  squares: SudokuSquare[]
  constructor(props: Readonly<{}>){
    super(props)
    this.squares = Array(9).map(()=> new SudokuSquare({}));
  }
  render(){return  (<span>
    <div><input type="text" pattern='[1-9]' size={1}></input><input type="text" pattern='[1-9]' size={1}></input><input type="text" pattern='[1-9]' size={1}></input></div>
    <div><input type="text" pattern='[1-9]' size={1}></input><input type="text" pattern='[1-9]' size={1}></input><input type="text" pattern='[1-9]' size={1}></input></div>
    <div><input type="text" pattern='[1-9]' size={1}></input><input type="text" pattern='[1-9]' size={1}></input><input type="text" pattern='[1-9]' size={1}></input></div>
    </span>
  );}
}

interface SquareState{
  selected: boolean,
  contents: number | null,
  pencilmarks: number[],
}

export class SudokuSquare extends React.Component{
  state: SquareState;
  constructor(props: Readonly<{}>){
    super(props);
    this.state = {selected:false, contents: null, pencilmarks: []};
  }
  render(){
    return <div></div>
  }
}

export default App;
