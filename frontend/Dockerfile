# Stage 1: Build the SvelteKit application
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy the rest of the application source code
COPY . .

# Build the application
RUN npm run build

# Stage 2: Create the final production image
FROM node:20-alpine

WORKDIR /app

# SvelteKit's node adapter requires a few files to run
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/package-lock.json ./package-lock.json

# Install only production dependencies
RUN npm install --omit=dev

# Expose the port the app will run on
EXPOSE 3000

# The node adapter for SvelteKit creates a server at build/index.js
ENV PORT=3000
CMD ["node", "build/index.js"]