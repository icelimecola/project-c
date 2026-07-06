<script lang="ts">
  import type { ClipKind, Folder } from "$lib/types/clip";

  type CreateClipInput = {
    folderId: string;
    title: string;
    content: string;
    source: string;
    kind: ClipKind;
  };

  type Props = {
    folders: Folder[];
    selectedFolderId: string;
    isSaving?: boolean;
    onCreateClip: (input: CreateClipInput) => Promise<void>;
  };

  let { folders, selectedFolderId, isSaving = false, onCreateClip }: Props = $props();

  let title = $state("");
  let content = $state("");
  let source = $state("");
  let kind = $state<ClipKind>("text");
  let targetFolderId = $state("inbox");

  const canSubmit = $derived(title.trim().length > 0 && content.trim().length > 0 && !isSaving);

  $effect(() => {
    if (selectedFolderId !== "inbox") {
      targetFolderId = selectedFolderId;
      return;
    }

    if (!folders.some((folder) => folder.id === targetFolderId)) {
      targetFolderId = "inbox";
    }
  });

  async function submit() {
    if (!canSubmit) return;

    await onCreateClip({
      folderId: targetFolderId,
      title,
      content,
      source,
      kind,
    });

    title = "";
    content = "";
    source = "";
    kind = "text";
  }
</script>

<form class="add-panel" aria-label="Add clip" onsubmit={(event) => event.preventDefault()}>
  <div class="top-row">
    <input bind:value={title} placeholder="Title" aria-label="Clip title" />
    <select bind:value={kind} aria-label="Clip kind">
      <option value="text">Text</option>
      <option value="code">Code</option>
      <option value="link">Link</option>
    </select>
  </div>

  <textarea bind:value={content} placeholder="Content" aria-label="Clip content"></textarea>

  <div class="bottom-row">
    <select bind:value={targetFolderId} aria-label="Target folder" disabled={selectedFolderId !== "inbox"}>
      {#each folders as folder}
        <option value={folder.id}>{folder.name}</option>
      {/each}
    </select>
    <input bind:value={source} placeholder="Source" aria-label="Clip source" />
    <button type="button" disabled={!canSubmit} onclick={submit}>
      {isSaving ? "Saving" : "Add"}
    </button>
  </div>
</form>

<style>
  .add-panel {
    display: grid;
    gap: 8px;
    padding: 12px 18px;
    border-bottom: 1px solid rgba(37, 50, 45, 0.08);
    background: #f3f6f4;
  }

  .top-row,
  .bottom-row {
    display: grid;
    gap: 8px;
  }

  .top-row {
    grid-template-columns: minmax(0, 1fr) 92px;
  }

  .bottom-row {
    grid-template-columns: 104px minmax(0, 1fr) 72px;
  }

  input,
  select,
  textarea {
    min-width: 0;
    border: 1px solid rgba(37, 50, 45, 0.16);
    border-radius: 8px;
    outline: 0;
    background: #ffffff;
    color: #18201d;
    font-size: 12px;
  }

  input,
  select {
    height: 34px;
    padding: 0 10px;
  }

  textarea {
    height: 62px;
    resize: none;
    padding: 9px 10px;
    line-height: 18px;
  }

  input:focus,
  select:focus,
  textarea:focus {
    border-color: #3a8f74;
    box-shadow: 0 0 0 3px rgba(58, 143, 116, 0.14);
  }

  select:disabled {
    color: #747d76;
  }

  button {
    height: 34px;
    border: 0;
    border-radius: 8px;
    background: #1e6f5c;
    color: #ffffff;
    cursor: pointer;
    font-size: 12px;
    font-weight: 750;
  }

  button:disabled {
    cursor: not-allowed;
    opacity: 0.54;
  }

  @media (max-width: 680px) {
    .top-row,
    .bottom-row {
      grid-template-columns: 1fr;
    }
  }
</style>
