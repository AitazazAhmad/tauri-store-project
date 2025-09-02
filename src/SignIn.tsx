import { useState, FormEvent, ChangeEvent } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

type SignInProps = {
    onLoginSuccess: (email: string) => void;
};

export default function SignIn({ onLoginSuccess }: SignInProps) {
    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [message, setMessage] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const navigate = useNavigate();

    const handleLogin = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setIsLoading(true);
        setMessage("");

        try {
            // ✅ Updated command names
            const user: { email: string; password: string } | null = await invoke("get_user", { email });

            if (user && user.password === password) {
                await invoke("set_current_user", { email }); // ✅ updated
                onLoginSuccess(email);
                navigate("/afterlogin"); // make sure this route matches your AfterLogin component
            } else {
                setMessage("❌ Invalid email or password");
            }
        } catch (error) {
            console.error("Login error:", error);
            setMessage("❌ An error occurred during login");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <main className="container">
            <h1>Login</h1>
            <form className="form" onSubmit={handleLogin}>
                <input
                    type="email"
                    placeholder="Email"
                    value={email}
                    required
                    disabled={isLoading}
                    onChange={(e: ChangeEvent<HTMLInputElement>) => setEmail(e.target.value)}
                />
                <input
                    type="password"
                    placeholder="Password"
                    value={password}
                    required
                    disabled={isLoading}
                    onChange={(e: ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)}
                />
                <button type="submit" disabled={isLoading}>
                    {isLoading ? "Processing..." : "Login"}
                </button>
            </form>

            <p className="toggle">
                Don't have an account?{" "}
                <button
                    type="button"
                    onClick={() => navigate("/signup")}
                    disabled={isLoading}
                >
                    Sign Up
                </button>
            </p>

            {message && <p className={message.includes("❌") ? "error" : "success"}>{message}</p>}
        </main>
    );
}
