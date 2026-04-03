import { useQuery } from "@tanstack/react-query";
import { fetchAppliedInternships } from "../api/client";
import type { AppliedInternship } from "../types";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { CheckCircle2, ExternalLink, Loader2, XCircle } from "lucide-react";

interface AppliedOffersPanelProps {
    studentId: string;
}

function formatPeriod(startDate?: string, endDate?: string) {
    if (!startDate || !endDate) {
        return null;
    }

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

function ApplicationRow({ internship }: { internship: AppliedInternship }) {
    const period = formatPeriod(
        internship.offer?.start_date,
        internship.offer?.end_date,
    );

    return (
        <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
            <div className="min-w-0 space-y-2">
                <div className="flex flex-wrap items-center gap-2">
                    <Badge variant={internship.approved ? "default" : "secondary"}>
                        {internship.approved ? "Approved" : "Rejected"}
                    </Badge>
                    {internship.offer?.domain ? (
                        <Badge variant="outline">{internship.offer.domain}</Badge>
                    ) : null}
                </div>

                <div className="space-y-1">
                    <p className="text-base font-semibold leading-tight">
                        {internship.offer?.title ?? "Offer unavailable"}
                    </p>
                    <p className="text-sm text-muted-foreground">
                        {internship.offer
                            ? `${internship.offer.city} · ${Math.round(internship.offer.salary).toLocaleString("en-GB")} EUR${period ? ` · ${period}` : ""}`
                            : `Offer ID: ${internship.offer_id}`}
                    </p>
                </div>

                <p className="text-sm text-muted-foreground">{internship.message}</p>
            </div>

            <div className="flex items-center gap-2">
                {internship.approved ? (
                    <span className="inline-flex items-center gap-1.5 text-sm text-emerald-600 dark:text-emerald-400">
                        <CheckCircle2 className="h-4 w-4" />
                        Confirmed
                    </span>
                ) : (
                    <span className="inline-flex items-center gap-1.5 text-sm text-muted-foreground">
                        <XCircle className="h-4 w-4" />
                        Closed
                    </span>
                )}

                {internship.offer?.link ? (
                    <Button asChild variant="outline" size="sm">
                        <a href={internship.offer.link} target="_blank" rel="noreferrer">
                            Open offer
                            <ExternalLink className="h-3.5 w-3.5" />
                        </a>
                    </Button>
                ) : null}
            </div>
        </div>
    );
}

export function AppliedOffersPanel({ studentId }: AppliedOffersPanelProps) {
    const { data, isLoading, isError, error } = useQuery({
        queryKey: ["applied-internships", studentId],
        queryFn: () => fetchAppliedInternships(studentId),
        enabled: !!studentId,
    });

    const internships = data ?? [];

    return (
        <Card className="border-0 shadow-md ring-1 ring-border/50">
            <CardHeader className="space-y-2">
                <CardTitle>Applications</CardTitle>
                <CardDescription>
                    Offers already submitted by this student.
                </CardDescription>
            </CardHeader>

            <CardContent className="space-y-4">
                {isLoading ? (
                    <div className="flex items-center gap-3 rounded-lg border border-dashed px-4 py-6 text-sm text-muted-foreground">
                        <Loader2 className="h-4 w-4 animate-spin" />
                        Loading applications...
                    </div>
                ) : null}

                {isError ? (
                    <div className="rounded-lg border border-destructive/20 bg-destructive/5 px-4 py-4 text-sm text-destructive">
                        {(error as Error).message}
                    </div>
                ) : null}

                {!isLoading && !isError && internships.length === 0 ? (
                    <div className="rounded-lg border border-dashed px-4 py-8 text-center text-sm text-muted-foreground">
                        No applications yet. Apply to an offer to see it here.
                    </div>
                ) : null}

                {!isLoading && !isError && internships.length > 0 ? (
                    <div className="space-y-4">
                        {internships.map((internship, index) => (
                            <div key={internship.id}>
                                {index > 0 ? <Separator className="mb-4" /> : null}
                                <ApplicationRow internship={internship} />
                            </div>
                        ))}
                    </div>
                ) : null}
            </CardContent>
        </Card>
    );
}
