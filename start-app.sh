#!/bin/bash

echo "🚀 Запуск Rust приложения..."
docker-compose -f docker-compose.app.yml up --build

echo "✅ Приложение запущено!"
echo "📝 Логи доступны в папке: ./logs/consumer.log"
