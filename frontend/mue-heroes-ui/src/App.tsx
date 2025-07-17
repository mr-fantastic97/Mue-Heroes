import HeroHeader from './components/HeroHeader';
import './index.css'; // no App.css needed

export default function App() {
  return (
    <div className="flex flex-col items-center justify-start min-h-screen pt-12 bg-gradient-to-tr from-indigo-900 via-purple-900 to-black">
      <HeroHeader />
    </div>
  );
}
