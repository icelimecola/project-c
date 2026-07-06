<script lang="ts">
  import ClipList from "$lib/components/ClipList.svelte";
  import ClipPreview from "$lib/components/ClipPreview.svelte";
  import SearchToolbar from "$lib/components/SearchToolbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import { mockClips, mockFolders } from "$lib/data/mock";

  const folders = mockFolders;
  const clips = mockClips;

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

  const selectedClipFolderName = $derived(
    folders.find((folder) => folder.id === selectedClip.folderId)?.name ?? "",
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
    <Sidebar {folders} {selectedFolderId} onChooseFolder={chooseFolder} />

    <section class="clip-column" aria-label="Clips">
      <SearchToolbar bind:query />

      <div class="section-title">
        <div>
          <span>{activeFolder.name}</span>
          <strong>{visibleClips.length} clips</strong>
        </div>
      </div>

      <ClipList clips={visibleClips} selectedClipId={selectedClip.id} onChooseClip={chooseClip} />
    </section>

    <ClipPreview clip={selectedClip} folderName={selectedClipFolderName} />
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

  .clip-column {
    display: flex;
    min-width: 0;
    min-height: 0;
    flex-direction: column;
    background: #fbfcfd;
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
  }

  @media (max-width: 680px) {
    .workspace {
      grid-template-columns: 1fr;
    }
  }
</style>
