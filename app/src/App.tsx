import { useAppDispatch, useAppSelector } from "./hooks";
import { increment, decrement, selectCount } from "./store/counter";
import { selectApiStatus } from "./store/apiStatus";
import "./App.css";

function App() {
  const dispatch = useAppDispatch();

  const count = useAppSelector(selectCount);
  const status = useAppSelector(selectApiStatus);

  const handleIncrement = () => dispatch(increment());
  const handleDecrement = () => dispatch(decrement());

  return (
    <div className="App">
      <header className="App-header">
        <p>API Status: {status}</p>
        <p>
          <button onClick={handleDecrement}>-</button>
          <span>{count}</span>
          <button onClick={handleIncrement}>+</button>
        </p>
      </header>
    </div>
  );
}

export default App;
