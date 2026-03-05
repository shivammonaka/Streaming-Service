<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import Hls from 'hls.js';

  let videoEl;
  let status = 'loading';
  let error = null;
  let interval;

  async function fetchStatus() {
    const slug = $page.params.slug;
    const res = await fetch(`http://localhost:3000/v/${slug}`);
    const data = await res.json();

    status = data.status;

    if (data.status === 'Ready' && data.manifest_url) {
      clearInterval(interval);
      loadVideo(data.manifest_url);
    }

    if (data.status === 'Failed') {
      clearInterval(interval);
      error = 'Transcoding failed.';
    }
  }

  function loadVideo(manifestUrl) {
    if (Hls.isSupported()) {
      const hls = new Hls();
      hls.loadSource(manifestUrl);
      hls.attachMedia(videoEl);
    } else if (videoEl.canPlayType('application/vnd.apple.mpegurl')) {
      // Safari native HLS support
      videoEl.src = manifestUrl;
    }
  }

  onMount(() => {
    fetchStatus();
    // poll every 2 seconds until ready
    interval = setInterval(fetchStatus, 2000);

    return () => clearInterval(interval);
  });
</script>

<main>
  <h1>Watch Video</h1>

  {#if error}
    <p style="color: red">{error}</p>
  {:else if status !== 'Ready'}
    <p>Status: {status}... please wait</p>
  {/if}

  <video bind:this={videoEl} controls width="800"></video>
</main>