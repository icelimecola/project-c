<script lang="ts">
  type Folder = {
    id: string;
    name: string;
    hotkey: string;
    count: number;
    color: string;
  };

  type Clip = {
    id: number;
    folderId: string;
    title: string;
    content: string;
    source: string;
    time: string;
    pinned: boolean;
    kind: "text" | "code" | "link";
  };

  const folders: Folder[] = [
    { id: "inbox", name: "Inbox", hotkey: "Ctrl Alt V", count: 42, color: "#3a8f74" },
    { id: "code", name: "Code", hotkey: "Ctrl Alt 1", count: 18, color: "#4d6fb3" },
    { id: "links", name: "Links", hotkey: "Ctrl Alt 2", count: 12, color: "#c45f42" },
    { id: "writing", name: "Writing", hotkey: "Ctrl Alt 3", count: 9, color: "#8a6f2a" },
    { id: "support", name: "Support", hotkey: "Ctrl Alt 4", count: 6, color: "#7c5c9f" },
  ];

  const clips: Clip[] = [
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

  let selectedFolderId = $state("inbox");
  let selectedClipId = $state(1);
  let query = $state("");

  const activeFolder = $derived(folders.find((folder) => folder.id === selectedFolderId) ?? folders[0]);

  const visibleClips = $derived(
    clips
      .filter((clip) => selectedFolderId === "inbox" || clip.folderId === selectedFolderId)
      .filter((clip) => {
        const needle = query.trim().toLowerCase();
        if (!needle) return true;

        return `${clip.title} ${clip.content} ${clip.source}`.toLowerCase().includes(needle);
      })
      .sort((a, b) => Number(b.pinned) - Number(a.pinned) || a.id - b.id),
  );

  const selectedClip = $derived(
    visibleClips.find((clip) => clip.id === selectedClipId) ?? visibleClips[0] ?? clips[0],
  );

  function chooseFolder(folderId: string) {
    selectedFolderId = folderId;
    const firstClip = clips.find((clip) => folderId === "inbox" || clip.folderId === folderId);
    selectedClipId = firstClip?.id ?? selectedClipId;
  }

  function chooseClip(clipId: number) {
    selectedClipId = clipId;
  }

  function handleKeyboard(event: KeyboardEvent) {
    if (!visibleClips.length) return;

    const currentIndex = Math.max(
      0,
      visibleClips.findIndex((clip) => clip.id === selectedClipId),
    );

    if (event.key === "ArrowDown") {
      event.preventDefault();
      selectedClipId = visibleClips[Math.min(currentIndex + 1, visibleClips.length - 1)].id;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      selectedClipId = visibleClips[Math.max(currentIndex - 1, 0)].id;
    }
  }
</script>

<svelte:head>
  <title>Project C</title>
</svelte:head>

<svelte:window onkeydown={handleKeyboard} />

<main class="app-shell">
  <section class="workspace" aria-label="Clipboard workspace">
    <aside class="sidebar" aria-label="Folders">
      <div class="brand">
        <div class="brand-mark">C</div>
        <div>
          <h1>Project C</h1>
          <p>Local clipboard workspace</p>
        </div>
      </div>

      <nav class="folder-list" aria-label="Clip folders">
        {#each folders as folder}
          <button
            class:active={folder.id === selectedFolderId}
            type="button"
            class="folder-button"
            onclick={() => chooseFolder(folder.id)}
          >
            <span class="folder-swatch" style={`--folder-color: ${folder.color}`}></span>
            <span class="folder-name">{folder.name}</span>
            <span class="folder-count">{folder.count}</span>
            <span class="folder-hotkey">{folder.hotkey}</span>
          </button>
        {/each}
      </nav>

      <div class="storage-panel">
        <div>
          <span class="storage-label">Local store</span>
          <strong>2.4 MB</strong>
        </div>
        <div class="storage-bar" aria-hidden="true">
          <span></span>
        </div>
      </div>
    </aside>

    <section class="clip-column" aria-label="Clips">
      <header class="toolbar">
        <div class="search-wrap">
          <span class="search-icon" aria-hidden="true"></span>
          <input bind:value={query} placeholder="Search clips" aria-label="Search clips" />
        </div>
        <div class="view-tabs" aria-label="Clip filters">
          <button type="button" class="selected">Recent</button>
          <button type="button">Pinned</button>
        </div>
      </header>

      <div class="section-title">
        <div>
          <span>{activeFolder.name}</span>
          <strong>{visibleClips.length} clips</strong>
        </div>
      </div>

      <div class="clip-list" aria-label="Clip list">
        {#each visibleClips as clip}
          <button
            class:active={clip.id === selectedClip.id}
            type="button"
            class="clip-row"
            onclick={() => chooseClip(clip.id)}
          >
            <span class={`clip-kind ${clip.kind}`}>{clip.kind}</span>
            <span class="clip-main">
              <span class="clip-title">
                {clip.title}
                {#if clip.pinned}
                  <span class="pin-dot" aria-label="Pinned"></span>
                {/if}
              </span>
              <span class="clip-body">{clip.content}</span>
            </span>
            <span class="clip-meta">
              <span>{clip.source}</span>
              <span>{clip.time}</span>
            </span>
          </button>
        {/each}
      </div>
    </section>

    <aside class="preview" aria-label="Clip preview">
      <header class="preview-header">
        <div>
          <span class={`clip-kind ${selectedClip.kind}`}>{selectedClip.kind}</span>
          <h2>{selectedClip.title}</h2>
        </div>
        <button type="button" class="icon-button" aria-label="Pin clip">
          <span aria-hidden="true">P</span>
        </button>
      </header>

      <div class="preview-content">
        <pre>{selectedClip.content}</pre>
      </div>

      <dl class="details">
        <div>
          <dt>Source</dt>
          <dd>{selectedClip.source}</dd>
        </div>
        <div>
          <dt>Folder</dt>
          <dd>{folders.find((folder) => folder.id === selectedClip.folderId)?.name}</dd>
        </div>
        <div>
          <dt>Last copied</dt>
          <dd>{selectedClip.time} ago</dd>
        </div>
      </dl>

      <footer class="actions">
        <button type="button" class="secondary-action">Copy</button>
        <button type="button" class="primary-action">Paste</button>
      </footer>
    </aside>
  </section>
</main>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(html),
  :global(body) {
    margin: 0;
    min-width: 320px;
    height: 100%;
    overflow: hidden;
    font-family:
      Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    color: #18201d;
    background: #e9edf1;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  .app-shell {
    height: 100vh;
    padding: 18px;
    background:
      linear-gradient(135deg, rgba(235, 239, 243, 0.94), rgba(224, 230, 235, 0.94)),
      #e9edf1;
  }

  .workspace {
    display: grid;
    grid-template-columns: minmax(190px, 230px) minmax(320px, 1fr) minmax(260px, 360px);
    width: min(1180px, 100%);
    height: calc(100vh - 36px);
    margin: 0 auto;
    overflow: hidden;
    border: 1px solid rgba(37, 50, 45, 0.14);
    border-radius: 8px;
    background: rgba(251, 252, 253, 0.96);
    box-shadow: 0 24px 80px rgba(40, 51, 47, 0.16);
  }

  .sidebar,
  .clip-column,
  .preview {
    min-width: 0;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    gap: 22px;
    padding: 18px;
    background: #f3f6f8;
    border-right: 1px solid rgba(49, 61, 70, 0.13);
  }

  .brand {
    display: grid;
    grid-template-columns: 38px 1fr;
    gap: 12px;
    align-items: center;
    min-height: 48px;
  }

  .brand-mark {
    display: grid;
    width: 38px;
    height: 38px;
    place-items: center;
    border-radius: 8px;
    background: #1d2723;
    color: #f7fbfa;
    font-weight: 750;
  }

  .brand h1,
  .brand p,
  .preview h2 {
    margin: 0;
  }

  .brand h1 {
    font-size: 18px;
    line-height: 22px;
  }

  .brand p {
    color: #64717a;
    font-size: 12px;
    line-height: 16px;
  }

  .folder-list {
    display: grid;
    gap: 5px;
  }

  .folder-button,
  .clip-row,
  .view-tabs button,
  .icon-button,
  .primary-action,
  .secondary-action {
    border: 0;
    cursor: pointer;
  }

  .folder-button {
    display: grid;
    grid-template-columns: 12px 1fr auto;
    grid-template-rows: auto auto;
    column-gap: 10px;
    row-gap: 2px;
    align-items: center;
    min-height: 52px;
    padding: 9px 10px;
    border-radius: 8px;
    color: #2c352f;
    background: transparent;
    text-align: left;
  }

  .folder-button:hover,
  .folder-button.active {
    background: rgba(255, 255, 255, 0.78);
  }

  .folder-swatch {
    grid-row: 1 / 3;
    width: 10px;
    height: 22px;
    border-radius: 8px;
    background: var(--folder-color);
  }

  .folder-name,
  .clip-title {
    overflow: hidden;
    font-weight: 650;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .folder-count {
    padding: 2px 7px;
    border-radius: 999px;
    background: rgba(44, 53, 47, 0.08);
    color: #4d5a53;
    font-size: 11px;
  }

  .folder-hotkey {
    grid-column: 2 / 4;
    color: #7a7164;
    font-size: 11px;
    line-height: 14px;
  }

  .storage-panel {
    margin-top: auto;
    padding: 12px;
    border: 1px solid rgba(49, 61, 70, 0.12);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.54);
  }

  .storage-panel > div:first-child {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    font-size: 12px;
  }

  .storage-label {
    color: #746b5d;
  }

  .storage-bar {
    height: 6px;
    margin-top: 10px;
    overflow: hidden;
    border-radius: 999px;
    background: rgba(44, 53, 47, 0.1);
  }

  .storage-bar span {
    display: block;
    width: 34%;
    height: 100%;
    background: #3a8f74;
  }

  .clip-column {
    display: flex;
    flex-direction: column;
    min-height: 0;
    background: #fbfcfd;
  }

  .toolbar {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 14px;
    align-items: center;
    min-height: 72px;
    padding: 16px 18px;
    border-bottom: 1px solid rgba(37, 50, 45, 0.1);
  }

  .search-wrap {
    position: relative;
    min-width: 0;
  }

  .search-icon {
    position: absolute;
    top: 50%;
    left: 12px;
    width: 13px;
    height: 13px;
    border: 2px solid #79827b;
    border-radius: 999px;
    transform: translateY(-58%);
  }

  .search-icon::after {
    position: absolute;
    right: -5px;
    bottom: -4px;
    width: 6px;
    height: 2px;
    border-radius: 999px;
    background: #79827b;
    content: "";
    transform: rotate(45deg);
  }

  .search-wrap input {
    width: 100%;
    height: 40px;
    padding: 0 12px 0 36px;
    border: 1px solid rgba(37, 50, 45, 0.16);
    border-radius: 8px;
    outline: 0;
    background: #ffffff;
    color: #18201d;
  }

  .search-wrap input:focus {
    border-color: #3a8f74;
    box-shadow: 0 0 0 3px rgba(58, 143, 116, 0.16);
  }

  .view-tabs {
    display: inline-grid;
    grid-template-columns: repeat(2, minmax(72px, 1fr));
    height: 36px;
    padding: 3px;
    border-radius: 8px;
    background: #e8ece7;
  }

  .view-tabs button {
    min-width: 0;
    padding: 0 10px;
    border-radius: 6px;
    background: transparent;
    color: #5c6660;
    font-size: 13px;
  }

  .view-tabs .selected {
    background: #ffffff;
    color: #18201d;
    box-shadow: 0 1px 8px rgba(37, 50, 45, 0.1);
  }

  .section-title {
    display: flex;
    justify-content: space-between;
    min-height: 52px;
    padding: 12px 18px;
    border-bottom: 1px solid rgba(37, 50, 45, 0.08);
  }

  .section-title span,
  .section-title strong {
    display: block;
  }

  .section-title span {
    font-size: 14px;
    font-weight: 700;
  }

  .section-title strong {
    margin-top: 2px;
    color: #68726c;
    font-size: 12px;
  }

  .clip-list {
    display: grid;
    gap: 4px;
    flex: 1;
    min-height: 0;
    overflow: auto;
    padding: 8px;
  }

  .clip-row {
    display: grid;
    grid-template-columns: 48px minmax(0, 1fr) 70px;
    gap: 12px;
    align-items: start;
    min-height: 86px;
    padding: 12px;
    border: 1px solid transparent;
    border-radius: 8px;
    background: transparent;
    color: #18201d;
    text-align: left;
  }

  .clip-row:hover,
  .clip-row.active {
    border-color: rgba(58, 143, 116, 0.22);
    background: #eef6f1;
  }

  .clip-kind {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 24px;
    border-radius: 999px;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
  }

  .clip-kind.text {
    background: #e5f1eb;
    color: #246b55;
  }

  .clip-kind.code {
    background: #e7ebf6;
    color: #395c9e;
  }

  .clip-kind.link {
    background: #f6ebe6;
    color: #9e4c34;
  }

  .clip-main {
    display: grid;
    min-width: 0;
    gap: 6px;
  }

  .clip-title {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 14px;
  }

  .pin-dot {
    flex: 0 0 auto;
    width: 7px;
    height: 7px;
    border-radius: 999px;
    background: #b7872f;
  }

  .clip-body {
    display: -webkit-box;
    overflow: hidden;
    color: #58625c;
    font-size: 12px;
    line-height: 18px;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    line-clamp: 2;
  }

  .clip-meta {
    display: grid;
    justify-items: end;
    gap: 4px;
    color: #77817a;
    font-size: 11px;
    line-height: 15px;
    white-space: nowrap;
  }

  .preview {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 18px;
    border-left: 1px solid rgba(37, 50, 45, 0.1);
    background: #f7f9fb;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    gap: 14px;
  }

  .preview-header h2 {
    margin-top: 9px;
    font-size: 19px;
    line-height: 24px;
  }

  .icon-button {
    display: grid;
    flex: 0 0 auto;
    width: 36px;
    height: 36px;
    place-items: center;
    border-radius: 8px;
    background: #e7ece7;
    color: #3d4a43;
    font-weight: 750;
  }

  .preview-content {
    min-height: 220px;
    overflow: auto;
    border: 1px solid rgba(37, 50, 45, 0.12);
    border-radius: 8px;
    background: #ffffff;
  }

  .preview-content pre {
    margin: 0;
    padding: 16px;
    white-space: pre-wrap;
    word-break: break-word;
    color: #26312c;
    font-family:
      "SFMono-Regular", Consolas, "Liberation Mono", Menlo, ui-monospace, monospace;
    font-size: 13px;
    line-height: 20px;
  }

  .details {
    display: grid;
    gap: 10px;
    margin: 0;
  }

  .details div {
    display: grid;
    grid-template-columns: 88px minmax(0, 1fr);
    gap: 12px;
    min-height: 28px;
    align-items: center;
  }

  .details dt {
    color: #747d76;
    font-size: 12px;
  }

  .details dd {
    min-width: 0;
    margin: 0;
    overflow: hidden;
    color: #26312c;
    font-size: 13px;
    font-weight: 650;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    margin-top: auto;
  }

  .primary-action,
  .secondary-action {
    height: 40px;
    border-radius: 8px;
    font-weight: 700;
  }

  .primary-action {
    background: #1e6f5c;
    color: #ffffff;
  }

  .secondary-action {
    border: 1px solid rgba(37, 50, 45, 0.14);
    background: #ffffff;
    color: #26312c;
  }

  @media (max-width: 940px) {
    .app-shell {
      padding: 0;
    }

    .workspace {
      grid-template-columns: 190px minmax(0, 1fr);
      min-height: 100vh;
      height: 100vh;
      border: 0;
      border-radius: 0;
    }

    .preview {
      display: none;
    }
  }

  @media (max-width: 680px) {
    .workspace {
      grid-template-columns: 1fr;
    }

    .sidebar {
      border-right: 0;
      border-bottom: 1px solid rgba(37, 50, 45, 0.1);
    }

    .folder-list {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .storage-panel {
      display: none;
    }

    .toolbar {
      grid-template-columns: 1fr;
    }

    .clip-row {
      grid-template-columns: 44px minmax(0, 1fr);
    }

    .clip-meta {
      grid-column: 2;
      grid-template-columns: auto auto;
      justify-content: start;
    }
  }
</style>
