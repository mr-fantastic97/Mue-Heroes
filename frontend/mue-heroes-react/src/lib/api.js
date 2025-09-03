export const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8000";

export async function getJSON(path, { timeout = 8000, signal } = {}) {
    const controller = new AbortController();
    const onAbort = () => controller.abort();
    if (signal) signal.addEventListener("abort", onAbort, { once: true });

    const id = setTimeout(() => controller.abort(), timeout);
    try {
        const res = await fetch(`${API_BASE}${path}`, { signal: controller.signal, credentials: "omit" });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return await res.json();
    } finally {
        clearTimeout(id);
        if (signal) signal.removeEventListener("abort", onAbort);
    }
}

export async function postJSON(path, body, { timeout = 10000, signal } = {}) {
    const controller = new AbortController();
    const onAbort = () => controller.abort();
    if (signal) signal.addEventListener("abort", onAbort, { once: true });

    const id = setTimeout(() => controller.abort(), timeout);
    try {
        const res = await fetch(`${API_BASE}${path}`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body ?? {}),
            signal: controller.signal,
            credentials: "omit",
        });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return await res.json();
    } finally {
        clearTimeout(id);
        if (signal) signal.removeEventListener("abort", onAbort);
    }
}
