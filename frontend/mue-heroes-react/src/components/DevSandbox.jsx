import { useState } from "react";

const BASE_URL = "http://localhost:8000";

export default function DevSandbox() {
    const [status, setStatus] = useState({ label: "Ready", color: "ok" });

    const [mine, setMine] = useState({
        wallet: "",
        mu_level: 15,
        block_height: "",
    });

    const [witness, setWitness] = useState({
        wallet: "",
        mu_level: 15,
        proof: "",
    });

    const postJSON = async (path, body) => {
        try {
            setStatus({ label: "Posting…", color: "warn" });
            const r = await fetch(`${BASE_URL}${path}`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(body),
            });
            if (!r.ok) throw new Error(`HTTP ${r.status}`);
            setStatus({ label: "Success", color: "ok" });
        } catch (e) {
            setStatus({ label: "Error", color: "err" });
            console.error(e);
        }
    };

    const submitMine = () => postJSON("/submit/mine", mine);
    const submitWitness = () => postJSON("/submit/witness", witness);

    const resetAll = async () => {
        try {
            setStatus({ label: "Resetting…", color: "warn" });
            await fetch(`${BASE_URL}/reset`, { method: "POST" });
            setStatus({ label: "Ready", color: "ok" });
        } catch {
            setStatus({ label: "Error", color: "err" });
        }
    };

    return (
        <section className="card tall">
            <div className="panel-head">
                <h2>🛠️ Dev Sandbox</h2>
                <div className="status">
                    <span className={`dot ${status.color}`} />
                    <span>{status.label}</span>
                </div>
            </div>

            {/* Quick Actions */}
            <div className="actions-row">
                <button className="btn btn-primary" onClick={submitMine}>
                    Submit Mine
                </button>
                <button className="btn btn-primary" onClick={submitWitness}>
                    Submit Witness
                </button>
                <button className="btn btn-success" onClick={resetAll}>
                    Reset
                </button>
            </div>

            {/* Forms */}
            <div className="sandbox-forms">
                <fieldset className="box">
                    <legend>Mine Superblock</legend>
                    <div className="grid-2">
                        <label>
                            Wallet
                            <input
                                value={mine.wallet}
                                onChange={(e) =>
                                    setMine((m) => ({ ...m, wallet: e.target.value }))
                                }
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
                                onChange={(e) =>
                                    setMine((m) => ({ ...m, mu_level: Number(e.target.value) }))
                                }
                            />
                        </label>
                        <label>
                            Block Height
                            <input
                                type="number"
                                value={mine.block_height}
                                onChange={(e) =>
                                    setMine((m) => ({ ...m, block_height: Number(e.target.value) }))
                                }
                                placeholder="e.g  212,600,009"

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
                                onChange={(e) =>
                                    setWitness((w) => ({ ...w, wallet: e.target.value }))
                                }
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
                                onChange={(e) =>
                                    setWitness((w) => ({
                                        ...w,
                                        mu_level: Number(e.target.value),
                                    }))
                                }
                            />
                        </label>
                        <label className="span-2">
                            Merkle Proof (JSON / hex)
                            <input
                                value={witness.proof}
                                onChange={(e) =>
                                    setWitness((w) => ({ ...w, proof: e.target.value }))
                                }
                                placeholder='{siblings: ["0x83f1…","0x4c9a…","0xb733…"], path: "010"}'
                            />
                        </label>
                    </div>
                </fieldset>
            </div>
        </section>
    );
}
