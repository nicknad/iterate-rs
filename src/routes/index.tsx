import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Link, useNavigate, createFileRoute } from "@tanstack/react-router";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export const Route = createFileRoute("/")({
  component: JournalSelect,
});

type JournalError =
  | { type: "Cancelled" }
  | { type: "InternalError"; message: string };

function JournalSelect() {
  const navigate = useNavigate();
  const [error, setError] = useState<string | null>(null);

  const handleOpenJournal = async () => {
    setError(null);
    try {
      await invoke<string>("open_journal_file");
      // If we reach here, validation passed
      navigate({
        to: "/UnlockJournal",
        params: {},
      });
    } catch (err) {
      const error = err as JournalError;

      switch (error.type) {
        case "Cancelled":
          return; // Stay on page silently
        case "InternalError":
          setError(`System error: ${error.message}`);
          break;
      }
    }
  };
  return (
    <Card className="mx-auto max-w-xl">
      <CardContent className="space-y-6 p-8 text-center">
        <div>
          <h1 className="text-2xl font-semibold">Welcome to Iterate</h1>
          <p className="text-muted-foreground">
            Your private space for growth and reflection.
          </p>
        </div>

        <div className="flex gap-4 justify-center">
          <Button asChild variant="outline" className="cursor-pointer">
            <Link to="/CreateJournal">Create New Journal</Link>
          </Button>
          <Button
            variant="outline"
            className="cursor-pointer"
            onClick={handleOpenJournal}
          >
            Open Existing Journal
          </Button>
        </div>
        {error && <p className="text-destructive text-sm mt-2">{error}</p>}
        <div className="text-left">
          <p className="text-sm text-muted-foreground mb-2">Recent Journals</p>
          <div className="rounded-md border p-3">
            <p className="font-medium">Work Notes</p>
            <p className="text-xs text-muted-foreground">Last opened: Today</p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
