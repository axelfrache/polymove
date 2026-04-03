import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Search } from "lucide-react";

const quickDomains = ["AI", "IT", "Data", "Finance"];

interface FiltersBarProps {
    city: string;
    domain: string;
    onCityChange: (value: string) => void;
    onDomainChange: (value: string) => void;
    onSubmit: () => void;
    onQuickDomain: (value: string) => void;
    onClear: () => void;
}

export function FiltersBar({
    city,
    domain,
    onCityChange,
    onDomainChange,
    onSubmit,
    onQuickDomain,
    onClear,
}: FiltersBarProps) {
    const hasFilters = city.trim().length > 0 || domain.trim().length > 0;

    return (
        <Card className="border-border/70 bg-card/95 shadow-sm">
            <CardContent className="space-y-4 px-5 py-5">
                <form
                    className="grid gap-3 lg:grid-cols-[1fr_1fr_auto]"
                    onSubmit={(event) => {
                        event.preventDefault();
                        onSubmit();
                    }}
                >
                    <Input
                        value={city}
                        onChange={(event) => onCityChange(event.target.value)}
                        placeholder="City"
                        className="h-11"
                    />
                    <Input
                        value={domain}
                        onChange={(event) => onDomainChange(event.target.value)}
                        placeholder="Domain"
                        className="h-11"
                    />
                    <Button type="submit" className="h-11 px-5">
                        <Search className="h-4 w-4" />
                        Search
                    </Button>
                </form>

                <div className="flex flex-wrap items-center gap-2">
                    {quickDomains.map((value) => {
                        const isActive = domain.trim().toLowerCase() === value.toLowerCase();

                        return (
                            <Button
                                key={value}
                                type="button"
                                variant={isActive ? "default" : "outline"}
                                size="sm"
                                onClick={() => onQuickDomain(value)}
                            >
                                {value}
                            </Button>
                        );
                    })}

                    {hasFilters ? (
                        <Button type="button" variant="ghost" size="sm" onClick={onClear}>
                            Clear filters
                        </Button>
                    ) : (
                        <Badge variant="secondary" className="rounded-full px-3 py-1">
                            Start with a city or domain
                        </Badge>
                    )}
                </div>
            </CardContent>
        </Card>
    );
}
