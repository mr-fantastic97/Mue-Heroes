// src/App.jsx
import Leaderboard from "./components/Leaderboard";

export default function App() {
  return (
    <div className="page">
      <header className="hero">
        <h1>🧙 Müe Heroes 🧙</h1>
        <p className="subtitle">The Superblock Leaderboard of Legends</p>
      </header>

      {/* Full-width leaderboard */}
      <Leaderboard />

      {/* Two tall cards below */}
      <div className="two-col">
        <section className="card tall">
          <h2>📡 Events</h2>
          <p>Events will stream here (fetched from backend)…</p>
        </section>

        <section className="card tall">
          <h2>🛠️ Dev Sandbox</h2>
          <p>Demo submission form / test buttons will go here…</p>
        </section>
      </div>
    </div>
  );
}





