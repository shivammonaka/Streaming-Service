# StreamDrop

A self-hosted video streaming service. Upload a video, get a shareable link, stream it anywhere.

## Requirements

- [Docker](https://www.docker.com/get-started) and Docker Compose

That's it. No other dependencies needed.

## Running the project

```bash
docker-compose up --build
```

First run takes a few minutes to build the Rust backend. Subsequent runs are much faster.

Once running:

| Service  | URL                   |
|----------|-----------------------|
| Frontend | http://localhost:5173 |
| Backend  | http://localhost:3000 |

## How to use

1. Open http://localhost:5173
2. Drag and drop a video file (up to 1GB — MP4, MKV, AVI, MOV supported)
3. Click **Upload Video**
4. Once uploaded, you'll get a shareable link
5. The video is transcoded in the background — the watch page will automatically start playing once it's ready

## Stopping

```bash
docker-compose down
```

To also delete all stored videos and the database:

```bash
docker-compose down -v
```

## Project structure

```
.
├── backend/        Rust API server (Axum) — handles uploads, transcoding, streaming
├── frontend/       SvelteKit app — upload UI and video player
├── init.sql        Database schema, auto-applied on first run
└── docker-compose.yml
```