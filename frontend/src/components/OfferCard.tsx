import type { EnrichedOffer } from "../types";
import { ScoreBars } from "./ScoreBars";
import { NewsList } from "./NewsList";
import { Card, CardContent, CardFooter, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import {
    AlertDialog,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogAction,
} from "@/components/ui/alert-dialog";
import { applyInternship } from "../api/client";
import { useMutation } from "@tanstack/react-query";
import { MapPin, Calendar, Banknote, ExternalLink, Loader2, CheckCircle2, XCircle } from "lucide-react";
import { useState } from "react";

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
            if (!studentId) throw new Error("Student ID is required to apply");
            return applyInternship(studentId, offer.id);
        },
        onSuccess: (data) => {
            setResult(data);
        },
        onError: (error: Error) => {
            setResult({ approved: false, message: error.message || "Failed to submit application" });
        }
    });

    return (
        <>
            <Card className="flex flex-col h-full shadow-sm hover:shadow-md transition-shadow">
                <CardHeader className="pb-3">
                    <div className="flex justify-between items-start gap-3">
                        <div className="min-w-0">
                            <CardTitle className="text-base leading-snug line-clamp-2">{offer.title}</CardTitle>
                            <CardDescription className="flex items-center gap-1 mt-1.5">
                                <MapPin className="w-3.5 h-3.5 shrink-0" />
                                <span className="truncate capitalize">{offer.city}</span>
                            </CardDescription>
                        </div>
                        <Badge variant="secondary" className="shrink-0 text-xs">{offer.domain}</Badge>
                    </div>
                </CardHeader>

                <Separator />

                <CardContent className="flex-1 pt-4 pb-2">
                    <div className="flex items-center justify-between text-sm mb-4">
                        <span className="flex items-center gap-1.5 font-semibold">
                            <Banknote className="w-4 h-4 text-emerald-500" />
                            {offer.salary.toLocaleString()} €<span className="font-normal text-muted-foreground text-xs">/an</span>
                        </span>
                        <div className="flex items-center gap-1 text-muted-foreground text-xs">
                            <Calendar className="w-3.5 h-3.5" />
                            {new Date(offer.startDate).toLocaleDateString("fr-FR", { month: "short", year: "numeric" })}
                            {offer.endDate && (
                                <> → {new Date(offer.endDate).toLocaleDateString("fr-FR", { month: "short", year: "numeric" })}</>
                            )}
                        </div>
                    </div>

                    <Separator className="mb-4" />

                    <p className="text-xs font-semibold text-muted-foreground uppercase tracking-wide mb-1">City Metrics</p>
                    <ScoreBars scores={offer.scores} />

                    <Separator className="my-4" />

                    <NewsList news={offer.latest_news} />
                </CardContent>

                <CardFooter className="flex justify-between items-center border-t pt-4 gap-2">
                    <Button variant="outline" size="sm" asChild>
                        <a href={offer.link.startsWith('http') ? offer.link : `https://${offer.link}`} target="_blank" rel="noreferrer">
                            <ExternalLink className="w-3.5 h-3.5 mr-1.5" />
                            Details
                        </a>
                    </Button>
                    {studentId && (
                        <Button
                            size="sm"
                            onClick={() => mutation.mutate()}
                            disabled={mutation.isPending}
                        >
                            {mutation.isPending
                                ? <><Loader2 className="w-3.5 h-3.5 mr-1.5 animate-spin" /> Applying…</>
                                : "Apply Now"
                            }
                        </Button>
                    )}
                </CardFooter>
            </Card>

            <AlertDialog open={!!result} onOpenChange={(open) => { if (!open) setResult(null); }}>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle className="flex items-center gap-2">
                            {result?.approved
                                ? <><CheckCircle2 className="w-5 h-5 text-emerald-500" /> Application Approved</>
                                : <><XCircle className="w-5 h-5 text-destructive" /> Application Rejected</>
                            }
                        </AlertDialogTitle>
                        <AlertDialogDescription className="text-sm">
                            <span className="font-medium text-foreground">{offer.title}</span>
                            <br />
                            {result?.message}
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogAction onClick={() => setResult(null)}>OK</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        </>
    );
}
