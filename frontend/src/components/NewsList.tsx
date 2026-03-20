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
        return <p className="text-sm text-muted-foreground mt-4">No recent news available.</p>;
    }

    return (
        <Accordion type="single" collapsible className="w-full mt-4">
            <AccordionItem value="news">
                <AccordionTrigger className="text-sm font-semibold">
                    Latest City News ({news.length})
                </AccordionTrigger>
                <AccordionContent>
                    <div className="space-y-4">
                        {news.map((item, idx) => (
                            <div key={idx} className="border-l-2 border-primary pl-3 py-1">
                                <h4 className="font-medium text-sm">{item.title}</h4>
                                <div className="flex items-center gap-2 mt-1 text-xs text-muted-foreground">
                                    <span>{item.source}</span>
                                    <span>•</span>
                                    <span>{item.date.includes('T') ? new Date(item.date).toLocaleDateString() : item.date}</span>
                                </div>
                                <div className="flex gap-1 mt-2 flex-wrap">
                                    {item.tags.map((tag) => (
                                        <Badge variant="secondary" key={tag} className="text-[10px]">
                                            {tag}
                                        </Badge>
                                    ))}
                                </div>
                            </div>
                        ))}
                    </div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
}
