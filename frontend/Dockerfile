# Build stage
FROM node:20-alpine AS build
WORKDIR /app

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
