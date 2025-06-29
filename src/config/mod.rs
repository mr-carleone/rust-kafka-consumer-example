use rdkafka::config::ClientConfig;
use std::env;
use log::info;

/// Инициализация конфигурации в зависимости от окружения
pub fn initialize_config() {
    let env = get_environment();
    info!("🌍 Окружение: {}", env);

    // Список файлов для загрузки в порядке приоритета
    let env_files = vec![
        format!(".env.{}", env),
        ".env".to_string(),
    ];

    // Загружаем первый найденный файл
    for env_file in env_files {
        if std::path::Path::new(&env_file).exists() {
            if let Ok(_) = dotenv::from_filename(&env_file) {
                info!("📁 Загружен файл конфигурации: {}", env_file);
                break;
            }
        }
    }

    // Выводим текущие настройки
    info!("⚙️  Настройки:");
    info!("   Kafka Brokers: {}", get_kafka_brokers());
    info!("   Kafka Topic: {}", get_topic_name());
    info!("   Consumer Group: {}", get_consumer_group_id());
}

/// Определение текущего окружения
fn get_environment() -> String {
    env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
}

/// Kafka конфигурация для consumer'а
pub fn create_consumer_config() -> ClientConfig {
    let mut config = ClientConfig::new();

    // Основные настройки consumer'а
    config.set("group.id", get_consumer_group_id());
    config.set("bootstrap.servers", get_kafka_brokers());
    config.set("enable.partition.eof", "false");
    config.set("session.timeout.ms", get_session_timeout_ms());
    config.set("enable.auto.commit", "true");
    config.set("auto.offset.reset", get_auto_offset_reset());

    config
}

/// Kafka конфигурация для producer'а
pub fn create_producer_config() -> ClientConfig {
    let mut config = ClientConfig::new();

    // Основные настройки producer'а
    config.set("bootstrap.servers", get_kafka_brokers());
    config.set("message.timeout.ms", get_message_timeout_ms());
    config.set("retry.backoff.ms", get_retry_backoff_ms());
    config.set("message.send.max.retries", get_max_retries());

    config
}

/// Kafka конфигурация для admin клиента
pub fn create_admin_config() -> ClientConfig {
    let mut config = ClientConfig::new();

    // Основные настройки admin клиента
    config.set("bootstrap.servers", get_kafka_brokers());
    config.set("request.timeout.ms", "10000");
    config.set("socket.timeout.ms", "10000");

    config
}

/// Получение адреса Kafka брокеров из переменных окружения
fn get_kafka_brokers() -> String {
    env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string())
}

/// Получение имени topic'а из переменных окружения
pub fn get_topic_name() -> String {
    env::var("KAFKA_TOPIC").unwrap_or_else(|_| "test-topic".to_string())
}

/// Получение ID группы consumer'ов
fn get_consumer_group_id() -> String {
    env::var("CONSUMER_GROUP_ID").unwrap_or_else(|_| "rust-kafka-consumer-group".to_string())
}

/// Получение timeout сессии
fn get_session_timeout_ms() -> String {
    env::var("CONSUMER_SESSION_TIMEOUT_MS").unwrap_or_else(|_| "6000".to_string())
}

/// Получение стратегии сброса offset
fn get_auto_offset_reset() -> String {
    env::var("CONSUMER_AUTO_OFFSET_RESET").unwrap_or_else(|_| "earliest".to_string())
}

/// Получение timeout сообщения для producer
fn get_message_timeout_ms() -> String {
    env::var("PRODUCER_MESSAGE_TIMEOUT_MS").unwrap_or_else(|_| "5000".to_string())
}

/// Получение задержки повтора
fn get_retry_backoff_ms() -> String {
    env::var("PRODUCER_RETRY_BACKOFF_MS").unwrap_or_else(|_| "100".to_string())
}

/// Получение максимального количества повторов
fn get_max_retries() -> String {
    env::var("PRODUCER_MAX_RETRIES").unwrap_or_else(|_| "3".to_string())
}
