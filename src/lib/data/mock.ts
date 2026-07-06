import type { Clip, Folder } from "$lib/types/clip";

export const mockFolders: Folder[] = [
  { id: "inbox", name: "Inbox", hotkey: "Ctrl Alt V", count: 42, color: "#3a8f74" },
  { id: "code", name: "Code", hotkey: "Ctrl Alt 1", count: 18, color: "#4d6fb3" },
  { id: "links", name: "Links", hotkey: "Ctrl Alt 2", count: 12, color: "#c45f42" },
  { id: "writing", name: "Writing", hotkey: "Ctrl Alt 3", count: 9, color: "#8a6f2a" },
  { id: "support", name: "Support", hotkey: "Ctrl Alt 4", count: 6, color: "#7c5c9f" },
];

export const mockClips: Clip[] = [
  {
    id: 1,
    folderId: "inbox",
    title: "SQLite migration note",
    content:
      "Keep clip content local by default. Store text in SQLite, index searchable text with FTS5, and keep binary payloads in an app-managed data folder.",
    source: "Notes",
    time: "2m",
    pinned: true,
    kind: "text",
  },
  {
    id: 2,
    folderId: "code",
    title: "Tauri command shape",
    content:
      '#[tauri::command]\nfn list_clips(folder_id: String, query: Option<String>) -> Vec<ClipSummary> {\n    // read from SQLite and return summaries\n}',
    source: "Cursor",
    time: "11m",
    pinned: true,
    kind: "code",
  },
  {
    id: 3,
    folderId: "links",
    title: "Tauri architecture",
    content: "https://tauri.app/concept/architecture/",
    source: "Arc",
    time: "24m",
    pinned: false,
    kind: "link",
  },
  {
    id: 4,
    folderId: "writing",
    title: "Project positioning",
    content:
      "A keyboard-first clipboard manager for people who live in folders, snippets, and repeated context switching.",
    source: "ChatGPT",
    time: "36m",
    pinned: true,
    kind: "text",
  },
  {
    id: 5,
    folderId: "support",
    title: "Reply template",
    content:
      "Thanks for the report. I reproduced the issue and will ship a fix in the next patch. For now, the workaround is to disable direct paste and use copy-only mode.",
    source: "Mail",
    time: "1h",
    pinned: false,
    kind: "text",
  },
  {
    id: 6,
    folderId: "code",
    title: "Search query",
    content:
      "select id, title, content_text from clips where folder_id = ? order by pinned desc, used_at desc limit 80;",
    source: "TablePlus",
    time: "2h",
    pinned: false,
    kind: "code",
  },
  {
    id: 7,
    folderId: "inbox",
    title: "Privacy baseline",
    content:
      "No account, no cloud sync, no telemetry. The app database and attachments should live in one user-visible local data location.",
    source: "Obsidian",
    time: "3h",
    pinned: false,
    kind: "text",
  },
];
