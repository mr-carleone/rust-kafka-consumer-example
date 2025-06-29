#!/bin/bash

echo "🛑 Остановка Kafka инфраструктуры..."
docker-compose -f docker-compose.kafka.yml down

echo "✅ Kafka инфраструктура остановлена!"
