// 🧠 Müe Heroes – script.js

// Maps μ-levels to tier emojis
function getMuEmoji(mu) {
    if (mu >= 21) return "🧙‍♂️";      // Honorius
    if (mu >= 19) return "🐉";         // Mythic
    if (mu === 18) return "🦁";        // Legend
    if (mu === 16 || mu === 17) return "⛏️"; // Forged
    if (mu === 15) return "🪙";        // Scout
    if (mu < 15 && mu !== null) return "👁️"; // Oracle (witness-only)
    return "—";                        // Default / missing
}

// Maps μ-levels to tier names
function getMuTier(mu) {
    if (mu >= 21) return "μHonorius";
    if (mu >= 19) return "μMythic";
    if (mu === 18) return "μLegend";
    if (mu === 16 || mu === 17) return "μForged";
    if (mu === 15) return "μOracle";
    return "";
}

// Formats ISO date to "Jul 21, 2025"
function formatDate(isoDate) {
    const date = new Date(isoDate);
    return date.toLocaleDateString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
}

// DOM loaded logic
document.addEventListener("DOMContentLoaded", () => {
    const tbody = document.getElementById("leaderboard-body");
    const refreshButton = document.querySelector(".refresh-btn");
    const historyButton = document.querySelector(".history-btn");
    const joinButton = document.querySelector(".join-btn");

    // Fetch leaderboard from backend and populate table
    const refreshLeaderboard = () => {
        fetch("http://localhost:8000/leaderboard")
            .then(res => res.json())
            .then(data => {
                tbody.innerHTML = ""; // Clear existing rows

                const totalRows = 10;
                for (let i = 0; i < totalRows; i++) {
                    const entry = data[i];
                    const row = document.createElement("tr");

                    if (entry) {
                        // μ-display
                        const muText = entry.mu_level != null
                            ? `${getMuEmoji(entry.mu_level)} ${entry.mu_level}`
                            : "—";

                        const dateText = entry.date_mined
                            ? formatDate(entry.date_mined)
                            : "—";

                        // Rank display
                        const rankEmojis = ["🥇", "🥈", "🥉"];
                        const tierName = getMuTier(entry.mu_level);
                        const rankDisplay =
                            i < 3
                                ? `${rankEmojis[i]} ${tierName}`
                                : `#${i + 1}`;

                        row.innerHTML = `
                            <td>${rankDisplay}</td>
                            <td>${entry.wallet}</td>
                            <td>${muText}</td>
                            <td>${entry.score}</td>
                            <td>${entry.block_height ?? "—"}</td>
                            <td>${dateText}</td>
                        `;
                    } else {
                        row.innerHTML = `
                            <td>#${i + 1}</td>
                            <td><em>Awaiting Hero...</em></td>
                            <td>—</td>
                            <td>—</td>
                            <td>—</td>
                            <td>—</td>
                        `;
                    }

                    tbody.appendChild(row);
                }
            })
            .catch(err => {
                console.error("Error loading leaderboard:", err);
            });
    };

    // Buttons
    const showHistoricalHeroes = () => {
        alert("Feature coming soon: View historical hero blocks and score history.");
    };

    const showHowToJoin = () => {
        alert("To join Müe Heroes, mine a superblock (μ ≥ 15) using your Kaspa wallet!");
    };

    // Event listeners
    refreshButton.addEventListener("click", refreshLeaderboard);
    historyButton.addEventListener("click", showHistoricalHeroes);
    joinButton.addEventListener("click", showHowToJoin);

    // Initial load + interval
    refreshLeaderboard();
    setInterval(refreshLeaderboard, 15000); // Refresh every 15s 
});
