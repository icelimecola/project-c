<script lang="ts">
  type Props = {
    query: string;
    isCapturing?: boolean;
    isMonitorSaving?: boolean;
    monitorEnabled?: boolean;
    onCaptureClipboard: () => Promise<void>;
    onToggleMonitor: () => Promise<void>;
  };

  let {
    query = $bindable(),
    isCapturing = false,
    isMonitorSaving = false,
    monitorEnabled = true,
    onCaptureClipboard,
    onToggleMonitor,
  }: Props = $props();
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
  <button type="button" class="capture-button" disabled={isCapturing} onclick={onCaptureClipboard}>
    {isCapturing ? "Capturing" : "Capture"}
  </button>
  <button type="button" class:active={monitorEnabled} class="monitor-button" disabled={isMonitorSaving} onclick={onToggleMonitor}>
    {monitorEnabled ? "Monitor On" : "Monitor Off"}
  </button>
</header>

<style>
  .toolbar {
    display: grid;
    grid-template-columns: 1fr auto auto auto;
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

  .capture-button,
  .monitor-button {
    height: 36px;
    padding: 0 13px;
    border: 0;
    border-radius: 8px;
    background: #1e6f5c;
    color: #ffffff;
    cursor: pointer;
    font-size: 13px;
    font-weight: 750;
  }

  .capture-button:disabled {
    cursor: not-allowed;
    opacity: 0.56;
  }

  .monitor-button {
    border: 1px solid rgba(37, 50, 45, 0.14);
    background: #ffffff;
    color: #4e5b54;
  }

  .monitor-button.active {
    border-color: rgba(30, 111, 92, 0.26);
    background: #e6f4ef;
    color: #1e6f5c;
  }

  .monitor-button:disabled {
    cursor: not-allowed;
    opacity: 0.56;
  }

  @media (max-width: 680px) {
    .toolbar {
      grid-template-columns: 1fr;
    }
  }
</style>
