import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import type { EnrichedOffer } from "../types";
import { applyInternship } from "../api/client";
import { MetricsBlock } from "./MetricsBlock";
import {
    formatOfferPeriod,
    getLeadingMetric,
    getOfferMatchScore,
} from "@/lib/offer-metrics";
import { AlertDialog, AlertDialogAction, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/components/ui/alert-dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { CheckCircle2, Loader2, MapPin, XCircle } from "lucide-react";

interface OfferCardProps {
    offer: EnrichedOffer;
    studentId?: string | null;
}

interface ApplicationResult {
    approved: boolean;
    message: string;
}

export function OfferCard({ offer, studentId }: OfferCardProps) {
    const [result, setResult] = useState<ApplicationResult | null>(null);

    const mutation = useMutation({
        mutationFn: () => {
            if (!studentId) {
                throw new Error("Student ID is required to apply");
            }

            return applyInternship(studentId, offer.id);
        },
        onSuccess: (data) => {
            setResult(data);
        },
        onError: (error: Error) => {
            setResult({
                approved: false,
                message: error.message || "Failed to submit application",
            });
        },
    });

    const matchScore = getOfferMatchScore(offer.scores);
    const leadingMetric = getLeadingMetric(offer.scores);
    const latestSignal = offer.latest_news[0];

    return (
        <>
            <Card className="flex h-full flex-col border-border/70 bg-card/95 shadow-sm transition-all duration-200 hover:-translate-y-0.5 hover:shadow-lg">
                <CardHeader className="space-y-4 pb-4">
                    <div className="flex items-start justify-between gap-4">
                        <div className="min-w-0 space-y-3">
                            <div className="flex flex-wrap items-center gap-2">
                                <Badge variant="secondary" className="rounded-full px-2.5 py-0.5">
                                    {offer.domain}
                                </Badge>
                            </div>
                            <div className="space-y-2">
                                <CardTitle className="text-xl leading-tight tracking-tight text-balance">
                                    {offer.title}
                                </CardTitle>
                                <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-sm text-muted-foreground">
                                    <span className="inline-flex items-center gap-1.5">
                                        <MapPin className="h-4 w-4" />
                                        {offer.city}
                                    </span>
                                    <span className="font-medium text-foreground">
                                        {Math.round(offer.salary).toLocaleString("en-GB")} EUR
                                    </span>
                                    <span>{formatOfferPeriod(offer.startDate, offer.endDate)}</span>
                                </div>
                            </div>
                        </div>

                        <div className="shrink-0 rounded-2xl border border-amber-500/30 bg-amber-500/10 px-4 py-3 text-right text-amber-600 dark:text-amber-400">
                            <p className="text-[11px] font-medium uppercase tracking-[0.18em]">
                                Match score
                            </p>
                            <p className="mt-1 text-4xl font-semibold tracking-tight">
                                {matchScore}
                                <span className="text-sm opacity-70">/100</span>
                            </p>
                        </div>
                    </div>

                    <div className="rounded-xl border bg-muted/25 px-4 py-3">
                        <p className="text-sm font-medium">
                            {leadingMetric
                                ? `${leadingMetric.label} is the strongest signal`
                                : "No city signal available yet"}
                        </p>
                        <p className="mt-1 text-sm text-muted-foreground">
                            {leadingMetric
                                ? `${leadingMetric.description} currently leads this destination.`
                                : "Local news will populate this card as events arrive."}
                        </p>
                    </div>
                </CardHeader>

                <Separator />

                <CardContent className="flex-1 space-y-5 pt-5">
                    <div className="space-y-3">
                        <div className="flex items-center justify-between">
                            <h4 className="text-sm font-semibold">City metrics</h4>
                            {leadingMetric ? (
                                <span className="text-xs text-muted-foreground">
                                    Strongest: {leadingMetric.label}
                                </span>
                            ) : null}
                        </div>
                        <MetricsBlock scores={offer.scores} />
                    </div>

                    <Separator />

                    <div className="space-y-3">
                        <div className="flex items-center justify-between gap-3">
                            <h4 className="text-sm font-semibold">Recent signal</h4>
                            <span className="text-xs text-muted-foreground">
                                {offer.latest_news.length > 0
                                    ? `${offer.latest_news.length} update${offer.latest_news.length > 1 ? "s" : ""}`
                                    : "No recent update"}
                            </span>
                        </div>

                        {latestSignal ? (
                            <div className="rounded-lg border bg-muted/20 px-3 py-3">
                                <div className="flex items-start justify-between gap-3">
                                    <div className="min-w-0 space-y-1">
                                        <p className="text-sm font-medium leading-snug">{latestSignal.title}</p>
                                        <p className="text-xs text-muted-foreground">
                                            {latestSignal.source} ·{" "}
                                            {new Date(latestSignal.date).toLocaleDateString("en-GB", {
                                                day: "2-digit",
                                                month: "short",
                                            })}
                                        </p>
                                    </div>
                                    {latestSignal.tags[0] ? (
                                        <Badge variant="outline" className="shrink-0 capitalize">
                                            {latestSignal.tags[0]}
                                        </Badge>
                                    ) : null}
                                </div>
                            </div>
                        ) : (
                            <p className="text-sm text-muted-foreground">
                                No recent city signals for this offer.
                            </p>
                        )}
                    </div>
                </CardContent>

                <CardFooter className="justify-end gap-3 border-t pt-4">
                    <Button asChild variant="outline">
                        <a href={offer.link} target="_blank" rel="noreferrer">
                            View details
                        </a>
                    </Button>
                    {studentId ? (
                        <Button onClick={() => mutation.mutate()} disabled={mutation.isPending}>
                            {mutation.isPending ? (
                                <>
                                    <Loader2 className="h-4 w-4 animate-spin" />
                                    Applying...
                                </>
                            ) : (
                                "Apply"
                            )}
                        </Button>
                    ) : (
                        <Button variant="outline" disabled>
                            Login to apply
                        </Button>
                    )}
                </CardFooter>
            </Card>

            <AlertDialog
                open={!!result}
                onOpenChange={(open) => {
                    if (!open) {
                        setResult(null);
                    }
                }}
            >
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle className="flex items-center gap-2">
                            {result?.approved ? (
                                <>
                                    <CheckCircle2 className="h-5 w-5 text-emerald-500" />
                                    Application approved
                                </>
                            ) : (
                                <>
                                    <XCircle className="h-5 w-5 text-destructive" />
                                    Application rejected
                                </>
                            )}
                        </AlertDialogTitle>
                        <AlertDialogDescription className="text-sm">
                            <span className="font-medium text-foreground">{offer.title}</span>
                            <br />
                            {result?.message}
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogAction onClick={() => setResult(null)}>Close</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        </>
    );
}
