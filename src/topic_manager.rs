use crate::config;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use anyhow::Result;
use log::{info, error};

/// Менеджер для работы с топиками Kafka
pub struct TopicManager {
    admin_client: AdminClient<DefaultClientContext>,
}

impl TopicManager {
    /// Создание нового менеджера топиков
    pub fn new() -> Result<Self> {
        let admin_config = config::create_admin_config();
        let admin_client: AdminClient<DefaultClientContext> = admin_config.create()?;

        Ok(Self { admin_client })
    }

    /// Создание топика если он не существует
    pub async fn create_topic_if_not_exists(&self, topic_name: &str) -> Result<()> {
        info!("📝 Создаем топик: {}", topic_name);

        let new_topic = NewTopic::new(
            topic_name,
            1, // количество партиций
            TopicReplication::Fixed(1), // фактор репликации
        );

        let admin_options = AdminOptions::new();

        match self.admin_client.create_topics(&[new_topic], &admin_options).await {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(_) => info!("✅ Топик {} создан успешно", topic_name),
                        Err(e) => {
                            if e.1.to_string().contains("already exists") {
                                info!("ℹ️  Топик {} уже существует", topic_name);
                            } else {
                                error!("❌ Ошибка создания топика {}: {}", topic_name, e.1);
                                return Err(anyhow::anyhow!("Ошибка создания топика: {}", e.1));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("❌ Ошибка при создании топика {}: {}", topic_name, e);
                return Err(anyhow::anyhow!("Ошибка при создании топика: {}", e));
            }
        }

        Ok(())
    }

    /// Проверка существования топика
    async fn topic_exists(&self, topic_name: &str) -> Result<bool> {
        // Получаем метаданные клиента
        let metadata = self.admin_client.fetch_metadata(None, std::time::Duration::from_secs(5))?;

        // Проверяем, есть ли топик в метаданных
        for topic in metadata.topics() {
            if topic.name() == topic_name {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Получение списка всех топиков
    pub async fn list_topics(&self) -> Result<Vec<String>> {
        let metadata = self.admin_client.fetch_metadata(None, std::time::Duration::from_secs(5))?;
        let topics: Vec<String> = metadata.topics().iter().map(|t| t.name().to_string()).collect();

        info!("📋 Список топиков: {:?}", topics);
        Ok(topics)
    }
}
