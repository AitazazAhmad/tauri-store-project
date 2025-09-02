import { useState, FormEvent, ChangeEvent } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

type SignUpProps = {
    onSignupSuccess: () => void;
};

export default function SignUp({ onSignupSuccess }: SignUpProps) {
    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [confirmPassword, setConfirmPassword] = useState<string>("");
    const [message, setMessage] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const navigate = useNavigate();

    const handleSignup = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setIsLoading(true);
        setMessage("");

        if (password !== confirmPassword) {
            setMessage("❌ Passwords do not match");
            setIsLoading(false);
            return;
        }

        try {
            // ✅ match Rust command: get_user
            const existingUser: { email: string } | null = await invoke("get_user", { email });

            if (existingUser) {
                setMessage("⚠️ User already exists");
                setIsLoading(false);
                return;
            }

            // ✅ match Rust command: create_user
            await invoke("create_user", { email, password });

            setMessage("✅ Signup successful! You can now log in.");
            setEmail("");
            setPassword("");
            setConfirmPassword("");
            onSignupSuccess();
            navigate("/");
        } catch (error) {
            console.error("Signup error:", error);
            setMessage("❌ An error occurred during signup");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <main className="container">
            <h1>Sign Up</h1>
            <form className="form" onSubmit={handleSignup}>
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
                <input
                    type="password"
                    placeholder="Confirm Password"
                    value={confirmPassword}
                    required
                    disabled={isLoading}
                    onChange={(e: ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)}
                />
                <button type="submit" disabled={isLoading}>
                    {isLoading ? "Processing..." : "Sign Up"}
                </button>
            </form>

            <p className="toggle">
                Already have an account?{" "}
                <button type="button" onClick={() => navigate("/")} disabled={isLoading}>
                    Login
                </button>
            </p>

            {message && (
                <p className={message.includes("✅") ? "success" : "error"}>{message}</p>
            )}
        </main>
    );
}
