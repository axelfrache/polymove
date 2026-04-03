import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchOffers } from "../api/client";
import { FiltersBar } from "../components/FiltersBar";
import { OfferCard } from "../components/OfferCard";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { sortOffers } from "@/lib/offer-metrics";
import { AlertCircle, PackageSearch } from "lucide-react";

const PAGE_SIZE = 6;

export function OffersExplorer() {
    const [draftCity, setDraftCity] = useState("");
    const [draftDomain, setDraftDomain] = useState("");
    const [appliedCity, setAppliedCity] = useState("");
    const [appliedDomain, setAppliedDomain] = useState("");
    const [sortBy, setSortBy] = useState("best_match");
    const [page, setPage] = useState(1);

    const { data, isLoading, isError, error } = useQuery({
        queryKey: ["offers", appliedCity, appliedDomain],
        queryFn: () =>
            fetchOffers(
                appliedCity || undefined,
                appliedDomain || undefined,
                100,
            ),
        enabled: !!(appliedCity || appliedDomain),
    });

    const handleSearch = () => {
        setAppliedCity(draftCity.trim());
        setAppliedDomain(draftDomain.trim());
        setPage(1);
    };

    const handleQuickDomain = (value: string) => {
        setDraftDomain(value);
        setAppliedCity(draftCity.trim());
        setAppliedDomain(value);
        setPage(1);
    };

    const handleClear = () => {
        setDraftCity("");
        setDraftDomain("");
        setAppliedCity("");
        setAppliedDomain("");
        setPage(1);
    };

    const offers = data?.offers ? sortOffers(data.offers, sortBy) : [];
    const count = offers.length;
    const totalPages = Math.max(1, Math.ceil(count / PAGE_SIZE));
    const safePage = Math.min(page, totalPages);
    const paginatedOffers = offers.slice(
        (safePage - 1) * PAGE_SIZE,
        safePage * PAGE_SIZE,
    );
    const visiblePages = Array.from(
        { length: totalPages },
        (_, index) => index + 1,
    ).filter((value) => Math.abs(value - safePage) <= 1 || value === 1 || value === totalPages);

    return (
        <div className="container mx-auto max-w-7xl px-4 py-8">
            <div className="space-y-6">
                <div className="space-y-2">
                    <h1 className="text-3xl font-semibold tracking-tight">Explore opportunities</h1>
                    <p className="max-w-2xl text-sm text-muted-foreground">
                        Filter by city or domain to surface the strongest internships quickly.
                    </p>
                </div>

                <FiltersBar
                    city={draftCity}
                    domain={draftDomain}
                    onCityChange={setDraftCity}
                    onDomainChange={setDraftDomain}
                    onSubmit={handleSearch}
                    onQuickDomain={handleQuickDomain}
                    onClear={handleClear}
                />

                {(appliedCity || appliedDomain) && !isError && (
                    <div className="flex flex-col gap-4 rounded-xl border bg-card/70 px-5 py-4 md:flex-row md:items-center md:justify-between">
                        <div className="space-y-1">
                            <div className="flex items-center gap-2">
                                <h2 className="text-lg font-semibold">Matching offers</h2>
                                {!isLoading && <Badge variant="secondary">{count}</Badge>}
                            </div>
                            <p className="text-sm text-muted-foreground">
                                {appliedCity || appliedDomain
                                    ? `Showing results${appliedCity ? ` in ${appliedCity}` : ""}${appliedDomain ? ` for ${appliedDomain}` : ""}.`
                                    : "Add a city or domain to browse offers."}
                            </p>
                        </div>

                        <div className="flex items-center gap-3">
                            <span className="text-sm text-muted-foreground">Sort by</span>
                            <Select value={sortBy} onValueChange={setSortBy}>
                                <SelectTrigger className="w-[180px]">
                                    <SelectValue placeholder="Best match" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="best_match">Best match</SelectItem>
                                    <SelectItem value="salary">Salary</SelectItem>
                                    <SelectItem value="safety">Safety</SelectItem>
                                    <SelectItem value="recent">Recent</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                )}

                {!appliedCity && !appliedDomain ? (
                    <Card className="border-dashed bg-muted/20">
                        <CardContent className="flex flex-col items-center px-6 py-16 text-center">
                            <PackageSearch className="mb-4 h-10 w-10 text-muted-foreground/60" />
                            <h3 className="text-lg font-semibold">Start with a filter</h3>
                            <p className="mt-2 max-w-md text-sm text-muted-foreground">
                                The current gateway requires at least one filter. Enter a city or a domain to load offers.
                            </p>
                        </CardContent>
                    </Card>
                ) : null}

                {isLoading ? (
                    <div className="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3">
                        {Array.from({ length: 6 }).map((_, index) => (
                            <Card key={index} className="border-border/70 bg-card/90">
                                <CardHeader className="space-y-4">
                                    <div className="h-5 w-16 animate-pulse rounded-full bg-muted" />
                                    <div className="h-7 w-2/3 animate-pulse rounded bg-muted" />
                                    <div className="h-4 w-1/2 animate-pulse rounded bg-muted" />
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <div className="grid gap-3 sm:grid-cols-2">
                                        {Array.from({ length: 4 }).map((_, metricIndex) => (
                                            <div
                                                key={metricIndex}
                                                className="h-16 animate-pulse rounded-lg border bg-muted/50"
                                            />
                                        ))}
                                    </div>
                                    <div className="h-16 animate-pulse rounded-lg border bg-muted/50" />
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                ) : null}

                {isError ? (
                    <div className="flex items-start gap-3 rounded-lg border border-destructive/20 bg-destructive/10 p-4 text-destructive">
                        <AlertCircle className="mt-0.5 h-5 w-5 shrink-0" />
                        <div>
                            <p className="text-sm font-semibold">Failed to load offers</p>
                            <p className="mt-0.5 text-sm opacity-80">
                                {error instanceof Error ? error.message : "Unexpected error"}
                            </p>
                        </div>
                    </div>
                ) : null}

                {!isLoading && !isError && (appliedCity || appliedDomain) ? (
                    count === 0 ? (
                        <Card className="border-dashed bg-muted/20">
                            <CardContent className="flex flex-col items-center px-6 py-16 text-center">
                                <PackageSearch className="mb-4 h-10 w-10 text-muted-foreground/60" />
                                <h3 className="text-lg font-semibold">No offers found</h3>
                                <p className="mt-2 max-w-md text-sm text-muted-foreground">
                                    Try another city, broaden the domain, or switch to a quick filter.
                                </p>
                            </CardContent>
                        </Card>
                    ) : (
                        <div className="space-y-6">
                            <div className="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3">
                                {paginatedOffers.map((offer) => (
                                    <OfferCard key={offer.id} offer={offer} />
                                ))}
                            </div>

                            {totalPages > 1 ? (
                                <div className="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center sm:justify-between">
                                    <p className="text-sm text-muted-foreground">
                                        Showing {(safePage - 1) * PAGE_SIZE + 1}-{Math.min(safePage * PAGE_SIZE, count)} of {count}
                                    </p>

                                    <div className="flex items-center gap-2">
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onClick={() => setPage((current) => Math.max(1, current - 1))}
                                            disabled={safePage === 1}
                                        >
                                            Previous
                                        </Button>

                                        {visiblePages.map((value, index) => {
                                            const previous = visiblePages[index - 1];
                                            const needsGap = previous && value - previous > 1;

                                            return (
                                                <div key={value} className="flex items-center gap-2">
                                                    {needsGap ? (
                                                        <span className="px-1 text-sm text-muted-foreground">...</span>
                                                    ) : null}
                                                    <Button
                                                        variant={value === safePage ? "default" : "outline"}
                                                        size="sm"
                                                        onClick={() => setPage(value)}
                                                    >
                                                        {value}
                                                    </Button>
                                                </div>
                                            );
                                        })}

                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onClick={() => setPage((current) => Math.min(totalPages, current + 1))}
                                            disabled={safePage === totalPages}
                                        >
                                            Next
                                        </Button>
                                    </div>
                                </div>
                            ) : null}
                        </div>
                    )
                ) : null}
            </div>
        </div>
    );
}
