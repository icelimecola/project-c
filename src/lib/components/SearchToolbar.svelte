<script lang="ts">
  type Props = {
    query: string;
  };

  let { query = $bindable() }: Props = $props();
</script>

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

<style>
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
    border: 0;
    border-radius: 6px;
    background: transparent;
    color: #5c6660;
    cursor: pointer;
    font-size: 13px;
  }

  .view-tabs .selected {
    background: #ffffff;
    color: #18201d;
    box-shadow: 0 1px 8px rgba(37, 50, 45, 0.1);
  }

  @media (max-width: 680px) {
    .toolbar {
      grid-template-columns: 1fr;
    }
  }
</style>
