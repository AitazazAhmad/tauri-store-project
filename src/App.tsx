import { useState, useEffect } from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import SignIn from "./SignIn";
import SignUp from "./SignUp";
import AfterLogin from "./afterlogin"; // ensure filename casing matches
import "./App.css";

export default function App() {
  const [currentUser, setCurrentUser] = useState<string | null>(null);

  useEffect(() => {
    const fetchCurrentUser = async () => {
      try {
        const user = await invoke("get_current_user");
        setCurrentUser(user as string);
      } catch (error) {
        console.error("Error fetching current user:", error);
      }
    };
    fetchCurrentUser();
  }, []);

  const handleLogout = async () => {
    try {
      await invoke("clear_current_user");
      setCurrentUser(null);
    } catch (error) {
      console.error("Logout error:", error);
    }
  };

  return (
    <Router>
      <Routes>
        <Route path="/" element={<SignIn onLoginSuccess={setCurrentUser} />} />
        <Route path="/signup" element={<SignUp onSignupSuccess={() => console.log("Signup success")} />} />
        <Route
          path="/afterlogin"
          element={<AfterLogin userEmail={currentUser} onLogout={handleLogout} />}
        />
      </Routes>
    </Router>
  );
}
