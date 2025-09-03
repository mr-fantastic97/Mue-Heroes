//src/components/EventsPanel.jsx

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
            inflightRef.current?.abort?.(); // cancel previous
            inflightRef.current = new AbortController();
            const res = await getJSON("/events?limit=50&order=desc", {
                timeout: 8000,
                signal: inflightRef.current.signal,
            });
            // backend wraps events inside { events: [...] }
            if (res && Array.isArray(res.events)) setEvents(res.events);
        } catch {
            /* silent */
        }
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
                    <button className={`chip ${auto ? "active" : ""}`} onClick={() => setAuto(true)}>● On</button>
                    <button className={`chip ${!auto ? "active" : ""}`} onClick={() => setAuto(false)}>○ Off</button>
                    <button className="btn btn-ghost" onClick={onClear}>Clear</button>
                    <button className="btn btn-ghost" onClick={onExport}>Export</button>
                </div>
            </div>

            {events.length === 0 ? (
                <p className="muted">Events will stream here …</p>
            ) : (
                <ul className="event-feed">
                    {events.map((ev, i) => (
                        <li key={i} className="event-row">
                            <span className="dot" /> {/* status icon */}
                            <span className="mono">{ev.wallet ? ev.wallet.slice(0, 12) + "…" : "—"}</span>
                            <span className="muted">{ev.event_type ?? "event"}</span>
                            <span className="strong">{ev.mu_level ? `μ ${ev.mu_level}` : ""}</span>
                            <span className="amount">{ev.score_delta} pts</span>
                            <span className="muted small">
                                {ev.date_mined ? new Date(ev.date_mined).toLocaleString() : ""}
                            </span>
                        </li>
                    ))}
                </ul>
            )}
        </section>
    );
}

