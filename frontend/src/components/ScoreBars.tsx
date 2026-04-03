import type { EnrichedScores } from "../types";
import { Progress } from "@/components/ui/progress";
import { offerMetrics, hasMetricSignals, normalizeMetricScore } from "@/lib/offer-metrics";

export function ScoreBars({ scores }: { scores: EnrichedScores }) {
    if (!hasMetricSignals(scores)) {
        return (
            <div className="rounded-lg border border-dashed px-3 py-4 text-sm text-muted-foreground">
                City metrics will appear once local signals are available.
            </div>
        );
    }

    return (
        <div className="space-y-3">
            {offerMetrics.map(({ key, label, description }) => {
                const value = normalizeMetricScore(scores[key] ?? 0);

                return (
                    <div key={key} className="space-y-1.5">
                        <div className="flex items-center justify-between gap-3">
                            <div className="min-w-0">
                                <p className="text-sm font-medium">{label}</p>
                                <p className="text-xs text-muted-foreground">{description}</p>
                            </div>
                            <span className="text-sm font-semibold tabular-nums">{value}</span>
                        </div>
                        <Progress value={value} className="h-2" />
                    </div>
                );
            })}
        </div>
    );
}
