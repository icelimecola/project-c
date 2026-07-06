<script lang="ts">
  import type { ClipKind } from "$lib/types/clip";

  type Props = {
    kind: ClipKind;
  };

  let { kind }: Props = $props();
</script>

<span class={`clip-kind ${kind}`}>{kind}</span>

<style>
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
</style>
