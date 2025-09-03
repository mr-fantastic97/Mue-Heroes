import { useEffect, useRef, useState } from "react";
import { getJSON } from "../lib/api";

const POLL_MS = 3000;

export default function EventsPanel() {
    const [events, setEvents] = useState([]);
    const [auto, setAuto] = useState(true);

    const pollRef = useRef(null);
    const inflightRef = useRef(null);

    const load = async () => {
        try {
            inflightRef.current?.abort?.();                 // abort previous
            inflightRef.current = new AbortController();
            const data = await getJSON("/events", { timeout: 8000, signal: inflightRef.current.signal });
            if (Array.isArray(data)) setEvents(data);
        } catch { /* silent by design */ }
    };

    useEffect(() => { load(); }, []);

    useEffect(() => {
        if (pollRef.current) clearInterval(pollRef.current);
        inflightRef.current?.abort?.();
        if (auto) pollRef.current = setInterval(load, POLL_MS);
        return () => {
            if (pollRef.current) clearInterval(pollRef.current);
            inflightRef.current?.abort?.();
        };
    }, [auto]);

    const onClear = () => setEvents([]);
    const onExport = () => {
        const blob = new Blob([JSON.stringify(events, null, 2)], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url; a.download = `mue-events-${Date.now()}.json`; a.click();
        URL.revokeObjectURL(url);
    };

    return (
        <section className="card tall">
            <div className="panel-head">
                <h2>📡 Events</h2>
                <div className="panel-actions">
                    <span className="label">Auto-refresh:</span>
                    <button className={`chip ${auto ? "active" : ""}`} onClick={() => setAuto(true)} aria-pressed={auto}>● On</button>
                    <button className={`chip ${!auto ? "active" : ""}`} onClick={() => setAuto(false)} aria-pressed={!auto}>○ Off</button>
                    <button className="btn btn-ghost" onClick={onClear}>Clear</button>
                    <button className="btn btn-ghost" onClick={onExport}>Export</button>
                </div>
            </div>

            {events.length === 0 ? (
                <p className="muted">Events will stream here (fetched from backend)…</p>
            ) : (
                <ul className="event-list">
                    {events.map((ev, i) => (
                        <li key={ev.id ?? i} className="event-row">
                            <span className="event-time">{ev.timestamp ? new Date(ev.timestamp).toLocaleString() : "—"}</span>
                            <span className="pill">{ev.kind ?? "event"}</span>
                            <span className="mono">{ev.wallet ? `${ev.wallet.slice(0, 12)}…` : "—"}</span>
                            <span>μ {ev.mu_level ?? "—"}</span>
                            <span>H {ev.block_height ?? "—"}</span>
                        </li>
                    ))}
                </ul>
            )}
        </section>
    );
}
