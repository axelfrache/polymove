import { useEffect, useMemo, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { fetchNotifications, markNotificationAsRead } from "../api/client";
import type { Notification } from "../types";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Bell, BellRing, Loader2, MailOpen, MessageSquareWarning, X } from "lucide-react";

interface NotificationPanelProps {
    studentId: string;
}

export function NotificationPanel({ studentId }: NotificationPanelProps) {
    const queryClient = useQueryClient();
    const hasInitialized = useRef(false);
    const knownNotificationIds = useRef<Set<string>>(new Set());
    const [incomingAlerts, setIncomingAlerts] = useState<Notification[]>([]);

    const { data, isLoading, isError, error } = useQuery({
        queryKey: ["notifications", studentId],
        queryFn: () => fetchNotifications(studentId),
        enabled: !!studentId,
        refetchInterval: 5000,
    });

    const mutation = useMutation({
        mutationFn: (notificationId: string) => markNotificationAsRead(notificationId),
        onSuccess: () => {
            void queryClient.invalidateQueries({ queryKey: ["notifications", studentId] });
        },
    });

    const notifications = useMemo(() => data ?? [], [data]);
    const unreadCount = notifications.filter((notification) => !notification.read).length;

    useEffect(() => {
        if (notifications.length === 0) {
            return;
        }

        const currentIds = new Set(notifications.map((notification) => notification.id));

        if (!hasInitialized.current) {
            knownNotificationIds.current = currentIds;
            hasInitialized.current = true;
            return;
        }

        const newUnreadNotifications = notifications.filter(
            (notification) =>
                !notification.read && !knownNotificationIds.current.has(notification.id),
        );

        if (newUnreadNotifications.length > 0) {
            setIncomingAlerts((current) => {
                const existingIds = new Set(current.map((notification) => notification.id));
                const deduped = newUnreadNotifications.filter(
                    (notification) => !existingIds.has(notification.id),
                );
                return [...deduped, ...current];
            });
        }

        knownNotificationIds.current = currentIds;
    }, [notifications]);

    useEffect(() => {
        setIncomingAlerts((current) =>
            current.filter((alert) => notifications.some((notification) => notification.id === alert.id && !notification.read)),
        );
    }, [notifications]);

    const dismissAlert = (notificationId: string) => {
        setIncomingAlerts((current) =>
            current.filter((notification) => notification.id !== notificationId),
        );
    };

    return (
        <Card className="border-0 shadow-md ring-1 ring-border/50">
            <CardHeader className="space-y-3">
                <div className="flex items-center justify-between gap-3">
                    <div className="space-y-1">
                        <CardTitle className="flex items-center gap-2">
                            <Bell className="h-5 w-5 text-primary" />
                            Notifications
                        </CardTitle>
                        <CardDescription>
                            Alerts created by Polytech when new offers match the student domain.
                        </CardDescription>
                    </div>
                    <Badge variant={unreadCount > 0 ? "default" : "secondary"}>
                        {unreadCount} unread
                    </Badge>
                </div>
            </CardHeader>

            <CardContent className="space-y-4">
                {incomingAlerts.length > 0 && (
                    <div className="space-y-3">
                        {incomingAlerts.map((notification) => (
                            <Alert
                                key={notification.id}
                                className="border-primary/25 bg-primary/5 pr-12"
                            >
                                <BellRing className="text-primary" />
                                <AlertTitle>New matching offer alert</AlertTitle>
                                <AlertDescription className="text-foreground/80">
                                    {notification.message}
                                </AlertDescription>
                                <Button
                                    type="button"
                                    size="icon-xs"
                                    variant="ghost"
                                    onClick={() => dismissAlert(notification.id)}
                                    aria-label="Dismiss alert"
                                    className="absolute right-3 top-3"
                                >
                                    <X className="h-3.5 w-3.5" />
                                </Button>
                            </Alert>
                        ))}
                    </div>
                )}

                {isLoading && (
                    <div className="flex items-center gap-3 rounded-lg border border-dashed px-4 py-6 text-sm text-muted-foreground">
                        <Loader2 className="h-4 w-4 animate-spin" />
                        Loading notifications...
                    </div>
                )}

                {isError && (
                    <div className="rounded-lg border border-destructive/20 bg-destructive/5 px-4 py-4 text-sm text-destructive">
                        {(error as Error).message}
                    </div>
                )}

                {!isLoading && !isError && notifications.length === 0 && (
                    <div className="rounded-lg border border-dashed px-4 py-8 text-center text-sm text-muted-foreground">
                        No notifications yet. Create an offer matching this student domain to test the flow.
                    </div>
                )}

                {!isLoading && !isError && notifications.length > 0 && (
                    <div className="space-y-3">
                        {notifications.map((notification, index) => (
                            <div key={notification.id}>
                                {index > 0 && <Separator className="mb-3" />}
                                <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                                    <div className="min-w-0 space-y-1">
                                        <div className="flex items-center gap-2">
                                            <Badge variant={notification.read ? "secondary" : "default"}>
                                                {notification.read ? "Read" : "Unread"}
                                            </Badge>
                                            <span className="text-xs font-medium uppercase tracking-wide text-muted-foreground">
                                                {notification.notification_type.replace("_", " ")}
                                            </span>
                                        </div>
                                        <p className="text-sm font-medium leading-relaxed">
                                            {notification.message}
                                        </p>
                                        <p className="text-xs text-muted-foreground">
                                            Offer ID: <span className="font-mono">{notification.offer_id}</span>
                                        </p>
                                    </div>

                                    {!notification.read ? (
                                        <Button
                                            size="sm"
                                            variant="outline"
                                            onClick={() => mutation.mutate(notification.id)}
                                            disabled={mutation.isPending}
                                        >
                                            {mutation.isPending ? (
                                                <>
                                                    <Loader2 className="h-3.5 w-3.5 animate-spin" />
                                                    Updating...
                                                </>
                                            ) : (
                                                <>
                                                    <MailOpen className="h-3.5 w-3.5" />
                                                    Mark as read
                                                </>
                                            )}
                                        </Button>
                                    ) : (
                                        <div className="inline-flex items-center gap-1.5 rounded-md bg-muted px-3 py-2 text-xs text-muted-foreground">
                                            <MessageSquareWarning className="h-3.5 w-3.5" />
                                            Already acknowledged
                                        </div>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </CardContent>
        </Card>
    );
}
