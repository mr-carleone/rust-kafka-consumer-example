#!/bin/bash

echo "🚀 Запуск Kafka инфраструктуры..."
docker-compose -f docker-compose.kafka.yml up -d

echo "⏳ Ожидание готовности Kafka..."
sleep 10

echo "✅ Kafka инфраструктура запущена!"
echo "📊 Kafka UI доступен по адресу: http://localhost:8080"
echo "🔌 Kafka доступен по адресу: localhost:9092"
