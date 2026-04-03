# Build stage
FROM node:20-alpine AS build
WORKDIR /app

ARG VITE_API_BASE_URL=http://localhost:3000
ARG VITE_LAPOSTE_BASE_URL=http://localhost:3002
ENV VITE_API_BASE_URL=${VITE_API_BASE_URL}
ENV VITE_LAPOSTE_BASE_URL=${VITE_LAPOSTE_BASE_URL}

# Install dependencies first for caching layer
COPY package*.json ./
RUN npm ci && chmod -R +x node_modules/.bin

# Copy the rest of the application
COPY . .

# Build the Vite application
RUN npm run build

# Production stage
FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
