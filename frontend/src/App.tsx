import { BrowserRouter as Router, Routes, Route, Link, useLocation } from "react-router-dom";
import { OffersExplorer } from "./pages/OffersExplorer";
import { StudentDashboard } from "./pages/StudentDashboard";
import { StudentLogin } from "./components/StudentLogin";
import { Toaster } from "@/components/ui/sonner";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { MapPin, UserCircle, Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import "./App.css";

function ThemeToggle() {
    const { theme, setTheme } = useTheme();
    return (
        <Button
            variant="ghost"
            size="icon"
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            aria-label="Toggle theme"
        >
            <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
            <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        </Button>
    );
}

function Navbar() {
    const location = useLocation();
    const isDashboard = location.pathname.includes("/dashboard") || location.pathname.includes("/login");

    return (
        <header className="bg-background sticky top-0 z-50">
            <div className="container mx-auto px-4 h-14 flex items-center justify-between max-w-7xl">
                <Link to="/" className="flex items-center gap-2 text-lg font-bold text-primary">
                    <MapPin className="w-5 h-5" /> Polymove
                </Link>
                <nav className="flex items-center gap-1">
                    <Button variant={!isDashboard ? "secondary" : "ghost"} size="sm" asChild>
                        <Link to="/">Explorer</Link>
                    </Button>
                    <Button variant={isDashboard ? "secondary" : "ghost"} size="sm" asChild>
                        <Link to="/dashboard" className="flex items-center gap-1.5">
                            <UserCircle className="w-4 h-4" /> Dashboard
                        </Link>
                    </Button>
                    <ThemeToggle />
                </nav>
            </div>
            <Separator />
        </header>
    );
}

function App() {
    return (
        <Router>
            <div className="min-h-screen bg-background font-sans antialiased text-foreground">
                <Navbar />
                <main>
                    <Routes>
                        <Route path="/" element={<OffersExplorer />} />
                        <Route path="/dashboard" element={<StudentDashboard />} />
                        <Route path="/login" element={<StudentLogin />} />
                    </Routes>
                </main>
                <Toaster position="bottom-right" richColors />
            </div>
        </Router>
    );
}

export default App;
