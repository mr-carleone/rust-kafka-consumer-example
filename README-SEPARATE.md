# Dashboard - Kafka Consumer (Отдельные Docker Compose файлы)

Этот проект теперь разделен на два отдельных docker-compose файла для лучшей организации:

## 📁 Структура файлов

- `docker-compose.kafka.yml` - Kafka инфраструктура (Zookeeper, Kafka, Kafka UI)
- `docker-compose.app.yml` - Rust приложение (Consumer/Producer)
- `start-kafka.sh` - Скрипт запуска Kafka
- `start-app.sh` - Скрипт запуска приложения
- `stop-kafka.sh` - Скрипт остановки Kafka
- `stop-app.sh` - Скрипт остановки приложения

## 🚀 Запуск

### 1. Запуск Kafka инфраструктуры
```bash
# Linux/Mac
./start-kafka.sh

# Windows PowerShell
docker-compose -f docker-compose.kafka.yml up -d
```

### 2. Запуск приложения
```bash
# Linux/Mac
./start-app.sh

# Windows PowerShell
docker-compose -f docker-compose.app.yml up --build
```

## 🛑 Остановка

### Остановка Kafka
```bash
# Linux/Mac
./stop-kafka.sh

# Windows PowerShell
docker-compose -f docker-compose.kafka.yml down
```

### Остановка приложения
```bash
# Linux/Mac
./stop-app.sh

# Windows PowerShell
docker-compose -f docker-compose.app.yml down
```

## 📊 Мониторинг

- **Kafka UI**: http://localhost:8080
- **Kafka Broker**: localhost:9092
- **Zookeeper**: localhost:2181

## 📝 Логи

### Docker логи
```bash
# Логи Kafka
docker-compose -f docker-compose.kafka.yml logs

# Логи приложения
docker-compose -f docker-compose.app.yml logs

# Логи конкретного сервиса
docker-compose -f docker-compose.app.yml logs rust-consumer
```

### Локальные логи
Логи приложения записываются в файл: `./logs/consumer.log`

## 🔧 Преимущества разделения

1. **Независимость**: Можно запускать/останавливать Kafka и приложение отдельно
2. **Гибкость**: Можно использовать разные версии Kafka для разных проектов
3. **Производительность**: Приложение не перезапускается при изменении Kafka конфигурации
4. **Отладка**: Легче отлаживать проблемы с конкретным компонентом

## 📋 Команды для разработки

### Пересборка приложения
```bash
docker-compose -f docker-compose.app.yml build
```

### Просмотр логов в реальном времени
```bash
docker-compose -f docker-compose.app.yml logs -f rust-consumer
```

### Очистка всех данных
```bash
docker-compose -f docker-compose.kafka.yml down -v
docker-compose -f docker-compose.app.yml down
```

## 🔄 Миграция с единого файла

Если у вас был старый `docker-compose.yml`, вы можете:

1. Остановить старые контейнеры: `docker-compose down`
2. Запустить новую инфраструктуру: `./start-kafka.sh`
3. Запустить приложение: `./start-app.sh`

Все данные Kafka сохранятся в volumes.
