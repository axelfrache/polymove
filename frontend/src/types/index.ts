export interface EnrichedScores {
    quality_of_life: number;
    economy: number;
    culture: number;
    safety: number;
}

export interface EnrichedNews {
    title: string;
    source: string;
    date: string;
    tags: string[];
}

export interface EnrichedOffer {
    id: string;
    title: string;
    link: string;
    city: string;
    domain: string;
    salary: number;
    startDate: string;
    endDate: string;
    scores: EnrichedScores;
    latest_news: EnrichedNews[];
}

export interface Student {
    id: string;
    firstname: string;
    name: string;
    domain: string;
}

export interface OffersResponse {
    offers: EnrichedOffer[];
}

export interface RecommendedOffersResponse {
    student: Student;
    offers: EnrichedOffer[];
}
