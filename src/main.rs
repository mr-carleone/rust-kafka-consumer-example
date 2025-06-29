// Модули проекта
mod config;
mod consumer;
mod producer;

// Импорты
use consumer::KafkaConsumer;
use producer::KafkaProducer;
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error, warn};
use flexi_logger::{Logger, FileSpec, Age, Cleanup, Criterion, Naming, WriteMode};
use std::io::Write;

/// Главная функция приложения
#[tokio::main]
async fn main() -> Result<()> {
    // Отладочная информация - эти строки должны появиться в любом случае
    println!("=== НАЧАЛО ПРОГРАММЫ ===");
    println!("Проверяем переменные окружения...");

    // Проверяем, что мы вообще дошли до этой точки
    eprintln!("STDERR: Приложение запущено");
    std::io::stdout().flush().unwrap();
    std::io::stderr().flush().unwrap();

    // Инициализация логгера с кастомным форматтером
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs").basename("combined"))
        .format(|w, now, record| {
            let message = format!("[{}][{}] {}\n",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                &record.args()
            );

            // Записываем в основной файл
            write!(w, "{}", message)
        })
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    println!("Логгер с разделением по уровням инициализирован");

    info!("🚀 Запуск Rust Kafka Consumer...");
    info!("==================================");
    println!("Логи записаны");

    // Инициализируем конфигурацию в зависимости от окружения
    config::initialize_config();
    info!("==================================");
    println!("Конфигурация инициализирована");

    // Ждем немного, чтобы Kafka успел запуститься
    info!("⏳ Ожидаем запуска Kafka...");
    println!("Начинаем ожидание Kafka...");
    sleep(Duration::from_secs(10)).await;
    println!("Ожидание Kafka завершено");

    // ПРОСТАЯ ВЕРСИЯ - без создания топиков
    println!("Используем автоматическое создание топиков");

    // Создаем producer для отправки тестовых сообщений
    info!("📤 Инициализация producer'а...");
    println!("Создаем producer...");
    let producer: Option<KafkaProducer> = match KafkaProducer::new() {
        Ok(p) => {
            println!("Producer создан успешно");
            Some(p)
        },
        Err(e) => {
            println!("Ошибка создания producer: {}", e);
            error!("❌ Ошибка создания producer'а: {}", e);
            warn!("   Продолжаем без producer'а...");
            None
        }
    };

    // Отправляем тестовые сообщения (если producer создан)
    if let Some(p) = producer {
        info!("📤 Отправляем тестовые сообщения...");
        println!("Отправляем тестовые сообщения...");

        // Отправляем одно тестовое сообщение
        if let Err(e) = p.send_test_message().await {
            println!("Ошибка отправки тестового сообщения: {}", e);
            warn!("⚠️  Ошибка отправки тестового сообщения: {}", e);
            warn!("   Это нормально, если Kafka еще не готов");
        } else {
            println!("Тестовое сообщение отправлено успешно");
        }

        // Отправляем несколько сообщений с номерами
        if let Err(e) = p.send_test_messages_with_numbers(5).await {
            println!("Ошибка отправки сообщений с номерами: {}", e);
            warn!("⚠️  Ошибка отправки сообщений с номерами: {}", e);
        } else {
            println!("Сообщения с номерами отправлены успешно");
        }
    }

    // Создаем consumer для чтения сообщений
    info!("📥 Инициализация consumer'а...");
    println!("Создаем consumer...");
    let consumer = match KafkaConsumer::new() {
        Ok(c) => {
            println!("Consumer создан успешно");
            c
        },
        Err(e) => {
            println!("Ошибка создания consumer: {}", e);
            error!("❌ Ошибка создания consumer'а: {}", e);
            return Err(e);
        }
    };

    // Подписываемся на topic
    println!("Подписываемся на topic...");
    if let Err(e) = consumer.subscribe() {
        println!("Ошибка подписки: {}", e);
        error!("❌ Ошибка подписки на topic: {}", e);
        return Err(e);
    }
    println!("Подписка на topic успешна");

    // Запускаем цикл чтения сообщений
    info!("🎧 Начинаем слушать сообщения...");
    info!("   Нажмите Ctrl+C для остановки");
    info!("==================================");
    println!("Начинаем цикл чтения сообщений...");

    consumer.start_consuming().await?;

    Ok(())
}
