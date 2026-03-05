<script>
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/stores';
  import Hls from 'hls.js';

  let videoEl;
  let status = 'loading';
  let error = null;
  let interval;
  let videoLoaded = false;
  let pollCount = 0;
  const MAX_POLLS = 150; // 5 minutes max polling

  const statusMessages = {
    loading:    'Connecting...',
    Pending:    'Upload received, preparing to process...',
    Processing: 'Transcoding video, almost ready...',
    Ready:      'Ready to play',
    Failed:     'Transcoding failed',
  };

  const statusIcons = {
    loading:    '◌',
    Pending:    '◌',
    Processing: '◎',
    Ready:      '●',
    Failed:     '✕',
  };

  async function fetchStatus() {
    const slug = $page.params.slug;

    try {
      const res = await fetch(`http://localhost:3000/v/${slug}`);

      if (!res.ok) {
        error = 'Video not found.';
        clearInterval(interval);
        return;
      }

      const data = await res.json();
      status = data.status;
      pollCount++;

      if (data.status === 'Ready' && data.manifest_url && !videoLoaded) {
        clearInterval(interval);
        videoLoaded = true;
        loadVideo(data.manifest_url);
      }

      if (data.status === 'Failed') {
        clearInterval(interval);
        error = 'Transcoding failed. The video format may not be supported.';
      }

      if (pollCount >= MAX_POLLS) {
        clearInterval(interval);
        error = 'Processing is taking too long. Please try again later.';
      }
    } catch (e) {
      error = 'Could not connect to server.';
      clearInterval(interval);
    }
  }

  function loadVideo(manifestUrl) {
    if (Hls.isSupported()) {
      const hls = new Hls({
        enableWorker: true,
        lowLatencyMode: false,
      });
      hls.loadSource(manifestUrl);
      hls.attachMedia(videoEl);
      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        videoEl.play().catch(() => {}); // autoplay may be blocked
      });
    } else if (videoEl.canPlayType('application/vnd.apple.mpegurl')) {
      // Safari native HLS
      videoEl.src = manifestUrl;
    } else {
      error = 'Your browser does not support HLS streaming.';
    }
  }

  onMount(() => {
    fetchStatus();
    interval = setInterval(fetchStatus, 2000);
  });

  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<div class="page">
  <div class="container">
    <header>
      <a href="/" class="back-link">← StreamDrop</a>
    </header>

    <!-- Video player -->
    <div class="player-wrapper" class:visible={status === 'Ready'}>
      <video
        bind:this={videoEl}
        controls
        playsinline
        class="video-player"
      ></video>
    </div>

    <!-- Status overlay — shown when not ready -->
    {#if status !== 'Ready' && !error}
      <div class="status-card">
        <div class="status-icon" class:spinning={status === 'Processing' || status === 'Pending' || status === 'loading'}>
          {statusIcons[status] || '◌'}
        </div>
        <p class="status-text">{statusMessages[status] || 'Loading...'}</p>
        {#if status === 'Processing' || status === 'Pending'}
          <div class="pulse-bar">
            <div class="pulse-fill"></div>
          </div>
          <p class="status-hint">You can share the link now — it will work once processing completes.</p>
        {/if}
      </div>
    {/if}

    <!-- Error state -->
    {#if error}
      <div class="error-card">
        <div class="error-icon">✕</div>
        <h2>Something went wrong</h2>
        <p>{error}</p>
        <a href="/" class="retry-btn">Upload a new video</a>
      </div>
    {/if}

    <!-- Share section — shown when ready -->
    {#if status === 'Ready'}
      <div class="share-bar">
        <span class="share-label">Share link:</span>
        <span class="share-url">{$page.url.href}</span>
        <button class="copy-btn" on:click={() => navigator.clipboard.writeText($page.url.href)}>
          Copy
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Syne:wght@400;600;700;800&family=DM+Sans:wght@300;400;500&display=swap');

  :global(body) {
    margin: 0;
    background: #0a0a0f;
    color: #f0f0f0;
    font-family: 'DM Sans', sans-serif;
  }

  .page {
    min-height: 100vh;
    background: #0a0a0f;
    display: flex;
    justify-content: center;
    padding: 1.5rem;
  }

  .container {
    width: 100%;
    max-width: 900px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  header {
    display: flex;
    align-items: center;
  }

  .back-link {
    color: #6b7280;
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: 500;
    transition: color 0.2s;
    font-family: 'Syne', sans-serif;
  }

  .back-link:hover { color: #a78bfa; }

  /* Video player */
  .player-wrapper {
    display: none;
    background: #000;
    border-radius: 16px;
    overflow: hidden;
    border: 1px solid #1e1e2e;
    aspect-ratio: 16/9;
  }

  .player-wrapper.visible {
    display: block;
  }

  .video-player {
    width: 100%;
    height: 100%;
    display: block;
  }

  /* Status card */
  .status-card {
    background: #13131a;
    border: 1px solid #1e1e2e;
    border-radius: 16px;
    padding: 3rem 2rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    aspect-ratio: 16/9;
    justify-content: center;
  }

  .status-icon {
    font-size: 2.5rem;
    color: #7c3aed;
    line-height: 1;
  }

  .status-icon.spinning {
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.4; transform: scale(0.95); }
  }

  .status-text {
    font-size: 1.1rem;
    color: #e2e8f0;
    font-weight: 500;
    margin: 0;
  }

  .pulse-bar {
    width: 200px;
    height: 4px;
    background: #1e1e2e;
    border-radius: 99px;
    overflow: hidden;
  }

  .pulse-fill {
    height: 100%;
    background: linear-gradient(90deg, #7c3aed, #a855f7);
    border-radius: 99px;
    animation: slide 1.5s ease-in-out infinite;
  }

  @keyframes slide {
    0% { transform: translateX(-100%); width: 60%; }
    50% { transform: translateX(100%); width: 60%; }
    100% { transform: translateX(200%); width: 60%; }
  }

  .status-hint {
    font-size: 0.82rem;
    color: #4b5563;
    margin: 0;
    max-width: 300px;
  }

  /* Error card */
  .error-card {
    background: #13131a;
    border: 1px solid #7f1d1d;
    border-radius: 16px;
    padding: 3rem 2rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .error-icon {
    width: 56px;
    height: 56px;
    background: #7f1d1d;
    border-radius: 50%;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 1.4rem;
    color: #fca5a5;
  }

  .error-card h2 {
    font-family: 'Syne', sans-serif;
    margin: 0;
    font-size: 1.5rem;
  }

  .error-card p {
    color: #9ca3af;
    margin: 0;
    font-size: 0.95rem;
  }

  .retry-btn {
    background: linear-gradient(135deg, #7c3aed, #a855f7);
    color: white;
    text-decoration: none;
    padding: 0.8rem 1.5rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.95rem;
    margin-top: 0.5rem;
    transition: all 0.2s;
    display: inline-block;
  }

  .retry-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 8px 25px rgba(124, 58, 237, 0.4);
  }

  /* Share bar */
  .share-bar {
    background: #13131a;
    border: 1px solid #1e1e2e;
    border-radius: 12px;
    padding: 0.9rem 1.2rem;
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .share-label {
    font-size: 0.85rem;
    color: #6b7280;
    white-space: nowrap;
    font-weight: 500;
  }

  .share-url {
    flex: 1;
    font-size: 0.85rem;
    color: #a78bfa;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .copy-btn {
    background: #1e1e2e;
    border: 1px solid #2a2a3e;
    color: #e2e8f0;
    padding: 0.4rem 0.9rem;
    border-radius: 8px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    font-family: 'DM Sans', sans-serif;
  }

  .copy-btn:hover {
    background: #2a2a3e;
    color: #a78bfa;
  }
</style>
