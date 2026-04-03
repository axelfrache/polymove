import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { fetchRecommendedOffers } from "../api/client";
import { AppliedOffersPanel } from "../components/AppliedOffersPanel";
import { NotificationPanel } from "../components/NotificationPanel";
import { OfferCard } from "../components/OfferCard";
import { sortOffersForDashboard } from "@/lib/offer-metrics";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { AlertCircle, Loader2, LogOut } from "lucide-react";

function getApiSort(sortBy: string): string {
    if (sortBy === "safety") {
        return "safety";
    }

    return "none";
}

function getStrategyLabel(sortBy: string): string {
    switch (sortBy) {
        case "salary":
            return "Salary-first";
        case "safety":
            return "Safety-first";
        case "recent":
            return "Recent openings";
        case "best_match":
        default:
            return "Best match";
    }
}

export function StudentDashboard() {
    const navigate = useNavigate();
    const [studentId, setStudentId] = useState<string | null>(null);
    const [sortBy, setSortBy] = useState("best_match");

    const clearStoredStudent = () => {
        localStorage.removeItem("studentId");
        window.dispatchEvent(new Event("polymove-student-change"));
        setStudentId(null);
    };

    useEffect(() => {
        const id = localStorage.getItem("studentId");
        if (!id) {
            navigate("/login");
            return;
        }

        setStudentId(id);
    }, [navigate]);

    const { data, isLoading, isError, error } = useQuery({
        queryKey: ["recommended-offers", studentId, getApiSort(sortBy)],
        queryFn: () => {
            if (!studentId) {
                throw new Error("No student ID");
            }

            return fetchRecommendedOffers(studentId, 5, getApiSort(sortBy));
        },
        enabled: !!studentId,
    });

    useEffect(() => {
        if (!(isError && error instanceof Error && error.message === "Student not found")) {
            return;
        }

        clearStoredStudent();
        navigate("/login", {
            replace: true,
            state: { reason: "student-not-found" },
        });
    }, [isError, error, navigate]);

    const handleLogout = () => {
        clearStoredStudent();
        navigate("/");
    };

    if (!studentId) {
        return null;
    }

    const sortedOffers = data?.offers ? sortOffersForDashboard(data.offers, sortBy) : [];
    const offerCount = sortedOffers.length;
    const strategyLabel = getStrategyLabel(sortBy);

    return (
        <div className="min-h-[calc(100vh-3.5rem)] bg-background">
            <div className="container mx-auto max-w-7xl px-4 py-8">
                {data?.student && (
                    <Card className="mb-6 border-border/70 bg-card shadow-sm">
                        <CardContent className="flex flex-col gap-4 px-6 py-5 md:flex-row md:items-center md:justify-between">
                            <div className="space-y-2">
                                <h1 className="text-2xl font-semibold tracking-tight">
                                    {data.student.firstname} {data.student.name}
                                </h1>
                                <div className="flex flex-wrap items-center gap-2">
                                    <Badge variant="secondary" className="rounded-full px-2.5 py-0.5">
                                        {data.student.domain}
                                    </Badge>
                                    <Badge variant="outline" className="rounded-full px-2.5 py-0.5">
                                        {strategyLabel}
                                    </Badge>
                                </div>
                            </div>

                            <Button
                                variant="ghost"
                                size="sm"
                                onClick={handleLogout}
                                className="self-start text-muted-foreground hover:text-foreground md:self-center"
                            >
                                <LogOut className="h-4 w-4" />
                                Logout
                            </Button>
                        </CardContent>
                    </Card>
                )}

                <Tabs defaultValue="recommendations" className="gap-5">
                    <TabsList
                        variant="line"
                        className="w-full justify-start rounded-none border-b bg-transparent p-0"
                    >
                        <TabsTrigger value="recommendations">Recommendations</TabsTrigger>
                        <TabsTrigger value="applications">Applications</TabsTrigger>
                        <TabsTrigger value="notifications">Notifications</TabsTrigger>
                    </TabsList>

                    <TabsContent value="recommendations" className="space-y-5">
                        <Card className="border-border/70 shadow-sm">
                            <CardContent className="flex flex-col gap-4 px-6 py-5 sm:flex-row sm:items-center sm:justify-between">
                                <div className="space-y-1">
                                    <div className="flex flex-wrap items-center gap-2">
                                        <h2 className="text-lg font-semibold tracking-tight">
                                            Recommended offers
                                        </h2>
                                        <Badge variant="secondary" className="rounded-full px-2.5 py-0.5">
                                            {offerCount}
                                        </Badge>
                                    </div>
                                    <p className="text-sm text-muted-foreground">
                                        Ranked for fast comparison and decision-making.
                                    </p>
                                </div>

                                <div className="flex items-center gap-3">
                                    <span className="text-sm text-muted-foreground">Sort by</span>
                                    <Select value={sortBy} onValueChange={setSortBy}>
                                        <SelectTrigger className="w-[180px]">
                                            <SelectValue placeholder="Select a sort" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="best_match">Best match</SelectItem>
                                            <SelectItem value="salary">Salary</SelectItem>
                                            <SelectItem value="safety">Safety</SelectItem>
                                            <SelectItem value="recent">Recent</SelectItem>
                                        </SelectContent>
                                    </Select>
                                </div>
                            </CardContent>
                        </Card>

                        {isLoading && (
                            <div className="flex flex-col items-center justify-center gap-3 py-24">
                                <Loader2 className="h-8 w-8 animate-spin text-primary" />
                                <p className="text-sm text-muted-foreground">
                                    Loading recommendations...
                                </p>
                            </div>
                        )}

                        {isError && (
                            <Card className="border-destructive/20 bg-destructive/5 shadow-none">
                                <CardContent className="flex items-start gap-3 px-5 py-5 text-destructive">
                                    <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
                                    <div className="space-y-1">
                                        <p className="text-sm font-semibold">
                                            Failed to load recommendations
                                        </p>
                                        <p className="text-sm opacity-80">{error.message}</p>
                                    </div>
                                </CardContent>
                            </Card>
                        )}

                        {!isLoading && !isError && (
                            <>
                                {sortedOffers.length === 0 ? (
                                    <Card className="border-dashed shadow-none">
                                        <CardContent className="px-6 py-20 text-center">
                                            <p className="text-base font-medium">
                                                No recommendations available
                                            </p>
                                            <p className="mt-1 text-sm text-muted-foreground">
                                                Matching offers will appear here once they are available.
                                            </p>
                                        </CardContent>
                                    </Card>
                                ) : (
                                    <div className="grid grid-cols-1 gap-5 xl:grid-cols-2">
                                        {sortedOffers.map((offer) => (
                                            <OfferCard
                                                key={offer.id}
                                                offer={offer}
                                                studentId={studentId}
                                            />
                                        ))}
                                    </div>
                                )}
                            </>
                        )}
                    </TabsContent>

                    <TabsContent value="applications" className="space-y-5">
                        <AppliedOffersPanel studentId={studentId} />
                    </TabsContent>

                    <TabsContent value="notifications" className="space-y-5">
                        <NotificationPanel studentId={studentId} />
                    </TabsContent>
                </Tabs>
            </div>
        </div>
    );
}
