import { Link } from 'react-router-dom';

export default function About() {
  return (
    <div>
      <p>About page!</p>
      <div>
        <Link to='/'>Home</Link>
      </div>
    </div>
  );
}
