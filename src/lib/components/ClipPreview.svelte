<script lang="ts">
  import ClipKindBadge from "$lib/components/ClipKindBadge.svelte";
  import type { Clip } from "$lib/types/clip";

  type Props = {
    clip: Clip;
    folderName?: string;
  };

  let { clip, folderName = "" }: Props = $props();
</script>

<aside class="preview" aria-label="Clip preview">
  <header class="preview-header">
    <div>
      <ClipKindBadge kind={clip.kind} />
      <h2>{clip.title}</h2>
    </div>
    <button type="button" class="icon-button" aria-label="Pin clip">
      <span aria-hidden="true">P</span>
    </button>
  </header>

  <div class="preview-content">
    <pre>{clip.content}</pre>
  </div>

  <dl class="details">
    <div>
      <dt>Source</dt>
      <dd>{clip.source}</dd>
    </div>
    <div>
      <dt>Folder</dt>
      <dd>{folderName}</dd>
    </div>
    <div>
      <dt>Last copied</dt>
      <dd>{clip.time} ago</dd>
    </div>
  </dl>

  <footer class="actions">
    <button type="button" class="secondary-action">Copy</button>
    <button type="button" class="primary-action">Paste</button>
  </footer>
</aside>

<style>
  .preview {
    display: flex;
    min-width: 0;
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
    margin: 9px 0 0;
    font-size: 19px;
    line-height: 24px;
  }

  .icon-button {
    display: grid;
    flex: 0 0 auto;
    width: 36px;
    height: 36px;
    place-items: center;
    border: 0;
    border-radius: 8px;
    background: #e7ece7;
    color: #3d4a43;
    cursor: pointer;
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
    color: #26312c;
    font-family:
      "SFMono-Regular", Consolas, "Liberation Mono", Menlo, ui-monospace, monospace;
    font-size: 13px;
    line-height: 20px;
    white-space: pre-wrap;
    word-break: break-word;
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
    align-items: center;
    min-height: 28px;
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
    border: 0;
    border-radius: 8px;
    cursor: pointer;
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
    .preview {
      display: none;
    }
  }
</style>
