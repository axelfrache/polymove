import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { fetchRecommendedOffers } from "../api/client";
import { OfferCard } from "../components/OfferCard";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Loader2, LogOut, GraduationCap, MapPin, AlertCircle, Compass } from "lucide-react";

export function StudentDashboard() {
    const navigate = useNavigate();
    const [studentId, setStudentId] = useState<string | null>(null);
    const [sortBy, setSortBy] = useState<string>("none");

    useEffect(() => {
        const id = localStorage.getItem("studentId");
        if (!id) {
            navigate("/login");
        } else {
            setStudentId(id);
        }
    }, [navigate]);

    const { data, isLoading, isError, error } = useQuery({
        queryKey: ["recommended-offers", studentId, sortBy],
        queryFn: () => {
            if (!studentId) throw new Error("No student ID");
            return fetchRecommendedOffers(studentId, 5, sortBy);
        },
        enabled: !!studentId,
    });

    const handleLogout = () => {
        localStorage.removeItem("studentId");
        navigate("/");
    };

    if (!studentId) return null;

    const count = data?.offers?.length ?? 0;

    return (
        <div className="min-h-[calc(100vh-3.5rem)] bg-gradient-to-b from-muted/20 to-background">
            <div className="container mx-auto py-10 px-4 max-w-7xl">
                <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
                    <div>
                        <h1 className="text-3xl font-bold tracking-tight">Student Dashboard</h1>
                        <p className="text-muted-foreground text-sm mt-1.5">
                            Personalized internship recommendations based on your domain.
                        </p>
                    </div>
                    <div className="flex items-center gap-2">
                        <Button variant="outline" size="sm" asChild>
                            <Link to="/" className="flex items-center gap-1.5">
                                <Compass className="w-4 h-4" /> Explore All
                            </Link>
                        </Button>
                        <Button variant="ghost" size="sm" onClick={handleLogout} className="text-muted-foreground hover:text-foreground">
                            <LogOut className="w-4 h-4 mr-1.5" /> Logout
                        </Button>
                    </div>
                </div>

                {data?.student && (
                    <Card className="mb-8 border-0 shadow-md bg-card ring-1 ring-border/50">
                        <CardContent className="flex items-center gap-5 py-5 px-6">
                            <div className="bg-primary p-3 rounded-xl shrink-0">
                                <GraduationCap className="w-6 h-6 text-primary-foreground" />
                            </div>
                            <div className="flex-1 min-w-0">
                                <p className="font-semibold text-lg leading-tight">
                                    {data.student.firstname} {data.student.name}
                                </p>
                                <p className="text-xs text-muted-foreground mt-1 font-mono truncate">{studentId}</p>
                            </div>
                            <div className="flex items-center gap-2 shrink-0">
                                <span className="text-xs text-muted-foreground uppercase tracking-wide font-medium">Domain</span>
                                <Badge className="px-3 py-1">{data.student.domain}</Badge>
                            </div>
                        </CardContent>
                    </Card>
                )}

                <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-6 gap-4">
                    <div className="flex items-baseline gap-3">
                        <h2 className="text-xl font-semibold">Recommended Offers</h2>
                        {!isLoading && !isError && data && (
                            <Badge variant="secondary" className="px-2.5 py-0.5">{count} offer{count !== 1 ? "s" : ""}</Badge>
                        )}
                    </div>
                    <div className="flex items-center gap-2.5">
                        <span className="text-sm text-muted-foreground whitespace-nowrap">Sort by:</span>
                        <Select value={sortBy} onValueChange={setSortBy}>
                            <SelectTrigger className="w-48 h-9 text-sm">
                                <SelectValue placeholder="Sort…" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="none">None (Default)</SelectItem>
                                <SelectItem value="safety">Safety</SelectItem>
                                <SelectItem value="economy">Economy</SelectItem>
                                <SelectItem value="quality_of_life">Quality of Life</SelectItem>
                                <SelectItem value="culture">Culture</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                </div>

                <Separator className="mb-8" />

                {isLoading && (
                    <div className="flex flex-col justify-center items-center py-32 gap-3">
                        <Loader2 className="h-8 w-8 animate-spin text-primary" />
                        <p className="text-sm text-muted-foreground">Loading recommendations…</p>
                    </div>
                )}

                {isError && (
                    <div className="flex items-start gap-3 bg-destructive/10 text-destructive border border-destructive/20 p-5 rounded-xl">
                        <AlertCircle className="w-5 h-5 mt-0.5 shrink-0" />
                        <div>
                            <p className="font-semibold text-sm">Failed to load recommendations</p>
                            <p className="text-sm opacity-80 mt-0.5">{error.message}</p>
                        </div>
                    </div>
                )}

                {!isLoading && !isError && data?.offers && (
                    <>
                        {data.offers.length === 0 ? (
                            <div className="text-center py-32 bg-muted/30 rounded-2xl border border-dashed">
                                <MapPin className="w-12 h-12 text-muted-foreground mx-auto mb-4 opacity-30" />
                                <h3 className="text-base font-medium text-muted-foreground">No recommendations available</h3>
                                <p className="text-sm text-muted-foreground mt-1.5 opacity-70">
                                    No offers match your domain yet.
                                </p>
                            </div>
                        ) : (
                            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                                {data.offers.map((offer) => (
                                    <OfferCard key={offer.id} offer={offer} studentId={studentId} />
                                ))}
                            </div>
                        )}
                    </>
                )}
            </div>
        </div>
    );
}
