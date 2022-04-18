import { Link } from 'react-router-dom';
import Body from '../components/page/Body';
import Container from '../components/page/Container';
import Header from '../components/page/Header';

export default function About() {
  return (
    <Container>
      <Header>
        <p>About page!</p>
      </Header>
      <Body>
        <p>
          <Link to='/'>Home</Link>
        </p>
      </Body>
    </Container>
  );
}
