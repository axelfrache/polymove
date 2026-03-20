import type { EnrichedScores } from "../types";
import { Progress } from "@/components/ui/progress";
import { Heart, TrendingUp, Palette, Shield } from "lucide-react";

const MAX_SCORE = 2000;

const metrics = [
    {
        key: "quality_of_life" as const,
        label: "Quality of Life",
        icon: Heart,
        color: "text-emerald-500",
    },
    {
        key: "economy" as const,
        label: "Economy",
        icon: TrendingUp,
        color: "text-blue-500",
    },
    {
        key: "culture" as const,
        label: "Culture",
        icon: Palette,
        color: "text-violet-500",
    },
    {
        key: "safety" as const,
        label: "Safety",
        icon: Shield,
        color: "text-amber-500",
    },
];

export function ScoreBars({ scores }: { scores: EnrichedScores }) {
    return (
        <div className="space-y-2.5 mt-4">
            {metrics.map(({ key, label, icon: Icon, color }) => {
                const raw = scores[key] ?? 0;
                const pct = Math.min(100, Math.round((raw / MAX_SCORE) * 100));
                return (
                    <div key={key}>
                        <div className="flex justify-between items-center text-xs mb-1">
                            <span className={`flex items-center gap-1 font-medium ${color}`}>
                                <Icon className="w-3 h-3" /> {label}
                            </span>
                            <span className="text-muted-foreground tabular-nums">{raw.toLocaleString()}</span>
                        </div>
                        <Progress value={pct} className="h-1.5" />
                    </div>
                );
            })}
        </div>
    );
}
