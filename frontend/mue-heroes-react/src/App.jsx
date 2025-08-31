import Leaderboard from "./components/Leaderboard";

export default function App() {
  const openReadme = () => {
    // Option A: point to your repo README
    // window.open("https://github.com/your/repo#readme", "_blank");

    // Option B: serve a copy from /public
    window.open("/README.md", "_blank");
  };

  return (
    <div className="page">
      <header className="hero">
        <h1>🧙 Müe Heroes 🧙</h1>
        <p className="subtitle">The Superblock Leaderboard of Legends</p>
      </header>

      {/* Wrapper lets us center the ReadMe button outside the card */}
      <div className="leaderboard-wrap">
        <Leaderboard />
        <button className="btn btn-success readme-floating" onClick={openReadme}>
          📜 ReadMe.md
        </button>
      </div>

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





