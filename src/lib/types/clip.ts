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
  source: string;
  time: string;
  pinned: boolean;
  kind: ClipKind;
};
