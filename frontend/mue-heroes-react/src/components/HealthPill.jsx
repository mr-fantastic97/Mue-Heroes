export default function HealthPill({ status = "ready", label = "Ready" }) {
    // status: "ready" | "degraded" | "down" | "posting" | "success" | "error"
    const map = { ready: "ok", degraded: "warn", down: "err", posting: "warn", success: "ok", error: "err" };
    return (
        <div className="status" aria-live="polite">
            <span className={`dot ${map[status] || "ok"}`} />
            <span>{label}</span>
        </div>
    );
}
