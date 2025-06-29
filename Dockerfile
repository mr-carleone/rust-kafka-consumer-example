# Многоэтапная сборка для оптимизации размера образа
FROM rust:1.75-slim as builder

# Установка зависимостей для сборки
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Создание рабочей директории
WORKDIR /app

# Копирование файлов зависимостей (Cargo.lock может отсутствовать)
COPY Cargo.toml ./

# Создание пустого main.rs для кэширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Сборка зависимостей (кэширование)
RUN cargo build --release

# Удаление пустого main.rs и копирование реального кода
RUN rm src/main.rs
COPY src ./src

# Сборка приложения
RUN cargo build --release

# Финальный образ
FROM debian:bookworm-slim

# Установка runtime зависимостей
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Создание пользователя для безопасности
RUN useradd -m -u 1000 rustuser

# Создание рабочей директории
WORKDIR /app

# Создание папки для логов
RUN mkdir -p /app/logs && chown -R rustuser:rustuser /app/logs

# Копирование бинарного файла из builder этапа
COPY --from=builder /app/target/release/dashboard .

# Изменение владельца файлов
RUN chown -R rustuser:rustuser /app

# Переключение на непривилегированного пользователя
USER rustuser

# Переменные окружения по умолчанию
ENV KAFKA_BROKERS=kafka:9092
ENV KAFKA_TOPIC=test-topic

# Точка входа
ENTRYPOINT ["./dashboard"]
