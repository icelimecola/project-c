<script lang="ts">
  import ClipKindBadge from "$lib/components/ClipKindBadge.svelte";
  import type { Clip } from "$lib/types/clip";

  type Props = {
    clips: Clip[];
    selectedClipId: number;
    onChooseClip: (clipId: number) => void;
  };

  let { clips, selectedClipId, onChooseClip }: Props = $props();
</script>

<div class="clip-list" aria-label="Clip list">
  {#each clips as clip}
    <button
      class:active={clip.id === selectedClipId}
      type="button"
      class="clip-row"
      onclick={() => onChooseClip(clip.id)}
    >
      <ClipKindBadge kind={clip.kind} />
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

<style>
  .clip-list {
    display: grid;
    flex: 1;
    gap: 4px;
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
    cursor: pointer;
    text-align: left;
  }

  .clip-row:hover,
  .clip-row.active {
    border-color: rgba(58, 143, 116, 0.22);
    background: #eef6f1;
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
    overflow: hidden;
    font-size: 14px;
    font-weight: 650;
    text-overflow: ellipsis;
    white-space: nowrap;
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

  @media (max-width: 680px) {
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
