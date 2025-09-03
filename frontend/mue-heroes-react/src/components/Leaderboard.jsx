// src/components/Leaderboard.jsx
import { useEffect, useState } from "react";

export default function Leaderboard({ entries: injected }) {
    const [entries, setEntries] = useState([]);
    const [limit, setLimit] = useState(10); // 5, 10, or ALL
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (injected) {
            setLoading(false);
            return;
        }
        const API = import.meta.env.VITE_API_URL;
        fetch(`${API}/leaderboard`)
            .then((r) => {
                if (!r.ok) throw new Error(`HTTP ${r.status}`);
                return r.json();
            })
            .then((data) => setEntries(Array.isArray(data) ? data : []))
            .catch((e) => console.error("leaderboard fetch failed:", e))
            .finally(() => setLoading(false));
    }, [injected]);

    const source = injected ?? entries;
    const visible = source.slice(0, limit === "ALL" ? source.length : limit);

    return (
        <section className="card leaderboard-card">
            <div className="topbar">
                <div className="chip-row">
                    <button
                        className={`chip ${limit === 5 ? "active" : ""}`}
                        onClick={() => setLimit(5)}
                    >
                        Top 5
                    </button>
                    <button
                        className={`chip ${limit === 10 ? "active" : ""}`}
                        onClick={() => setLimit(10)}
                    >
                        Top 10
                    </button>
                    <button
                        className={`chip ${limit === "ALL" ? "active" : ""}`}
                        onClick={() => setLimit("ALL")}
                    >
                        All
                    </button>
                </div>

                <button className="btn btn-primary connect-btn">🔑 Connect Wallet</button>
            </div>

            <table>
                <thead>
                    <tr>
                        <th>Rank</th>
                        <th>Hero Tag</th>
                        <th>μ-Level</th>
                        <th>Score</th>
                        <th>Block Height</th>
                        <th>Date Mined</th>
                    </tr>
                </thead>
                <tbody>
                    {loading ? (
                        <tr>
                            <td colSpan={6} className="empty">
                                Loading leaderboard…
                            </td>
                        </tr>
                    ) : visible.length === 0 ? (
                        <tr>
                            <td colSpan={6} className="empty">
                                No entries yet
                            </td>
                        </tr>
                    ) : (
                        visible.map((e, i) => (
                            <tr key={e.wallet || e.wallet_tag || i}>
                                <td>#{i + 1}</td>
                                <td className="wallet-cell" title={e.wallet || e.wallet_tag}>
                                    {e.wallet_tag || e.wallet || "—"}
                                </td>
                                <td>{e.mu_level ?? "—"}</td>
                                <td>{e.score ?? "—"}</td>
                                <td>{e.block_height ?? "—"}</td>
                                <td>
                                    {e.date_mined ? new Date(e.date_mined).toLocaleString() : "—"}
                                </td>
                            </tr>
                        ))
                    )}
                </tbody>
            </table>
        </section>
    );
}

