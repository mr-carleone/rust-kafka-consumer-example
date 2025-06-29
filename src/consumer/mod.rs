use crate::config;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use std::time::Duration;
use tokio::time::sleep;
use anyhow::Result;
use log::{info, error, warn};

/// Структура для хранения информации о сообщении
#[derive(Debug)]
pub struct MessageInfo {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
    pub key: Option<String>,
    pub payload: String,
    pub timestamp: i64,
}

/// Основной consumer для чтения сообщений из Kafka
pub struct KafkaConsumer {
    consumer: StreamConsumer,
    topic: String,
}

impl KafkaConsumer {
    /// Создание нового consumer'а
    pub fn new() -> Result<Self> {
        let consumer: StreamConsumer = config::create_consumer_config().create()?;
        let topic = config::get_topic_name();

        Ok(Self { consumer, topic })
    }

    /// Подписка на topic
    pub fn subscribe(&self) -> Result<()> {
        self.consumer.subscribe(&[&self.topic])?;
        info!("Подписались на topic: {}", self.topic);
        Ok(())
    }

    /// Запуск цикла чтения сообщений
    pub async fn start_consuming(&self) -> Result<()> {
        info!("Начинаем чтение сообщений из topic: {}", self.topic);
        info!("Ожидаем сообщения...");

        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 5;

        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    retry_count = 0; // Сброс счетчика при успешном получении
                    let message_info = self.process_message(&msg)?;
                    self.display_message(&message_info);
                }
                Err(e) => {
                    retry_count += 1;
                    error!("❌ Ошибка получения сообщения (попытка {}/{}): {}", retry_count, MAX_RETRIES, e);

                    if retry_count >= MAX_RETRIES {
                        error!("🚨 Превышено максимальное количество попыток. Завершение работы.");
                        return Err(anyhow::anyhow!("Превышено максимальное количество попыток подключения"));
                    }

                    let delay = Duration::from_secs(2 * retry_count as u64);
                    warn!("⏳ Ожидание {} секунд перед повторной попыткой...", delay.as_secs());
                    sleep(delay).await;
                }
            }
        }
    }

    /// Обработка полученного сообщения
    fn process_message(&self, msg: &rdkafka::message::BorrowedMessage<'_>) -> Result<MessageInfo> {
        let payload = match msg.payload_view::<str>() {
            Some(Ok(s)) => s.to_string(),
            Some(Err(e)) => {
                return Err(anyhow::anyhow!("Ошибка декодирования payload: {}", e));
            }
            None => {
                return Err(anyhow::anyhow!("Пустой payload"));
            }
        };

        let key = msg.key_view::<str>()
            .unwrap_or(Ok(""))
            .unwrap_or("")
            .to_string();

        let key = if key.is_empty() { None } else { Some(key) };

        Ok(MessageInfo {
            topic: msg.topic().to_string(),
            partition: msg.partition(),
            offset: msg.offset(),
            key,
            payload,
            timestamp: msg.timestamp().to_millis().unwrap_or(0),
        })
    }

    /// Отображение информации о сообщении
    fn display_message(&self, msg_info: &MessageInfo) {
        info!("📨 Получено новое сообщение:");
        info!("   Topic: {}", msg_info.topic);
        info!("   Partition: {}", msg_info.partition);
        info!("   Offset: {}", msg_info.offset);
        info!("   Key: {:?}", msg_info.key);
        info!("   Payload: {}", msg_info.payload);
        info!("   Timestamp: {}", msg_info.timestamp);
        info!("   ---");
    }
}
