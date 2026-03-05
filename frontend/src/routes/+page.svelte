<script>
  let file = null;
  let uploading = false;
  let shareUrl = null;
  let error = null;

  function handleFileChange(e) {
    file = e.target.files[0];
  }

  async function handleUpload() {
    if (!file) return;

    uploading = true;
    error = null;

    try {
      const formData = new FormData();
      formData.append('file', file);

      const res = await fetch('http://localhost:3000/api/videos', {
        method: 'POST',
        body: formData,
      });

      const data = await res.json();
      shareUrl = data.share_url;
    } catch (e) {
      error = 'Upload failed. Is the backend running?';
    } finally {
      uploading = false;
    }
  }
</script>

<main>
  <h1>Upload a Video</h1>

  {#if !shareUrl}
    <input type="file" accept="video/*" on:change={handleFileChange} />

    <button on:click={handleUpload} disabled={!file || uploading}>
      {uploading ? 'Uploading...' : 'Upload'}
    </button>

    {#if error}
      <p style="color: red">{error}</p>
    {/if}
  {:else}
    <p>Your video is ready!</p>
    <a href={shareUrl}>Click here to watch</a>
  {/if}
</main>