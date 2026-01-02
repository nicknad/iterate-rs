import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

export const Route = createFileRoute("/UnlockJournal")({
  component: UnlockComponent,
});

type UnlockJournalError =
  | { type: "Cancelled" }
  | { type: "InvalidPassword" }
  | { type: "InternalError"; message: string };

function UnlockComponent() {
  const navigate = useNavigate();
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);

  const unlock = async () => {
    setError(null);
    const encoder = new TextEncoder();
    const passwordBytes = encoder.encode(password);
    setPassword("");

    try {
      await invoke("unlock_journal", {
        password: Array.from(passwordBytes), // Tauri-safe
      });
    } catch (err) {
      let unlock_err = err as UnlockJournalError;
      switch (unlock_err.type) {
        case "Cancelled":
          navigate({ to: "/" });
          break;
        case "InternalError":
          setError(unlock_err.message);
          break;
        case "InvalidPassword":
          setError("Password is wrong");
          break;
      }
    } finally {
      passwordBytes.fill(0);
    }
  };

  return (
    <>
      <div className="flex min-h-screen items-center justify-center">
        <Card className="w-full max-w-xl">
          <CardContent className="space-y-6 px-8 py-7">
            <div className="space-y-4 text-left">
              <label className="text-sm font-medium">Journal password</label>
              <form
                onSubmit={(e) => {
                  e.preventDefault(); // prevent page reload
                  unlock();
                }}
              >
                <Input
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  autoComplete="off"
                />
                <button type="submit" hidden />
              </form>
            </div>

            {error && (
              <div className="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-2 text-sm text-destructive">
                {error}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </>
  );
}
