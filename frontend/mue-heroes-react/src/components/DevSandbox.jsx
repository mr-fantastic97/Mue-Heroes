import { useEffect, useRef, useState } from "react";
import HealthPill from "./HealthPill";
import { getJSON, postJSON } from "../lib/api";

const HEALTH_MS = 15000;

const statusFromDecision = (d) => {
    if (!d) return { ui: "down", label: "API unreachable" };
    if (d.status === "down" || d.ok === false) return { ui: "down", label: d.message || "Down" };
    if (d.status === "degraded") return { ui: "degraded", label: d.message || "Degraded" };
    return { ui: "ready", label: "Ready" };
};

export default function DevSandbox() {
    const [health, setHealth] = useState({ ui: "ready", label: "Ready" });
    const [posting, setPosting] = useState(false);
    const [message, setMessage] = useState(null); // ✅ for inline feedback

    const [mine, setMine] = useState({ wallet: "", mu_level: 15, block_height: "" });
    const [witness, setWitness] = useState({ wallet: "", mu_level: 15, proof: "" });

    const pollRef = useRef(null);
    const inflightRef = useRef(null);

    const refreshHealth = async () => {
        try {
            inflightRef.current?.abort?.();
            inflightRef.current = new AbortController();
            const decision = await getJSON("/health", { timeout: 8000, signal: inflightRef.current.signal });
            setHealth(statusFromDecision(decision));
        } catch {
            setHealth({ ui: "down", label: "API unreachable" });
        }
    };

    useEffect(() => {
        refreshHealth();
        pollRef.current = setInterval(refreshHealth, HEALTH_MS);
        return () => {
            if (pollRef.current) clearInterval(pollRef.current);
            inflightRef.current?.abort?.();
        };
    }, []);

    const doAction = async (fn, label) => {
        setPosting(true);
        setMessage(null);
        try {
            const data = await fn();
            setMessage({ type: "success", text: `${label} succeeded! ✅` });
            console.log("Response:", data);
        } catch (e) {
            setMessage({ type: "error", text: `${label} failed: ${e?.message || "Unknown error"}` });
        } finally {
            setPosting(false);
            setTimeout(refreshHealth, 300);
        }
    };

    return (
        <section className="card tall">
            <div className="panel-head">
                <h2>🛠️ Dev Sandbox</h2>
                <HealthPill
                    status={health.ui === "ready" ? "ready" : health.ui === "degraded" ? "degraded" : "down"}
                    label={health.label}
                />
            </div>

            <div className="actions-row">
                <button
                    className="btn btn-primary"
                    disabled={posting}
                    onClick={() =>
                        doAction(
                            () =>
                                postJSON("/submit/mine", mine, {
                                    headers: {
                                        "content-type": "application/json",
                                        "x-mue-key": import.meta.env.VITE_DEV_SUBMIT_KEY,
                                    },
                                }),
                            "Mine submission"
                        )
                    }
                >
                    Submit Mine
                </button>

                <button
                    className="btn btn-primary"
                    disabled={posting}
                    onClick={() =>
                        doAction(
                            () =>
                                postJSON("/submit/witness", witness, {
                                    headers: {
                                        "content-type": "application/json",
                                        "x-mue-key": import.meta.env.VITE_DEV_SUBMIT_KEY,
                                    },
                                }),
                            "Witness submission"
                        )
                    }
                >
                    Submit Witness
                </button>

                <button
                    className="btn btn-success"
                    disabled={posting}
                    onClick={() =>
                        doAction(
                            () =>
                                postJSON("/reset", {}, {
                                    headers: {
                                        "x-admin-token": import.meta.env.VITE_DEV_ADMIN_TOKEN,
                                    },
                                }),
                            "Reset"
                        )
                    }
                >
                    Reset
                </button>
            </div>

            {message && (
                <p
                    style={{
                        color: message.type === "error" ? "red" : "green",
                        fontWeight: "600",
                        marginTop: "8px",
                    }}
                >
                    {message.text}
                </p>
            )}

            <div className="sandbox-forms">
                <fieldset className="box">
                    <legend>Mine Superblock</legend>
                    <div className="grid-2">
                        <label>
                            Wallet
                            <input
                                value={mine.wallet}
                                onChange={(e) => setMine((m) => ({ ...m, wallet: e.target.value }))}
                                placeholder="kaspa:..."
                            />
                        </label>
                        <label>
                            μ-Level
                            <input
                                type="number"
                                min={15}
                                max={32}
                                value={mine.mu_level}
                                onChange={(e) => setMine((m) => ({ ...m, mu_level: Number(e.target.value) }))}
                            />
                        </label>
                        <label>
                            Block Height
                            <input
                                type="number"
                                value={mine.block_height}
                                onChange={(e) => setMine((m) => ({ ...m, block_height: Number(e.target.value) || "" }))}
                                placeholder="e.g. 2,520,000"
                            />
                        </label>
                    </div>
                </fieldset>

                <fieldset className="box">
                    <legend>Witness Superblock</legend>
                    <div className="grid-2">
                        <label>
                            Wallet
                            <input
                                value={witness.wallet}
                                onChange={(e) => setWitness((w) => ({ ...w, wallet: e.target.value }))}
                                placeholder="kaspa:..."
                            />
                        </label>
                        <label>
                            μ-Level
                            <input
                                type="number"
                                min={15}
                                max={32}
                                value={witness.mu_level}
                                onChange={(e) => setWitness((w) => ({ ...w, mu_level: Number(e.target.value) }))}
                            />
                        </label>
                        <label className="span-2">
                            Merkle Proof (JSON / hex)
                            <input
                                value={witness.proof}
                                onChange={(e) => setWitness((w) => ({ ...w, proof: e.target.value }))}
                                placeholder='{ siblings: ["0x...", "0x..."], path: "010" }'
                            />
                        </label>
                    </div>
                </fieldset>
            </div>
        </section>
    );
}
