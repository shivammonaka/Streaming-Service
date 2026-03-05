<script>
  const MAX_SIZE = 1024 * 1024 * 1024; // 1GB

  let file = null;
  let uploading = false;
  let shareUrl = null;
  let error = null;
  let progress = 0;
  let dragOver = false;

  function handleFileChange(e) {
    const selected = e.target.files[0];
    validateAndSet(selected);
  }

  function handleDrop(e) {
    e.preventDefault();
    dragOver = false;
    const dropped = e.dataTransfer.files[0];
    validateAndSet(dropped);
  }

  function validateAndSet(selected) {
    error = null;
    if (!selected) return;

    if (!selected.type.startsWith('video/')) {
      error = 'Only video files are supported.';
      return;
    }

    if (selected.size > MAX_SIZE) {
      error = 'File exceeds 1GB limit.';
      return;
    }

    file = selected;
  }

  function formatSize(bytes) {
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
    return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
  }

  async function handleUpload() {
    if (!file) return;

    uploading = true;
    error = null;
    progress = 0;

    try {
      const formData = new FormData();
      formData.append('file', file);

      // use XMLHttpRequest for upload progress
      const result = await new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();

        xhr.upload.addEventListener('progress', (e) => {
          if (e.lengthComputable) {
            progress = Math.round((e.loaded / e.total) * 100);
          }
        });

        xhr.addEventListener('load', () => {
          if (xhr.status >= 200 && xhr.status < 300) {
            resolve(JSON.parse(xhr.responseText));
          } else {
            reject(new Error(xhr.responseText || 'Upload failed'));
          }
        });

        xhr.addEventListener('error', () => reject(new Error('Network error')));

        xhr.open('POST', 'http://localhost:3000/api/videos');
        xhr.send(formData);
      });

      shareUrl = result.share_url;
    } catch (e) {
      error = e.message || 'Upload failed. Is the backend running?';
    } finally {
      uploading = false;
    }
  }

  function reset() {
    file = null;
    shareUrl = null;
    error = null;
    progress = 0;
  }

  function copyLink() {
    navigator.clipboard.writeText(shareUrl);
  }
</script>

<div class="page">
  <div class="container">
    <header>
      <div class="logo">▶</div>
      <h1>StreamDrop</h1>
      <p class="subtitle">Upload once. Stream anywhere.</p>
    </header>

    {#if !shareUrl}
      <div class="card">
        <!-- Drop zone -->
        <div
          class="dropzone"
          class:active={dragOver}
          class:has-file={file}
          on:dragover|preventDefault={() => dragOver = true}
          on:dragleave={() => dragOver = false}
          on:drop={handleDrop}
          role="button"
          tabindex="0"
        >
          {#if file}
            <div class="file-info">
              <span class="file-icon">🎬</span>
              <span class="file-name">{file.name}</span>
              <span class="file-size">{formatSize(file.size)}</span>
              {#if !uploading}
                <button class="remove-btn" on:click={reset}>✕</button>
              {/if}
            </div>
          {:else}
            <div class="drop-prompt">
              <span class="drop-icon">⬆</span>
              <p>Drag & drop your video here</p>
              <p class="hint">or click to browse</p>
              <p class="limit">Max 1GB · MP4, MKV, AVI, MOV supported</p>
            </div>
            <input
              type="file"
              accept="video/*"
              on:change={handleFileChange}
              class="file-input"
            />
          {/if}
        </div>

        {#if error}
          <div class="error-banner">
            <span>⚠</span> {error}
          </div>
        {/if}

        {#if uploading}
          <div class="progress-section">
            <div class="progress-bar">
              <div class="progress-fill" style="width: {progress}%"></div>
            </div>
            <div class="progress-label">
              {#if progress < 100}
                Uploading... {progress}%
              {:else}
                Processing upload...
              {/if}
            </div>
          </div>
        {/if}

        <button
          class="upload-btn"
          on:click={handleUpload}
          disabled={!file || uploading}
        >
          {#if uploading}
            <span class="spinner"></span> Uploading...
          {:else}
            Upload Video
          {/if}
        </button>
      </div>

    {:else}
      <!-- Success state -->
      <div class="card success-card">
        <div class="success-icon">✓</div>
        <h2>Video Uploaded!</h2>
        <p class="success-sub">Your video is being processed. Share the link below.</p>

        <div class="link-box">
          <span class="link-text">{shareUrl}</span>
          <button class="copy-btn" on:click={copyLink}>Copy</button>
        </div>

        <div class="success-actions">
          <a href={shareUrl} class="watch-btn">Watch Now →</a>
          <button class="new-btn" on:click={reset}>Upload Another</button>
        </div>
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
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    background: radial-gradient(ellipse at 20% 50%, #1a0a2e 0%, #0a0a0f 60%);
  }

  .container {
    width: 100%;
    max-width: 520px;
  }

  header {
    text-align: center;
    margin-bottom: 2.5rem;
  }

  .logo {
    font-size: 2rem;
    width: 56px;
    height: 56px;
    background: linear-gradient(135deg, #7c3aed, #a855f7);
    border-radius: 16px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1rem;
  }

  h1 {
    font-family: 'Syne', sans-serif;
    font-size: 2.2rem;
    font-weight: 800;
    margin: 0 0 0.5rem;
    background: linear-gradient(135deg, #fff 0%, #a78bfa 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .subtitle {
    color: #6b7280;
    font-size: 0.95rem;
    margin: 0;
    font-weight: 300;
  }

  .card {
    background: #13131a;
    border: 1px solid #1e1e2e;
    border-radius: 20px;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  .dropzone {
    border: 2px dashed #2a2a3e;
    border-radius: 14px;
    padding: 2.5rem 1.5rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
    background: #0d0d14;
  }

  .dropzone:hover, .dropzone.active {
    border-color: #7c3aed;
    background: #110d1f;
  }

  .dropzone.has-file {
    border-style: solid;
    border-color: #4c1d95;
    background: #110d1f;
  }

  .file-input {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
    width: 100%;
    height: 100%;
  }

  .drop-icon {
    font-size: 2rem;
    display: block;
    margin-bottom: 0.8rem;
    opacity: 0.6;
  }

  .drop-prompt p {
    margin: 0.3rem 0;
    color: #9ca3af;
  }

  .drop-prompt .hint {
    color: #6b7280;
    font-size: 0.85rem;
  }

  .drop-prompt .limit {
    color: #4b5563;
    font-size: 0.78rem;
    margin-top: 0.8rem;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .file-icon { font-size: 1.5rem; }

  .file-name {
    font-weight: 500;
    color: #e2e8f0;
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-size {
    color: #7c3aed;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .remove-btn {
    background: none;
    border: none;
    color: #6b7280;
    cursor: pointer;
    font-size: 1rem;
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
    transition: color 0.2s;
  }

  .remove-btn:hover { color: #ef4444; }

  .error-banner {
    background: #1f0a0a;
    border: 1px solid #7f1d1d;
    color: #fca5a5;
    padding: 0.8rem 1rem;
    border-radius: 10px;
    font-size: 0.9rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .progress-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .progress-bar {
    height: 6px;
    background: #1e1e2e;
    border-radius: 99px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #7c3aed, #a855f7);
    border-radius: 99px;
    transition: width 0.3s ease;
  }

  .progress-label {
    font-size: 0.85rem;
    color: #7c3aed;
    text-align: right;
    font-weight: 500;
  }

  .upload-btn {
    background: linear-gradient(135deg, #7c3aed, #a855f7);
    color: white;
    border: none;
    padding: 0.9rem 1.5rem;
    border-radius: 12px;
    font-size: 1rem;
    font-weight: 600;
    font-family: 'DM Sans', sans-serif;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .upload-btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 8px 25px rgba(124, 58, 237, 0.4);
  }

  .upload-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    transform: none;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Success state */
  .success-card {
    text-align: center;
    padding: 2.5rem 2rem;
  }

  .success-icon {
    width: 64px;
    height: 64px;
    background: linear-gradient(135deg, #059669, #10b981);
    border-radius: 50%;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 1.8rem;
    margin-bottom: 1.2rem;
  }

  .success-card h2 {
    font-family: 'Syne', sans-serif;
    font-size: 1.8rem;
    font-weight: 700;
    margin: 0 0 0.5rem;
    color: #f0f0f0;
  }

  .success-sub {
    color: #6b7280;
    margin: 0 0 1.5rem;
    font-size: 0.95rem;
  }

  .link-box {
    background: #0d0d14;
    border: 1px solid #1e1e2e;
    border-radius: 12px;
    padding: 0.8rem 1rem;
    display: flex;
    align-items: center;
    gap: 0.8rem;
    margin-bottom: 1.5rem;
  }

  .link-text {
    flex: 1;
    font-size: 0.85rem;
    color: #a78bfa;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: left;
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

  .success-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .watch-btn {
    background: linear-gradient(135deg, #7c3aed, #a855f7);
    color: white;
    text-decoration: none;
    padding: 0.8rem 1.5rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.95rem;
    transition: all 0.2s;
  }

  .watch-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 8px 25px rgba(124, 58, 237, 0.4);
  }

  .new-btn {
    background: transparent;
    border: 1px solid #2a2a3e;
    color: #9ca3af;
    padding: 0.8rem 1.5rem;
    border-radius: 12px;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s;
    font-family: 'DM Sans', sans-serif;
  }

  .new-btn:hover {
    border-color: #4b5563;
    color: #e2e8f0;
  }
</style>
