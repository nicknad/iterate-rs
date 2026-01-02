import { load } from "@tauri-apps/plugin-store";

// This creates (or loads) a file called "settings.json" in the AppData folder
const store = await load("last_use.json");

export const getDbHistory = async () => {
  const name = await store.get<string>("last_db_name");
  const path = await store.get<string>("last_db_path");
  return { name, path };
};
