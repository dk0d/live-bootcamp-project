# Multi-stage Bun build optimized for caching & minimal runtime size
# Base Bun image (private ECR) - keep pinned for reproducibility
# see all bun versions at https://hub.docker.com/r/oven/bun/tags
FROM oven/bun:1.3.8-slim AS base 

WORKDIR /app

##############################
# 1. Full deps (incl dev) for build
##############################
FROM base AS deps

COPY package.json bun.lock ./

# BuildKit cache mount drastically speeds up repeat installs
RUN --mount=type=cache,target=/root/.cache/bun \
    bun install --frozen-lockfile

##############################
# 2. Builder (uses full deps)
##############################

FROM base AS builder

COPY --from=deps /app/node_modules ./node_modules

# Ensure production optimizations during build (Vite/SvelteKit already respects this)
ENV NODE_ENV=production

COPY . .

# Single layer for build steps improves cache reuse
RUN bun run build

##############################
# 3. Final runtime image 
##############################
FROM oven/bun:1.3.8-slim AS release

WORKDIR /app

ENV NODE_ENV=production

# Copy dependencies
COPY --from=deps /app/node_modules ./node_modules

# Copy build artifacts & required runtime files
COPY --from=builder /app/build ./build

USER bun

EXPOSE 3000/tcp

ENTRYPOINT ["bun", "run", "./build/index.js"]
