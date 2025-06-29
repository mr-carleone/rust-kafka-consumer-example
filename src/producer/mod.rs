use crate::config;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use anyhow::Result;
use log::{info, error, warn};

/// Producer для отправки сообщений в Kafka
pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    /// Создание нового producer'а
    pub fn new() -> Result<Self> {
        let producer: FutureProducer = config::create_producer_config().create()?;
        let topic = config::get_topic_name();

        Ok(Self { producer, topic })
    }

    /// Отправка сообщения в Kafka
    pub async fn send_message(&self, key: &str, payload: &str) -> Result<()> {
        let delivery_status = self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .payload(payload.as_bytes())
                    .key(key),
                Duration::from_secs(5),
            )
            .await;

        match delivery_status {
            Ok((partition, offset)) => {
                info!("✅ Сообщение успешно отправлено:");
                info!("   Topic: {}", self.topic);
                info!("   Partition: {}", partition);
                info!("   Offset: {}", offset);
                info!("   Key: {}", key);
                info!("   Payload: {}", payload);
            }
            Err((e, _)) => {
                return Err(anyhow::anyhow!("Ошибка отправки сообщения: {}", e));
            }
        }

        Ok(())
    }

    /// Отправка тестового сообщения
    pub async fn send_test_message(&self) -> Result<()> {
        let test_key = "test-key";
        let test_payload = "Привет из Rust Kafka Producer! Это тестовое сообщение.";

        info!("📤 Отправляем тестовое сообщение...");
        self.send_message(test_key, test_payload).await?;

        Ok(())
    }

    /// Отправка нескольких тестовых сообщений с номерами
    pub async fn send_test_messages_with_numbers(&self, count: usize) -> Result<()> {
        info!("📤 Отправляем {} тестовых сообщений с номерами...", count);

        for i in 1..=count {
            let key = format!("test-key-{}", i);
            let payload = format!("Тестовое сообщение номер {}", i);

            if let Err(e) = self.send_message(&key, &payload).await {
                error!("❌ Ошибка отправки сообщения {}: {}", i, e);
            } else {
                info!("✅ Сообщение {} отправлено успешно", i);
            }

            // Небольшая пауза между сообщениями
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        info!("✅ Отправка {} сообщений завершена", count);
        Ok(())
    }

    /// Отправка нескольких тестовых сообщений
    pub async fn send_multiple_test_messages(&self, count: usize) -> Result<()> {
        info!("📤 Отправляем {} тестовых сообщений...", count);

        for i in 1..=count {
            let key = format!("test-key-{}", i);
            let payload = format!("Тестовое сообщение номер {}", i);

            if let Err(e) = self.send_message(&key, &payload).await {
                error!("❌ Ошибка отправки сообщения {}: {}", i, e);
            }

            // Небольшая пауза между сообщениями
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        info!("✅ Отправка {} сообщений завершена", count);
        Ok(())
    }
}
