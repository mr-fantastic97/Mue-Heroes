//src/components/EventsPanel.jsx

import { useEffect, useRef, useState } from "react";
import { getJSON } from "../lib/api";

const truncate = (s, n = 12) => (typeof s === "string" && s.length ? `${s.slice(0, n)}…` : "—");
const POLL_MS = 3000;

export default function EventsPanel({ events: external }) {
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
            if (res && Array.isArray(res.events)) setEvents(res.events);
        } catch {
            /* silent */
        }
    };

    // Initial load only if not externally driven
    useEffect(() => {
        if (!external) load();

    }, [external]);

    // Polling only when not externally driven
    useEffect(() => {
        if (external) return;
        if (pollRef.current) clearInterval(pollRef.current);
        inflightRef.current?.abort?.();
        if (auto) pollRef.current = setInterval(load, POLL_MS);
        return () => {
            if (pollRef.current) clearInterval(pollRef.current);
            inflightRef.current?.abort?.();
        };

    }, [auto, external]);

    const onClear = () => setEvents([]);
    const onExport = () => {
        const view = external ?? events;
        const blob = new Blob([JSON.stringify(view, null, 2)], {
            type: "application/json",
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `mue-events-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
    };

    const view = external ?? events;

    return (
        <section className="card tall">
            <div className="panel-head">
                <h2>📡 Events</h2>
                <div className="panel-actions">
                    <span className="label">Auto-refresh:</span>
                    <button
                        className={`chip ${auto ? "active" : ""}`}
                        onClick={() => setAuto(true)}
                        disabled={!!external}
                        title={external ? "External events (no polling)" : ""}
                    >
                        ● On
                    </button>
                    <button
                        className={`chip ${!auto ? "active" : ""}`}
                        onClick={() => setAuto(false)}
                        disabled={!!external}
                        title={external ? "External events (no polling)" : ""}
                    >
                        ○ Off
                    </button>
                    <button
                        className="btn btn-ghost"
                        onClick={onClear}
                        disabled={!!external}
                        title={external ? "Clear is disabled for external events" : ""}
                    >
                        Clear
                    </button>
                    <button className="btn btn-ghost" onClick={onExport}>
                        Export
                    </button>
                </div>
            </div>

            {view.length === 0 ? (
                <p className="muted">Events will stream here …</p>
            ) : (
                <ul className="event-feed">
                    {view.map((ev, i) => {

                        const wallet =
                            ev.wallet ??
                            ev.witness_wallet ??
                            ev.miner_wallet ??
                            ev.wallet_tag ??
                            "";

                        const points = ev.score_delta ?? ev.score ?? 0;
                        const action = ev.event_type ?? (ev.is_witness ? "witness" : "mined"); // robust fallback

                        return (
                            <li key={i} className="event-row">
                                <span className="dot" />
                                <span className="mono">{truncate(wallet)}</span>
                                <span className="muted">{action}</span>
                                <span className="strong">{ev.mu_level ? `μ ${ev.mu_level}` : ""}</span>
                                <span className="amount">{points} pts</span>
                                <span className="muted small">
                                    {ev.date_mined ? new Date(ev.date_mined).toLocaleString() : ""}
                                </span>
                            </li>
                        );
                    })}
                </ul>
            )}
        </section>
    );
}
