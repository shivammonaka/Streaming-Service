# Video Streaming Service — Progress Notes

## What We Are Building

A private video streaming service where:
1. User uploads a video file
2. Gets a shareable link
3. Anyone with the link can stream the video

---

## Current Status

### ✅ Done
- Rust project set up with all dependencies
- PostgreSQL database and table created
- File upload working end to end
- File saved to disk
- Database record created on upload
- Share URL generated and returned

### 🔲 Not Done Yet
- ffmpeg transcoding (HLS chunking)
- Streaming route
- Svelte frontend

---

## Project Structure

```
video-service/
├── Cargo.toml              → dependencies (like pom.xml in Java)
├── .env                    → environment variables (DB url etc)
├── storage/
│   └── uploads/            → uploaded video files saved here
│       └── test.mp4
└── src/
    ├── main.rs             → starts server, wires everything together
    ├── models/
    │   ├── mod.rs
    │   └── video.rs        → Video struct and VideoStatus enum
    ├── db/
    │   ├── mod.rs
    │   └── videos.rs       → all SQL queries (create, get, update)
    ├── storage/
    │   ├── mod.rs          → StorageBackend trait (swappable interface)
    │   └── local.rs        → local disk implementation
    ├── routes/
    │   ├── mod.rs
    │   ├── videos.rs       → POST /api/videos, GET /api/videos/:id/status
    │   └── stream.rs       → GET /v/:slug, GET /videos/:slug/*file
    └── services/
        ├── mod.rs
        └── transcode.rs    → ffmpeg transcoding (stub, not implemented yet)
```

---

## API Routes

| Method | Route | Status | Description |
|--------|-------|--------|-------------|
| POST | `/api/videos` | ✅ Working | Upload video, get share URL |
| GET | `/api/videos/:id/status` | ✅ Written | Poll transcoding status |
| GET | `/v/:slug` | ✅ Written | Get video info by share link |
| GET | `/videos/:slug/*file` | ✅ Written | Serve HLS chunks to browser |

---

## Database Schema

```sql
CREATE TABLE videos (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug          TEXT UNIQUE NOT NULL,      -- shareable link token e.g. "xK92mPqR"
    status        TEXT NOT NULL DEFAULT 'Pending',
    original_path TEXT,                      -- where raw file is saved on disk
    hls_path      TEXT,                      -- where ffmpeg output is saved
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### Video Status Flow
```
Pending → Processing → Ready
                    → Failed
```

---

## Tech Stack

| Layer | Choice | Why |
|-------|--------|-----|
| Backend | Rust + Axum | Fast, low memory, great async |
| Frontend | Svelte + hls.js | Lightweight, hls.js handles HLS natively |
| Database | PostgreSQL | Simple, reliable, UUID support |
| Storage | Local disk (swappable to S3) | Simple for now, trait makes it swappable |
| Transcoding | ffmpeg | One command produces all HLS chunks |

---

## How Upload Works (End to End)

```
1. Client sends video file via multipart POST /api/videos
2. Server reads file bytes from request
3. Server generates a random 8 char slug e.g. "xK92mPqR"
4. Server saves file to ./storage/uploads/filename.mp4
5. Server inserts row into videos table (status=Pending)
6. Server returns:
   {
     video_id: "uuid...",
     slug: "xK92mPqR",
     share_url: "http://localhost:3000/v/xK92mPqR"
   }
```

---

## How Streaming Will Work (Not Implemented Yet)

```
1. After upload, ffmpeg runs in background:
   ffmpeg -i input.mp4 -codec:copy -hls_time 10 -f hls output/index.m3u8

2. This produces:
   storage/videos/xK92mPqR/
     index.m3u8      ← manifest file (list of chunks)
     seg_000.ts      ← chunk 1 (0-10 seconds)
     seg_001.ts      ← chunk 2 (10-20 seconds)
     ...

3. DB updated: status=Ready, hls_path=./storage/videos/xK92mPqR/index.m3u8

4. User opens share link → frontend gets manifest URL
5. hls.js fetches chunks one by one and plays video
```

---

## Storage is Swappable

The `StorageBackend` trait means switching from local disk to S3 is one line change in `main.rs`:

```rust
// Current (local disk)
let storage = LocalStorage { base_path: "./storage".to_string() };

// Future (S3)
// let storage = S3Storage { bucket: "my-bucket".to_string() };
```

All route code stays the same — routes only talk to the trait, not the implementation.

---

## What's Left To Build

### Next: Transcoding
- Implement `services/transcode.rs`
- Run ffmpeg as async background process
- Update DB status when done

### After That: Frontend (Svelte)
- Upload page with progress bar
- Share link page with hls.js video player
- Status polling while video processes