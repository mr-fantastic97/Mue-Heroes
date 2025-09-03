export default function HealthPill({ status, label }) {
    let dotClass = "dot err"; // default red

    if (status === "ready") dotClass = "dot ok";
    else if (status === "degraded") dotClass = "dot warn";

    return (
        <div className="status">
            <span className={dotClass}></span>
            <span>{label}</span>
        </div>
    );
}
