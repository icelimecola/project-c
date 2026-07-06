<script lang="ts">
  import type { Folder } from "$lib/types/clip";

  type Props = {
    folders: Folder[];
    selectedFolderId: string;
    onChooseFolder: (folderId: string) => void;
  };

  let { folders, selectedFolderId, onChooseFolder }: Props = $props();
</script>

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
        onclick={() => onChooseFolder(folder.id)}
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

<style>
  .sidebar {
    display: flex;
    min-width: 0;
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
  .brand p {
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

  .folder-button {
    display: grid;
    grid-template-columns: 12px 1fr auto;
    grid-template-rows: auto auto;
    column-gap: 10px;
    row-gap: 2px;
    align-items: center;
    min-height: 52px;
    padding: 9px 10px;
    border: 0;
    border-radius: 8px;
    background: transparent;
    color: #2c352f;
    cursor: pointer;
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

  .folder-name {
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

  @media (max-width: 680px) {
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
  }
</style>
