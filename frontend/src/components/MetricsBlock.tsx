import type { EnrichedScores } from "../types";
import { Progress } from "@/components/ui/progress";
import { hasMetricSignals, normalizeMetricScore, offerMetrics } from "@/lib/offer-metrics";

export function MetricsBlock({ scores }: { scores: EnrichedScores }) {
    if (!hasMetricSignals(scores)) {
        return (
            <div className="rounded-lg border border-dashed px-3 py-3 text-sm text-muted-foreground">
                Metrics will appear when city signals become available.
            </div>
        );
    }

    return (
        <div className="grid gap-3 sm:grid-cols-2">
            {offerMetrics.map(({ key, label }) => {
                const value = normalizeMetricScore(scores[key] ?? 0);

                return (
                    <div key={key} className="space-y-2 rounded-lg border bg-muted/20 px-3 py-3">
                        <div className="flex items-center justify-between gap-3">
                            <span className="text-sm font-medium">{label}</span>
                            <span className="text-sm font-semibold tabular-nums">{value}</span>
                        </div>
                        <Progress value={value} className="h-1.5" />
                    </div>
                );
            })}
        </div>
    );
}
