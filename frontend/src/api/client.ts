import type { OffersResponse, RecommendedOffersResponse } from "../types";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:3000";

export async function fetchOffers(city?: string, domain?: string, limit: number = 10): Promise<OffersResponse> {
    const params = new URLSearchParams();
    params.append("limit", limit.toString());
    if (city) params.append("city", city);
    if (domain) params.append("domain", domain);

    const res = await fetch(`${API_BASE_URL}/offers?${params.toString()}`);
    if (!res.ok) {
        throw new Error("Failed to fetch offers");
    }
    return res.json();
}

export async function fetchRecommendedOffers(studentId: string, limit: number = 5, sortBy?: string): Promise<RecommendedOffersResponse> {
    const params = new URLSearchParams();
    params.append("limit", limit.toString());
    if (sortBy && sortBy !== "none") params.append("sort_by", sortBy);

    const res = await fetch(`${API_BASE_URL}/students/${studentId}/recommended-offers?${params.toString()}`);
    if (!res.ok) {
        throw new Error("Failed to fetch recommended offers");
    }
    return res.json();
}

export async function applyInternship(studentId: string, offerId: string): Promise<{ approved: boolean, message: string }> {
    const res = await fetch(`${API_BASE_URL}/internship`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ studentId, offerId })
    });

    if (!res.ok) {
        throw new Error("Failed to apply for internship");
    }

    const text = await res.text();
    try {
        const json = JSON.parse(text);
        return {
            approved: json.approved !== false,
            message: json.message || "Application successful"
        };
    } catch {
        return { approved: true, message: text || "Applied!" };
    }
}
