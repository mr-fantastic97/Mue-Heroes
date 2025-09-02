import { useEffect, useRef, useState } from "react";

const BASE_URL = "http://localhost:8000";

export default function EventsPanel() {
    const [events, setEvents] = useState([]);
    const [auto, setAuto] = useState(true);
    const timerRef = useRef(null);

    // Poll events while auto-refresh is on
    useEffect(() => {
        const fetchOnce = async () => {
            try {
                const r = await fetch(`${BASE_URL}/events`);
                if (!r.ok) return;
                const data = await r.json();
                if (Array.isArray(data)) setEvents(data);
            } catch (_) {
                /* silent fail for now */
            }
        };

        fetchOnce(); // initial
        if (auto) {
            timerRef.current = setInterval(fetchOnce, 3000);
        }
        return () => clearInterval(timerRef.current);
    }, [auto]);

    const clearEvents = () => setEvents([]);

    const exportEvents = () => {
        const blob = new Blob([JSON.stringify(events, null, 2)], {
            type: "application/json",
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `mue-events-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
    };

    return (
        <section className="card tall">
            <div className="panel-head">
                <h2>📡 Events</h2>
                <div className="panel-actions">
                    <span className="label">Auto-refresh:</span>
                    <button
                        className={`chip ${auto ? "active" : ""}`}
                        onClick={() => setAuto(true)}
                    >
                        ● On
                    </button>
                    <button
                        className={`chip ${!auto ? "active" : ""}`}
                        onClick={() => setAuto(false)}
                    >
                        ○ Off
                    </button>
                    <button className="btn btn-ghost" onClick={clearEvents}>
                        Clear
                    </button>
                    <button className="btn btn-ghost" onClick={exportEvents}>
                        Export
                    </button>
                </div>
            </div>

            {events.length === 0 ? (
                <p className="muted">Events will stream here (fetched from backend)…</p>
            ) : (
                <ul className="event-list">
                    {events.map((ev, i) => (
                        <li key={ev.id ?? i} className="event-row">
                            <span className="event-time">
                                {ev.timestamp
                                    ? new Date(ev.timestamp).toLocaleString()
                                    : "—"}
                            </span>
                            <span className="pill">{ev.kind ?? "event"}</span>
                            <span className="mono">
                                {ev.wallet?.slice(0, 10) ?? "wallet"}…
                            </span>
                            <span>μ {ev.mu_level ?? "—"}</span>
                            <span>H {ev.block_height ?? "—"}</span>
                        </li>
                    ))}
                </ul>
            )}
        </section>
    );
}
