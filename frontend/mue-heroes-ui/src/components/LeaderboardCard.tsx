// src/components/LeaderboardCard.tsx

type HeroEntry = {
    rank: number;
    tag: string;
    muLevel: number;
    score: number;
    blockHeight: string;
    dateMined: string;
};

const getMuEmoji = (mu: number): string => {
    if (mu >= 21) return "🦍";
    if (mu >= 19) return "🐉";
    if (mu === 18) return "🦁";
    if (mu >= 16) return "🛠️";
    return "🧭";
};

const leaderboard: HeroEntry[] = [
    { rank: 1, tag: "μFiend", muLevel: 18, score: 165, blockHeight: "2,841,122", dateMined: "2025-07-13" },
    { rank: 2, tag: "SnowHash", muLevel: 17, score: 90, blockHeight: "2,838,419", dateMined: "2025-07-12" },
    { rank: 3, tag: "ShadowMiner", muLevel: 17, score: 90, blockHeight: "2,835,210", dateMined: "2025-07-11" },
    { rank: 4, tag: "TxTor", muLevel: 16, score: 45, blockHeight: "2,833,019", dateMined: "2025-07-11" },
    { rank: 5, tag: "BlockScoutX", muLevel: 15, score: 20, blockHeight: "2,832,777", dateMined: "2025-07-10" },
];

const LeaderboardCard = () => {
    return (
        <div className="bg-white border shadow-md p-6 rounded-xl max-w-4xl mx-auto mt-8">
            <h2 className="text-2xl font-bold text-center mb-4">🧙 Müe Heroes 🧙</h2>
            <p className="text-center mb-6">The Superblock Leaderboard of Legends</p>
            <table className="w-full text-left border-collapse">
                <thead>
                    <tr className="border-b border-gray-300">
                        <th className="p-2">Rank</th>
                        <th className="p-2">Hero Tag</th>
                        <th className="p-2">μ-Level</th>
                        <th className="p-2">Score</th>
                        <th className="p-2">Block Height</th>
                        <th className="p-2">Date Mined</th>
                    </tr>
                </thead>
                <tbody>
                    {leaderboard.map((entry) => (
                        <tr key={entry.rank} className="border-b border-gray-200 hover:bg-gray-50">
                            <td className="p-2">{entry.rank <= 3 ? `🥇🥈🥉`[entry.rank - 1] + ` #${entry.rank}` : `#${entry.rank}`}</td>
                            <td className="p-2">{entry.tag}</td>
                            <td className="p-2">{`μ = ${entry.muLevel} ${getMuEmoji(entry.muLevel)}`}</td>
                            <td className="p-2">{entry.score}</td>
                            <td className="p-2">{entry.blockHeight}</td>
                            <td className="p-2">{entry.dateMined}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default LeaderboardCard;
