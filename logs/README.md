# Logs Directory

Эта папка используется для хранения логов контейнеров Docker.

## Конфигурация логирования

В docker-compose для каждого сервиса настроено логирование:

- **Log Driver**: `json-file`
- **Max Size**: 10MB на файл
- **Max Files**: 3 файла (примерно 3 дня хранения)
- **Log Level**: INFO

## Где искать логи

Логи управляются стандартной системой Docker и хранятся:

- **Windows**: `%USERPROFILE%\AppData\Local\Docker\containers\[container-id]\[container-id]-json.log`
- **Linux/Mac**: `/var/lib/docker/containers/[container-id]/[container-id]-json.log`

## Уровни логов

Для Rust-приложения используются переменные:

- `RUST_LOG=info` — уровень логов INFO
- `LOG_LEVEL=info` — общий уровень логов

## Как смотреть логи

```bash
# Все логи всех сервисов
docker-compose logs

# Логи только Rust consumer
docker-compose logs rust-consumer

# Онлайн просмотр логов
docker-compose logs -f rust-consumer

# Последние N строк
docker-compose logs --tail=100 rust-consumer
```

## Как чистить логи

```bash
# Очистить все логи Docker
docker system prune -f

# Очистить логи конкретного контейнера
docker logs --truncate=0 [container-name]
```
