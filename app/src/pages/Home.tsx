import { useAppDispatch, useAppSelector } from '../hooks';
import { increment, decrement, selectCount } from '../store/counter';
import { selectApiStatus } from '../store/apiStatus';
import { Link } from 'react-router-dom';

function Index() {
  const dispatch = useAppDispatch();

  const count = useAppSelector(selectCount);
  const status = useAppSelector(selectApiStatus);

  const handleIncrement = () => dispatch(increment());
  const handleDecrement = () => dispatch(decrement());

  return (
    <div className='App'>
      <header className='App-header'>
        <p>API Status: {status}</p>
        <p>
          <button onClick={handleDecrement}>-</button>
          <span>{count}</span>
          <button onClick={handleIncrement}>+</button>
        </p>
        <p>
          <Link to='/about'>About</Link>
        </p>
      </header>
    </div>
  );
}

export default Index;
