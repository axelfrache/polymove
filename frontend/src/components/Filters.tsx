import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import { Search, MapPin, Tag, X } from "lucide-react";

interface FiltersProps {
    onFilter: (city: string, domain: string) => void;
}

export function Filters({ onFilter }: FiltersProps) {
    const [city, setCity] = useState("");
    const [domain, setDomain] = useState("");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        onFilter(city, domain);
    };

    const handleClear = () => {
        setCity("");
        setDomain("");
        onFilter("", "");
    };

    const hasFilters = city || domain;

    return (
        <form onSubmit={handleSubmit} className="flex flex-col sm:flex-row gap-4 items-end mb-8 bg-card p-4 rounded-lg border shadow-sm">
            <div className="grid w-full items-center gap-1.5">
                <Label htmlFor="city" className="flex items-center gap-1">
                    <MapPin className="w-3.5 h-3.5 text-muted-foreground" /> City
                </Label>
                <Input
                    type="text"
                    id="city"
                    placeholder="e.g. Paris"
                    value={city}
                    onChange={(e) => setCity(e.target.value)}
                />
            </div>
            <div className="grid w-full items-center gap-1.5">
                <Label htmlFor="domain" className="flex items-center gap-1">
                    <Tag className="w-3.5 h-3.5 text-muted-foreground" /> Domain
                </Label>
                <Input
                    type="text"
                    id="domain"
                    placeholder="e.g. IT"
                    value={domain}
                    onChange={(e) => setDomain(e.target.value)}
                />
            </div>
            <div className="flex gap-2 w-full sm:w-auto">
                {hasFilters && (
                    <Button type="button" variant="outline" onClick={handleClear} className="flex-1 sm:flex-none">
                        <X className="w-4 h-4 mr-1" /> Clear
                    </Button>
                )}
                <Button type="submit" className="flex-1 sm:flex-none">
                    <Search className="w-4 h-4 mr-1" /> Search
                </Button>
            </div>
        </form>
    );
}
