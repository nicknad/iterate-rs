export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="dark min-h-screen bg-background text-foreground flex items-center justify-center">
      <div className="w-full max-w-5xl p-4">{children}</div>
    </div>
  );
}
