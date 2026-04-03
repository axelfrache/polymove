import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { GraduationCap } from "lucide-react";

export function StudentLogin() {
    const [studentId, setStudentId] = useState("");
    const navigate = useNavigate();

    const handleLogin = (e: React.FormEvent) => {
        e.preventDefault();
        if (studentId.trim()) {
            localStorage.setItem("studentId", studentId.trim());
            window.dispatchEvent(new Event("polymove-student-change"));
            navigate("/dashboard");
        }
    };

    return (
        <div className="flex h-[calc(100vh-3.5rem)] items-center justify-center px-4 bg-gradient-to-b from-muted/30 to-background">
            <Card className="w-full max-w-md shadow-xl border-0 ring-1 ring-border/50">
                <CardHeader className="space-y-4 pt-8 pb-6 px-8">
                    <div className="w-14 h-14 rounded-2xl bg-primary flex items-center justify-center mb-1">
                        <GraduationCap className="w-7 h-7 text-primary-foreground" />
                    </div>
                    <div className="space-y-1.5">
                        <CardTitle className="text-2xl font-bold">Student Login</CardTitle>
                        <CardDescription className="text-sm leading-relaxed">
                            Enter your Student ID to access your personalized internship recommendations.
                        </CardDescription>
                    </div>
                </CardHeader>
                <form onSubmit={handleLogin}>
                    <CardContent className="px-8 pb-4">
                        <div className="grid gap-2.5">
                            <Label htmlFor="studentId" className="font-medium">Student ID (UUID)</Label>
                            <Input
                                id="studentId"
                                placeholder="e.g. 123e4567-e89b-12d3-a456-426614174000"
                                value={studentId}
                                onChange={(e) => setStudentId(e.target.value)}
                                autoFocus
                                required
                                className="font-mono text-sm h-11"
                            />
                            <p className="text-xs text-muted-foreground">
                                Your UUID is provided by your academic institution.
                            </p>
                        </div>
                    </CardContent>
                    <CardFooter className="px-8 pb-8 pt-4">
                        <Button className="w-full h-11 text-base font-semibold" type="submit" disabled={!studentId.trim()}>
                            Access Dashboard
                        </Button>
                    </CardFooter>
                </form>
            </Card>
        </div>
    );
}
