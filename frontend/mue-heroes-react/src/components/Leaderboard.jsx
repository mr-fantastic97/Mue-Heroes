import { useEffect, useState } from "react";

export default function Leaderboard() {
    const [entries, setEntries] = useState([]);
    const [limit, setLimit] = useState(10); // 5, 10, or ALL

    useEffect(() => {
        fetch("http://localhost:8000/leaderboard")
            .then((r) => r.json())
            .then((data) => setEntries(Array.isArray(data) ? data : []))
            .catch((e) => console.error("leaderboard fetch failed:", e));
    }, []);

    const visible = entries.slice(0, limit === "ALL" ? entries.length : limit);

    return (
        <section className="card leaderboard-card">
            {/* Top bar: chips (left) + Connect Wallet (right) */}
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
                    {visible.length === 0 ? (
                        <tr>
                            <td colSpan={6} className="empty">No entries yet.</td>
                        </tr>
                    ) : (
                        visible.map((e, i) => (
                            <tr key={e.wallet || i}>
                                <td>#{i + 1}</td>
                                <td className="wallet-cell" title={e.wallet || e.wallet_tag}>
                                    {e.wallet_tag || e.wallet || "—"}
                                </td>
                                <td>{e.mu_level ?? "—"}</td>
                                <td>{e.score ?? "—"}</td>
                                <td>{e.block_height ?? "—"}</td>
                                <td>{e.date_mined ? new Date(e.date_mined).toLocaleString() : "—"}</td>
                            </tr>
                        ))
                    )}
                </tbody>
            </table>
        </section>
    );
}
