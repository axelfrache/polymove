import type {
    AppliedInternship,
    Notification,
    OffersResponse,
    RecommendedOffersResponse,
} from "../types";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "/api";

async function readErrorMessage(res: Response, fallback: string): Promise<string> {
    const text = await res.text();
    if (!text) {
        return fallback;
    }

    try {
        const json = JSON.parse(text) as { error?: string; message?: string };
        return json.error || json.message || text;
    } catch {
        return text;
    }
}

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
        if (res.status === 404) {
            throw new Error("Student not found");
        }

        throw new Error(await readErrorMessage(res, "Failed to fetch recommended offers"));
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

export async function fetchNotifications(studentId: string): Promise<Notification[]> {
    const res = await fetch(`${API_BASE_URL}/students/${studentId}/notifications`);
    if (!res.ok) {
        if (res.status === 404) {
            throw new Error("Student not found");
        }

        throw new Error(await readErrorMessage(res, "Failed to fetch notifications"));
    }
    return res.json();
}

export async function markNotificationAsRead(notificationId: string): Promise<Notification> {
    const res = await fetch(`${API_BASE_URL}/notifications/${notificationId}/read`, {
        method: "PUT",
    });

    if (!res.ok) {
        throw new Error("Failed to update notification");
    }

    const text = await res.text();
    return text ? JSON.parse(text) : { id: notificationId } as Notification;
}

export async function fetchAppliedInternships(studentId: string): Promise<AppliedInternship[]> {
    const res = await fetch(`${API_BASE_URL}/students/${studentId}/internships`);
    if (!res.ok) {
        if (res.status === 404) {
            throw new Error("Student not found");
        }

        throw new Error(await readErrorMessage(res, "Failed to fetch applications"));
    }
    return res.json();
}
