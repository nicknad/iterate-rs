import { useState } from "react";
import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { Input } from "@/components/ui/input";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";

export const Route = createFileRoute("/CreateJournal")({
  component: NewJournal,
});

function NewJournal() {
  const encoder = new TextEncoder();
  const navigate = useNavigate();
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleConfirm = async () => {
    setError(null);

    // Strict Validation
    if (!password || !confirm) {
      setPassword("");
      setConfirm("");
      setError("Both fields are required.");
      return;
    }

    if (password !== confirm) {
      setPassword("");
      setConfirm("");
      setError("Passwords do not match.");
      return;
    }

    if (password.length < 8) {
      setPassword("");
      setConfirm("");
      setError("Password must be at least 8 characters.");

      return;
    }

    setLoading(true);
    try {
      const passwordBytes = encoder.encode(password);
      await invoke("create_journal", {
        password: Array.from(passwordBytes),
      });
      // TODO: On success, go to the dashboard or success page
      setPassword("");
      setConfirm("");
      navigate({ to: "/" });
    } catch (err: any) {
      if (err.type === "Cancelled") return;
      setError(err.message || "An unexpected error occurred.");
    } finally {
      setLoading(false);
    }
  };
  return (
    <div className="flex min-h-screen items-center justify-center">
      <Card className="w-full max-w-xl">
        <CardContent className="space-y-6 px-8 py-7">
          {/* Header */}
          <div className="space-y-2 text-center">
            <h1 className="text-xl font-semibold tracking-tight">
              Choose your master password
            </h1>
            <p className="text-sm text-muted-foreground">
              This password encrypts your journal.
            </p>
          </div>

          {error && (
            <div className="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-2 text-sm text-destructive">
              {error}
            </div>
          )}

          {/* Warning */}
          <div className="rounded-md border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm">
            <p className="font-medium text-destructive">Attention</p>
            <p className="text-muted-foreground">
              There is no way to recover a forgotten password. If you lose it,
              your journal is permanently inaccessible.
            </p>
          </div>

          {/* Form */}
          <div className="space-y-4 text-left">
            <form
              onSubmit={(e) => {
                e.preventDefault(); // prevent page reload
                handleConfirm();
              }}
            >
              <div className="space-y-1">
                <label className="text-sm font-medium">Master password</label>
                <Input
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Enter a strong password"
                  autoComplete="off"
                />
              </div>

              <div className="space-y-1">
                <label className="text-sm font-medium">Confirm password</label>
                <Input
                  type="password"
                  value={confirm}
                  onChange={(e) => setConfirm(e.target.value)}
                  placeholder="Repeat your password"
                  autoComplete="off"
                />
              </div>
              <button type="submit" hidden />
            </form>
          </div>

          <div className="pt-4">
            <Button
              className="w-full cursor-pointer"
              onClick={handleConfirm}
              disabled={loading}
            >
              {loading ? "Creating..." : "Confirm"}
            </Button>
          </div>

          {/* Back */}
          <div className="text-center">
            <Link
              to="/"
              className="text-sm text-muted-foreground hover:text-foreground"
            >
              Back
            </Link>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
