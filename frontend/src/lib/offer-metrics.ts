import type { EnrichedOffer, EnrichedScores } from "../types";

export const offerMetrics = [
    {
        key: "quality_of_life" as const,
        label: "Quality of Life",
        description: "Daily comfort",
    },
    {
        key: "economy" as const,
        label: "Economy",
        description: "Local market",
    },
    {
        key: "culture" as const,
        label: "Culture",
        description: "City energy",
    },
    {
        key: "safety" as const,
        label: "Safety",
        description: "Urban stability",
    },
];

type MetricKey = (typeof offerMetrics)[number]["key"];

export function normalizeMetricScore(rawScore: number): number {
    if (!rawScore) {
        return 0;
    }

    if (rawScore <= 100) {
        return Math.max(0, Math.min(100, Math.round(rawScore)));
    }

    return Math.max(0, Math.min(100, Math.round((rawScore - 850) / 2)));
}

export function hasMetricSignals(scores: EnrichedScores): boolean {
    return offerMetrics.some(({ key }) => (scores[key] ?? 0) > 0);
}

export function getOfferMatchScore(scores: EnrichedScores): number {
    if (!hasMetricSignals(scores)) {
        return 0;
    }

    const total = offerMetrics.reduce(
        (sum, { key }) => sum + normalizeMetricScore(scores[key] ?? 0),
        0,
    );

    return Math.round(total / offerMetrics.length);
}

export function getLeadingMetric(scores: EnrichedScores): {
    key: MetricKey;
    label: string;
    description: string;
    value: number;
} | null {
    if (!hasMetricSignals(scores)) {
        return null;
    }

    const ranked = offerMetrics
        .map((metric) => ({
            ...metric,
            value: normalizeMetricScore(scores[metric.key] ?? 0),
        }))
        .sort((left, right) => right.value - left.value);

    return ranked[0] ?? null;
}

export function sortOffersForDashboard(offers: EnrichedOffer[], sortBy: string): EnrichedOffer[] {
    return sortOffers(offers, sortBy);
}

export function sortOffers(offers: EnrichedOffer[], sortBy: string): EnrichedOffer[] {
    const next = [...offers];

    switch (sortBy) {
        case "salary":
            return next.sort((left, right) => right.salary - left.salary);
        case "safety":
            return next.sort(
                (left, right) =>
                    normalizeMetricScore(right.scores.safety) -
                    normalizeMetricScore(left.scores.safety),
            );
        case "recent":
            return next.sort(
                (left, right) =>
                    new Date(right.startDate).getTime() - new Date(left.startDate).getTime(),
            );
        case "best_match":
        default:
            return next.sort(
                (left, right) =>
                    getOfferMatchScore(right.scores) - getOfferMatchScore(left.scores),
            );
    }
}

export function getMatchTone(score: number): {
    label: string;
    className: string;
} {
    if (score >= 78) {
        return {
            label: "Great match",
            className: "border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400",
        };
    }

    if (score >= 58) {
        return {
            label: "Solid match",
            className: "border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400",
        };
    }

    return {
        label: "Low signal",
        className: "border-muted-foreground/20 bg-muted text-muted-foreground",
    };
}

export function formatOfferPeriod(startDate: string, endDate: string): string {
    const start = new Date(startDate);
    const end = new Date(endDate);

    return `${start.toLocaleDateString("en-GB", {
        month: "short",
        year: "numeric",
    })} - ${end.toLocaleDateString("en-GB", {
        month: "short",
        year: "numeric",
    })}`;
}
