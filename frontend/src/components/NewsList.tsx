import type { EnrichedNews } from "../types";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { Badge } from "@/components/ui/badge";

export function NewsList({ news }: { news: EnrichedNews[] }) {
    if (!news || news.length === 0) {
        return (
            <p className="text-sm text-muted-foreground">
                No recent city signals for this offer yet.
            </p>
        );
    }

    return (
        <Accordion type="single" collapsible className="w-full">
            <AccordionItem value="news" className="border-none">
                <AccordionTrigger className="py-0 text-sm font-medium hover:no-underline">
                    {news.length} recent city signal{news.length > 1 ? "s" : ""}
                </AccordionTrigger>
                <AccordionContent className="pt-3">
                    <div className="space-y-3">
                        {news.slice(0, 3).map((item, index) => (
                            <div
                                key={`${item.title}-${index}`}
                                className="rounded-lg border bg-muted/30 px-3 py-3"
                            >
                                <div className="flex items-start justify-between gap-3">
                                    <div className="min-w-0 space-y-1">
                                        <p className="text-sm font-medium leading-snug">{item.title}</p>
                                        <p className="text-xs text-muted-foreground">
                                            {item.source} ·{" "}
                                            {item.date.includes("T")
                                                ? new Date(item.date).toLocaleDateString("en-GB", {
                                                      day: "2-digit",
                                                      month: "short",
                                                  })
                                                : item.date}
                                        </p>
                                    </div>
                                    {item.tags[0] && (
                                        <Badge variant="secondary" className="shrink-0 capitalize">
                                            {item.tags[0]}
                                        </Badge>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
}
