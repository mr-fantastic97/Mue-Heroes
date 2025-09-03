import { useMemo, useState } from "react";
import Leaderboard from "./components/Leaderboard";
import EventsPanel from "./components/EventsPanel";
import DevSandbox from "./components/DevSandbox";

export default function App() {
  const openReadme = () => window.open("/README.md", "_blank");

  const [events, setEvents] = useState([]);

  // Same scoring table used in DevSandbox (mirrors backend)
  const pointsFor = (mu) =>
    mu >= 21
      ? 400
      : mu >= 20
        ? 200
        : mu === 19
          ? 120
          : mu === 18
            ? 70
            : mu === 17
              ? 40
              : mu === 16
                ? 25
                : mu === 15
                  ? 15
                  : 0;

  const addEvent = (e) => setEvents((prev) => [e, ...prev]); // newest first
  const resetEvents = () => setEvents([]);

  // Derive leaderboard from events (kdapp replay principle)
  const leaderboard = useMemo(() => {
    const byWallet = new Map();
    for (const ev of [...events].reverse()) {
      if (!ev.wallet) continue;
      const cur =
        byWallet.get(ev.wallet) || {
          wallet: ev.wallet,
          score: 0,
          mu_level: 0,
          block_height: 0,
          date_mined: "",
        };
      const delta =
        ev.event_type === "witness"
          ? Math.floor(pointsFor(ev.mu_level) / 2)
          : pointsFor(ev.mu_level);
      cur.score += delta;
      // Keep latest metadata for display columns
      if (!cur.date_mined || ev.date_mined > cur.date_mined) {
        cur.mu_level = ev.mu_level;
        cur.block_height = ev.block_height || 0;
        cur.date_mined = ev.date_mined;
      }
      byWallet.set(ev.wallet, cur);
    }
    const rows = Array.from(byWallet.values()).sort(
      (a, b) =>
        b.score - a.score ||
        new Date(b.date_mined).getTime() - new Date(a.date_mined).getTime()
    );
    // add wallet_tag to match existing UI column
    for (const r of rows) {
      const s = r.wallet || "";
      r.wallet_tag = s.length <= 14 ? s : `${s.slice(0, 10)}...${s.slice(-4)}`;
    }
    return rows;
  }, [events]);

  return (
    <div className="page">
      <header className="hero">
        <h1>🛡️ Müe Heroes ⛏️</h1>
        <p className="subtitle">The Superblock Leaderboard of Legends</p>
      </header>

      <div className="leaderboard-wrap">
        <Leaderboard entries={leaderboard} />
        <button className="btn btn-success readme-floating" onClick={openReadme}>
          📄 ReadMe.md
        </button>
      </div>

      <div className="two-col">
        <EventsPanel events={events} />
        <DevSandbox onLocalEvent={addEvent} onLocalReset={resetEvents} />
      </div>
    </div>
  );
}







