import { useState, useEffect } from "react";
import "./App.css";

function App() {
  const [status, setStatus] = useState("loading...");

  useEffect(() => {
    fetch("/api/status").then(async (resp) => {
      const { text } = await resp.json();
      setStatus(text);
    });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <p>API Status: {status}</p>
      </header>
    </div>
  );
}

export default App;
