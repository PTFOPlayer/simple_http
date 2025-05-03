import "./App.css";
import { BrowserRouter, Route, Routes } from "react-router-dom";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/sth" element={<Sth />} />
        <Route path="/about" element={<About />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;

function Home() {
  return (
    <>
      <h1>Home</h1>
      <a href="/">home</a>
      <br />
      <a href="/about">about</a>
      <br />
      <a href="/sth">sth</a>
      <br />
    </>
  );
}

function Sth() {
  return (
    <>
      <h1>Sth</h1>
      <a href="/">home</a>
      <br />
      <a href="/about">about</a>
      <br />
      <a href="/sth">sth</a>
      <br />
    </>
  );
}

function About() {
  return (
    <>
      <h1>Sth</h1>
      <a href="/">home</a>
      <br />
      <a href="/about">about</a>
      <br />
      <a href="/sth">sth</a>
      <br />
    </>
  );
}
