// 🧠 Müe Heroes – script.js

// Maps μ-levels to tier emojis
function getMuEmoji(mu, eventType) {
    if (eventType === "witness") return "👁️"; // Oracle always
    if (mu >= 21) return "🧙‍♂️";      // Honorius
    if (mu >= 19) return "🐉";         // Mythic
    if (mu === 18) return "🦁";        // Legend
    if (mu === 16 || mu === 17) return "⛏️"; // Forged
    if (mu === 15) return "🪙";        // Scout
    return "—";
}

// Maps μ-levels to tier names
function getMuTier(mu, eventType) {
    if (eventType === "witness") return "μOracle";
    if (mu >= 21) return "μHonorius";
    if (mu >= 19) return "μMythic";
    if (mu === 18) return "μLegend";
    if (mu === 16 || mu === 17) return "μForged";
    if (mu === 15) return "μScout";
    return "";
}

// Formats ISO date to readable string
function formatDate(isoDate) {
    const date = new Date(isoDate);
    return date.toLocaleDateString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
}

//Truncate Wallet
function truncateWallet(wallet) {
    if (!wallet) return "";
    if (wallet.length <= 22) return wallet;
    return wallet.slice(0, 14) + "..." + wallet.slice(-6); // Can tweak as needed for added kns functionality. 
}


document.addEventListener("DOMContentLoaded", () => {
    const tbody = document.getElementById("leaderboard-body");
    const refreshButton = document.querySelector(".refresh-btn");
    const historyButton = document.querySelector(".history-btn");
    const joinButton = document.querySelector(".join-btn");

    const refreshLeaderboard = () => {
        fetch("http://localhost:8000/leaderboard")
            .then(res => res.json())
            .then(data => {
                tbody.innerHTML = "";

                const totalRows = 10;
                for (let i = 0; i < totalRows; i++) {
                    const entry = data[i];
                    const row = document.createElement("tr");

                    if (entry) {
                        // μ-display logic
                        let muText = "—";
                        if (entry.event_type === "witness") {
                            muText = getMuEmoji(null, "witness");  // Just 👁️
                        } else if (entry.mu_level != null) {
                            muText = `${getMuEmoji(entry.mu_level, "mined")} ${entry.mu_level}`;
                        }

                        const dateText = entry.date_mined
                            ? formatDate(entry.date_mined)
                            : "—";

                        const rankEmojis = ["🥇", "🥈", "🥉"];
                        const tierName = getMuTier(entry.mu_level, entry.event_type);
                        const rankDisplay =
                            i < 3
                                ? `${rankEmojis[i]} ${tierName}`
                                : `🥉 ${tierName}`; // Bronze for everyone 3+

                        row.innerHTML = `
                            <td>${rankDisplay}</td>
                            <td class="wallet-cell" title="${entry.wallet}">${truncateWallet(entry.wallet)}</td>
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

    const showHistoricalHeroes = () => {
        alert("Feature coming soon: View historical hero blocks and score history.");
    };

    const showHowToJoin = () => {
        alert("To join Müe Heroes, mine a superblock (μ ≥ 15) using your Kaspa wallet!");
    };

    refreshButton.addEventListener("click", refreshLeaderboard);
    historyButton.addEventListener("click", showHistoricalHeroes);
    joinButton.addEventListener("click", showHowToJoin);

    refreshLeaderboard();
    setInterval(refreshLeaderboard, 15000);
});