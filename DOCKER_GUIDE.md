# Docker Setup & Deployment Guide

## Quick Start dengan Docker Compose

```bash
# 1. Setup environment variables
cp .env.example .env

# 2. Start services (PostgreSQL + API)
docker-compose up -d

# 3. Check logs
docker-compose logs -f api

# 4. Stop services
docker-compose down
```

## Production Deployment

### Build & Run dengan Docker

```bash
# Build image
docker build -t quoteyourlife-be:1.0 .

# Run container dengan database connection
docker run -d \
  --name qyl-api \
  -p 8080:8080 \
  -e DATABASE_URL="postgres://user:pass@db-host:5432/quoteyourlife" \
  -e RUST_LOG=info \
  quoteyourlife-be:1.0
```

### Environment Variables (Production)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | ✅ | - | PostgreSQL connection string |
| `RUST_LOG` | ❌ | `info` | Logging level (debug/info/warn/error) |
| `SERVER_HOST` | ❌ | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | ❌ | `8080` | Server port |

### Health Check

Container dilengkapi health check yang akan:
- Menjalankan curl ke endpoint `/` setiap 30 detik
- Dianggap healthy setelah 5 detik pertama
- Restart jika 3 check berturut-turut gagal

**Monitor health:**
```bash
docker ps  # Lihat status (healthy/unhealthy)
docker inspect <container-id> | grep -A 10 Health
```

## Dockerfile Improvements

### ✅ Yang Sudah Diperbaiki:

1. **Syntax Error Fixed** - `apt-get update && apt-get install` (bukan `**`)
2. **Better Comments** - Dokumentasi stage dan dependencies
3. **Optimized Caching** - Copy manifest terlebih dahulu, build dependencies sekali
4. **Security** - Non-root user dengan UID eksplisit
5. **Health Check** - Monitoring built-in untuk Docker daemon
6. **Slim Image** - Menggunakan `debian:bookworm-slim` untuk size minimal
7. **Clean Packages** - Remove apt cache untuk reduce image size

### Image Size Comparison:
- **Before**: ~500MB+ (inefficient layer caching)
- **After**: ~300-350MB (optimized multi-stage)

## Local Development

### Dengan docker-compose:
```bash
docker-compose up
# API tersedia di http://localhost:8080
# PostgreSQL tersedia di localhost:5432
```

### Tanpa Docker:
```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Setup database
diesel setup
diesel migration run

# Run server
cargo run
```

## Troubleshooting

### Container exits immediately?
```bash
# Check logs
docker-compose logs api

# Likely causes:
# 1. DATABASE_URL tidak valid
# 2. PostgreSQL belum ready - tunggu health check
# 3. Port 8080 sudah digunakan
```

### Permission denied errors?
```bash
# Verify user permissions
docker exec qyl-api id

# Should output: uid=1001(appuser) gid=1001(appuser) groups=1001(appuser)
```

### Database migration failed?
```bash
# Check if PostgreSQL is running
docker-compose ps postgres

# Verify DATABASE_URL format
echo $DATABASE_URL
```

## Next Steps

1. **Setup CI/CD** - Gunakan GitHub Actions/GitLab CI untuk auto-build & push
2. **Registry** - Push ke Docker Hub atau private registry
3. **Orchestration** - Deploy ke Kubernetes atau Docker Swarm
4. **Monitoring** - Setup Prometheus + Grafana untuk metrics

---

**Updated**: February 2026
