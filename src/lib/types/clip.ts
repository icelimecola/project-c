export type ClipKind = "text" | "code" | "link";

export type Folder = {
  id: string;
  name: string;
  hotkey: string;
  count: number;
  color: string;
};

export type Clip = {
  id: number;
  folderId: string;
  title: string;
  content: string;
  contentHash: string;
  source: string;
  sourceApp?: string | null;
  time: string;
  createdAt: string;
  updatedAt: string;
  lastUsedAt?: string | null;
  mimeType: string;
  deletedAt?: string | null;
  pinned: boolean;
  kind: ClipKind;
};
