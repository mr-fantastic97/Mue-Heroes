import Leaderboard from "./components/Leaderboard";
import EventsPanel from "./components/EventsPanel";
import DevSandbox from "./components/DevSandbox";

export default function App() {
  const openReadme = () => window.open("/README.md", "_blank");

  return (
    <div className="page">
      <header className="hero">
        <h1>🧙 Müe Heroes 🧙</h1>
        <p className="subtitle">The Superblock Leaderboard of Legends</p>
      </header>

      <div className="leaderboard-wrap">
        <Leaderboard />
        <button className="btn btn-success readme-floating" onClick={openReadme}>
          📜 ReadMe.md
        </button>
      </div>

      <div className="two-col">
        <EventsPanel />
        <DevSandbox />
      </div>
    </div>
  );
}






