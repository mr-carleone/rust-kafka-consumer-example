#!/bin/bash

echo "🛑 Остановка Rust приложения..."
docker-compose -f docker-compose.app.yml down

echo "✅ Приложение остановлено!"
