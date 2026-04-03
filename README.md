# Polymove

Student mobility platform built as a microservices lab project. Polymove helps students discover internship opportunities enriched with local context: city signals, news, recommendations, notifications, and messaging-driven integrations.

[![Frontend](https://img.shields.io/badge/frontend-React%20%2B%20Vite-111827?logo=react)](./frontend)
[![Polytech](https://img.shields.io/badge/service-polytech-2563eb)](./services/polytech)
[![Erasmumu](https://img.shields.io/badge/service-erasmumu-16a34a)](./services/erasmumu)
[![MI8](https://img.shields.io/badge/service-mi8-f59e0b)](./services/mi8)
[![La%20Poste](https://img.shields.io/badge/service-laposte-9333ea)](./services/laposte)
[![Colporteur](https://img.shields.io/badge/tool-colporteur-ef4444)](./services/colporteur)
[![Seeder](https://img.shields.io/badge/tool-seeder-14b8a6)](./services/seeder)
[![Docker Compose](https://img.shields.io/badge/runtime-docker%20compose-0ea5e9?logo=docker)](./docker-compose.yml)
[![Proto](https://img.shields.io/badge/shared-proto-64748b)](./proto)

## Goal

Polymove exposes a single entrypoint for students:
- explore internship offers
- get recommended offers based on their domain
- receive offer notifications
- view offers enriched with MI8 city signals and recent news

The project also demonstrates event-driven messaging with RabbitMQ across the services.

## Services

### Application services

- [polytech](./services/polytech): API gateway and orchestrator. Stores students and notifications in PostgreSQL, exposes the main HTTP API, aggregates offers from Erasmumu and city signals from MI8, and publishes/consumes messaging events.
- [erasmumu](./services/erasmumu): internship offer service. Stores offers in MongoDB and exposes CRUD plus filtered listing.
- [mi8](./services/mi8): city intelligence service. Stores news in Redis, exposes gRPC methods for latest news and city scores, and consumes `news.created` events.
- [laposte](./services/laposte): subscriber and alert-preferences service. Stores subscriber preferences in MongoDB and reacts to student and offer events.
- [frontend](./frontend): React + TypeScript + Vite frontend, served via Nginx in Docker.
- [colporteur](./services/colporteur): demo publisher that injects random city news into RabbitMQ.
- [seeder](./services/seeder): optional Rust seed service that creates a larger offer dataset through the public API, using the versioned fixture file [`services/seeder/data/offers.json`](./services/seeder/data/offers.json).

### Infrastructure

- PostgreSQL: students and notifications
- MongoDB: offers and La Poste subscribers
- Redis: MI8 news timelines and scores
- RabbitMQ: event bus

## Event flows

Polymove currently demonstrates these main flows:

- `student.registered`
  Polytech publishes when a student is created, La Poste auto-creates a subscriber profile.
- `offer.created`
  Erasmumu publishes when a new offer is created, Polytech creates notifications and La Poste can react for alerts.
- `news.created`
  Colporteur publishes city news, MI8 consumes and updates latest news and city scores.

## Stack

- Rust microservices
- React + TypeScript + Vite frontend
- shadcn/ui + Tailwind CSS
- PostgreSQL, MongoDB, Redis
- RabbitMQ
- Docker Compose

## Project structure

```text
.
├── frontend/              # current React frontend
├── proto/                 # shared protobuf definitions
├── services/
│   ├── colporteur/
│   ├── erasmumu/
│   ├── laposte/
│   ├── mi8/
│   ├── polytech/
│   └── seeder/
└── docker-compose.yml
```

## Quick start

### 1. Configure environment

Create a local `.env` from the example:

```bash
cp .env.example .env
```

### 2. Start everything

```bash
docker compose up --build
```

Optional seeded startup:

```bash
docker compose --profile seed up --build
```

This keeps the normal startup unchanged. The `seed` profile only adds the one-shot `seeder` service that creates offers.

### 3. Useful URLs

- Frontend: `http://localhost:5173`
- Polytech API: `http://localhost:3000`
- Erasmumu API: `http://localhost:3001`
- La Poste API: `http://localhost:3002`
- RabbitMQ management: `http://localhost:15672`

### Optional seed profiles

- `seed`: creates a larger set of offers from [`services/seeder/data/offers.json`](./services/seeder/data/offers.json)
- `tools`: enables one-shot tools such as `colporteur`

Example with demo data and news:

```bash
docker compose --profile seed --profile tools up --build
docker compose --profile tools run --rm colporteur
```

Important:
- `seed` only creates offers
- `colporteur` is what feeds `mi8` with city news
- without `colporteur`, offer cards are expected to stay flat in the UI:
  `match score = 0`, no city metrics, and no recent signal

## How to test

### UI flow

1. Either create a student and offers manually with the API flow below, or start the stack with `--profile seed`
   The `seed` profile only creates offers for the Explorer. The Dashboard still needs a student created through the API.
2. Open `http://localhost:5173`
3. Go to `Explorer`
4. Search with at least one filter such as:
   - `city = Paris`
   - `domain = AI`
   If you only started with `--profile seed`, the cards will load but remain unenriched until you run `colporteur`
5. Go to `Dashboard`
6. Paste a valid `Student ID`
7. Verify:
   - `Recommendations`
   - `Applications`
   - `Notifications`

### API flow

Create a student for the Dashboard:

```bash
curl -s -X POST http://localhost:3000/student \
  -H 'Content-Type: application/json' \
  -d '{
    "firstname": "John",
    "name": "Doe",
    "domain": "AI"
  }'
```

Check subscriber creation:

```bash
curl -s http://localhost:3002/subscribers/<student-id>
```

Create an offer manually if you are not using `--profile seed`:

```bash
curl -s -X POST http://localhost:3001/offer \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "AI Mobility Analyst",
    "link": "https://example.com/offer/ai-mobility-analyst",
    "city": "Paris",
    "domain": "AI",
    "salary": 1450,
    "start_date": "2026-04-15",
    "end_date": "2026-09-30"
  }'
```

Create more offers if you want to test sorting and pagination:

```bash
curl -s -X POST http://localhost:3001/offer \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "Computer Vision Intern",
    "link": "https://example.com/offer/computer-vision-intern",
    "city": "Lyon",
    "domain": "AI",
    "salary": 1550,
    "start_date": "2026-05-01",
    "end_date": "2026-10-31"
  }'

curl -s -X POST http://localhost:3001/offer \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "Urban Data Intern",
    "link": "https://example.com/offer/urban-data-intern",
    "city": "Nice",
    "domain": "AI",
    "salary": 1350,
    "start_date": "2026-06-01",
    "end_date": "2026-11-30"
  }'
```

Check notifications:

```bash
curl -s http://localhost:3000/students/<student-id>/notifications
```

Publish MI8 demo news:

```bash
docker compose --profile tools run --rm colporteur
```

This step is what makes the UI interesting:
- Explorer cards get non-zero match scores
- city metrics appear
- recent signals appear
- dashboard recommendations become differentiated by city context

Check enriched offers:

```bash
curl -s "http://localhost:3000/offers?city=Paris&limit=10"
curl -s "http://localhost:3000/students/<student-id>/recommended-offers?limit=5&sort_by=safety"
```

## Notes

- The current backend requires at least one filter for `GET /offers`, so the Explorer starts from a filtered search flow.
- The frontend only talks to the gateway and La Poste through same-origin proxy routes in Docker.
- The `seed` profile is optional and keeps the default startup unchanged.
- If you want a clean demo, reset volumes first:

```bash
docker compose down -v
docker compose up --build
```

## Development

Frontend only:

```bash
cd frontend
npm i
npm run dev
```

Full stack rebuild:

```bash
docker compose up --build
```
