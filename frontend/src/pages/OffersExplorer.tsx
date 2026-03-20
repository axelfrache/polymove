import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchOffers } from "../api/client";
import { Filters } from "../components/Filters";
import { OfferCard } from "../components/OfferCard";
import { Badge } from "@/components/ui/badge";
import { Loader2, PackageSearch, AlertCircle } from "lucide-react";

export function OffersExplorer() {
    const [city, setCity] = useState("Paris");
    const [domain, setDomain] = useState("IT");

    const { data, isLoading, isError, error } = useQuery({
        queryKey: ["offers", city, domain],
        queryFn: () => fetchOffers(city, domain, 10),
        enabled: !!(city || domain),
    });

    const handleFilter = (newCity: string, newDomain: string) => {
        setCity(newCity);
        setDomain(newDomain);
    };

    const count = data?.offers?.length ?? 0;

    return (
        <div className="container mx-auto py-8 px-4 max-w-7xl">
            <div className="mb-6">
                <div className="flex items-center gap-3 mb-1">
                    <h1 className="text-3xl font-bold tracking-tight leading-none">Offers Explorer</h1>
                    {!isLoading && !isError && data && (
                        <Badge variant="secondary">{count} offer{count !== 1 ? "s" : ""}</Badge>
                    )}
                </div>
                <p className="text-muted-foreground text-sm">
                    Discover internship offers globally. Filter by city and domain.
                </p>
            </div>

            <Filters onFilter={handleFilter} />

            {isLoading && (
                <div className="flex justify-center items-center py-24">
                    <Loader2 className="h-7 w-7 animate-spin text-primary" />
                </div>
            )}

            {isError && (
                <div className="flex items-start gap-3 bg-destructive/10 text-destructive border border-destructive/20 p-4 rounded-lg">
                    <AlertCircle className="w-5 h-5 mt-0.5 shrink-0" />
                    <div>
                        <p className="font-semibold text-sm">Failed to load offers</p>
                        <p className="text-sm opacity-80 mt-0.5">{error.message}</p>
                    </div>
                </div>
            )}

            {!isLoading && !isError && data?.offers && (
                <>
                    {data.offers.length === 0 ? (
                        <div className="text-center py-24 bg-muted/30 rounded-xl border border-dashed">
                            <PackageSearch className="w-10 h-10 text-muted-foreground mx-auto mb-3 opacity-40" />
                            <h3 className="text-base font-medium text-muted-foreground">No offers found</h3>
                            <p className="text-sm text-muted-foreground mt-1 opacity-70">Try adjusting your filters.</p>
                        </div>
                    ) : (
                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5">
                            {data.offers.map((offer) => (
                                <OfferCard key={offer.id} offer={offer} />
                            ))}
                        </div>
                    )}
                </>
            )}
        </div>
    );
}
