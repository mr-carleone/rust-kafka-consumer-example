# Rust Kafka Consumer

Проект демонстрирует создание Kafka consumer'а на языке Rust с использованием Docker Compose для развертывания.

## 🏗️ Структура проекта

```
dashboard/
├── src/
│   ├── main.rs          # Главная точка входа
│   ├── config/
│   │   └── mod.rs       # Конфигурация Kafka
│   ├── consumer/
│   │   └── mod.rs       # Consumer логика
│   └── producer/
│       └── mod.rs       # Producer логика
├── Dockerfile           # Docker образ для приложения
├── docker-compose.yml   # Оркестрация всех сервисов
├── env.example          # Пример конфигурации
└── README.md           # Документация
```

## 🚀 Быстрый старт

### 1. Настройка конфигурации

Создайте файлы конфигурации на основе примера:

```bash
# Основной файл конфигурации
cp env.example .env

# Для локальной разработки
cp env.example .env.local

# Для Docker окружения
cp env.example .env.docker

# Для продакшена
cp env.example .env.production
```

### 2. Запуск с Docker Compose

```bash
# Сборка и запуск всех сервисов
docker-compose up --build

# Запуск в фоновом режиме
docker-compose up -d --build
```

### 3. Локальная разработка

#### Windows (Git Bash):
```bash
# Установка зависимостей
cargo build

# Запуск в локальном окружении
RUST_ENV=local cargo run

# Запуск в продакшен окружении
RUST_ENV=production cargo run
```

#### Windows (PowerShell):
```powershell
# Установка зависимостей
cargo build

# Запуск в локальном окружении
$env:RUST_ENV="local"; cargo run

# Запуск в продакшен окружении
$env:RUST_ENV="production"; cargo run
```

#### Windows (Command Prompt):
```cmd
# Установка зависимостей
cargo build

# Запуск в локальном окружении cmd
set RUST_ENV=local && cargo run

# Запуск в локальном окружении powershell
$env:RUST_ENV="local"; cargo run

# Запуск в продакшен окружении cmd
set RUST_ENV=production && cargo run

# Запуск в продакшен окружении powershell
$env:RUST_ENV="production"; cargo run
```

#### Альтернативный способ (рекомендуемый):
Создайте файл `.env.local` с настройками и запускайте просто:
```bash
cargo run
```

## 📋 Требования

- Docker и Docker Compose
- Rust 1.75+ (для локальной разработки)
- Git Bash (рекомендуется для Windows)
- CMake (для сборки rdkafka на Windows)
- Kafka broker (автоматически запускается в Docker)

### Установка зависимостей на Windows

#### 1. Rust
```bash
# Скачайте и установите с https://rustup.rs/
rustup update
```

#### 2. CMake (для локальной сборки)
```bash
# Скачайте с https://cmake.org/download/
# Выберите "Windows x64 Installer"
# При установке выберите "Add CMake to the system PATH"
```

#### 3. Git Bash
```bash
# Скачайте с https://git-scm.com/download/win
# Установите с опцией "Git Bash"
```

#### 4. Docker Desktop
```bash
# Скачайте с https://www.docker.com/products/docker-desktop/
# Установите и запустите Docker Desktop
```

## 🔧 Конфигурация

### Переменные окружения

| Переменная | Описание | По умолчанию |
|------------|----------|--------------|
| `RUST_ENV` | Окружение (development/local/docker/production) | `development` |
| `KAFKA_BROKERS` | Адрес Kafka брокеров | `localhost:9092` |
| `KAFKA_TOPIC` | Имя topic'а для чтения | `test-topic` |
| `CONSUMER_GROUP_ID` | ID группы consumer'ов | `rust-kafka-consumer-group` |
| `CONSUMER_SESSION_TIMEOUT_MS` | Timeout сессии | `6000` |
| `CONSUMER_AUTO_OFFSET_RESET` | Стратегия сброса offset | `earliest` |
| `PRODUCER_MESSAGE_TIMEOUT_MS` | Timeout сообщения | `5000` |
| `PRODUCER_RETRY_BACKOFF_MS` | Задержка повтора | `100` |
| `PRODUCER_MAX_RETRIES` | Максимум повторов | `3` |

### Файлы конфигурации

Приложение автоматически загружает конфигурацию в следующем порядке:

1. `.env.{RUST_ENV}` - Специфичный для окружения файл
2. `.env` - Основной файл конфигурации

**Примеры файлов:**

#### `.env.local` (локальная разработка):
```bash
RUST_ENV=local
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC=test-topic
RUST_LOG=debug
```

#### `.env.docker` (Docker окружение):
```bash
RUST_ENV=docker
KAFKA_BROKERS=kafka:29092
KAFKA_TOPIC=test-topic
RUST_LOG=info
```

#### `.env.production` (продакшен):
```bash
RUST_ENV=production
KAFKA_BROKERS=kafka-prod:9092
KAFKA_TOPIC=production-topic
RUST_LOG=warn
```

## 🐳 Docker сервисы

### Основные сервисы

1. **zookeeper** - Координация Kafka кластера
2. **kafka** - Kafka брокер
3. **rust-consumer** - Наше Rust приложение
4. **test-producer** - Тестовый producer для демонстрации

### Дополнительные сервисы

- **kafka-ui** - Веб-интерфейс для управления Kafka (http://localhost:8080)

## 📊 Мониторинг

### Kafka UI
Откройте http://localhost:8080 для доступа к веб-интерфейсу Kafka.

### Логи приложения
```bash
# Просмотр логов consumer'а
docker-compose logs -f rust-consumer

# Просмотр логов Kafka
docker-compose logs -f kafka
```

## 🔍 Использование

### Отправка тестовых сообщений

```bash
# Через Docker Compose (автоматически)
docker-compose up test-producer

# Вручную через Kafka CLI
docker exec -it kafka kafka-console-producer \
  --bootstrap-server localhost:9092 \
  --topic test-topic
```

### Просмотр сообщений

```bash
# Через Kafka CLI
docker exec -it kafka kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic test-topic \
  --from-beginning
```

## 🛠️ Разработка

### Добавление новых функций

1. **Новые модули**: Создайте папку в `src/` и добавьте `mod.rs`
2. **Конфигурация**: Обновите `src/config/mod.rs`
3. **Логика**: Добавьте в соответствующий модуль

### Тестирование

```bash
# Запуск тестов
cargo test

# Проверка кода
cargo clippy
cargo fmt
```

## 📝 Логирование

Приложение выводит структурированные логи:

- 🌍 Определение окружения
- 📁 Загрузка конфигурации
- ⚙️ Текущие настройки
- 📤 Отправка сообщений
- 📨 Получение сообщений
- ❌ Ошибки
- ⚠️ Предупреждения

## 🔧 Устранение неполадок

### Проблемы с подключением

1. Проверьте, что Kafka запущен:
```bash
docker-compose ps
```

2. Проверьте логи:
```bash
docker-compose logs kafka
```

### Проблемы с конфигурацией

1. Проверьте переменные окружения:
```bash
# В Docker
docker-compose exec rust-consumer env | grep KAFKA

# Локально (Git Bash)
env | grep KAFKA

# PowerShell
Get-ChildItem Env: | Where-Object {$_.Name -like "*KAFKA*"}
```

2. Проверьте файлы конфигурации:
```bash
ls -la .env*
```

### Проблемы с компиляцией

1. Обновите Rust:
```bash
rustup update
```

2. Очистите кэш:
```bash
cargo clean
```

### Проблемы на Windows

1. **Используйте Git Bash** для лучшей совместимости
2. **Проверьте кодировку файлов** - используйте UTF-8
3. **Убедитесь, что Docker Desktop запущен** перед использованием docker-compose

## 📚 Дополнительные ресурсы

- [rdkafka документация](https://docs.rs/rdkafka/)
- [Kafka документация](https://kafka.apache.org/documentation/)
- [Docker Compose документация](https://docs.docker.com/compose/)
