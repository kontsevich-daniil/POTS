// --- Импорты и зависимости ---
use std::collections::{HashSet, VecDeque};
use reqwest::Client; // Асинхронный HTTP клиент
use scraper::{Html, Selector};
use thiserror::Error;
use url::Url; // <-- Убрали Host, так как он не используется
use tokio::time::{sleep, Duration}; // Для асинхронной задержки


// --- Определение ошибок (CrawlerError) ---
#[derive(Debug, Error)]
pub enum CrawlerError {
    #[error("Ошибка запроса: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Неправильный HTTP ответ. Статус: {0}")] 
    BadResponse(u16), // Храним статус как u16, а не String

    #[error("Некорректный URL: {0}")]
    UrlParseError(#[from] url::ParseError),
}


/// Результат проверки одной страницы
#[derive(Debug)]
struct PageResult {
    url: Url,
    // Статус может быть либо Ok(u16) (успех), либо Err(CrawlerError) (сбой/404)
    status: Result<u16, CrawlerError>, 
    links_found: usize,
}


/// Посещает страницу, извлекает все ссылки в том же домене.
async fn visit_page(
    client: &Client,
    url: &Url,
    base_domain: &str,
) -> Result<(PageResult, Vec<Url>), CrawlerError> {
    println!("-> Проверяем {}", url);


    // 1. Выполнение запроса (Используем ? для автоматического проброса ReqwestError)
    let response = client.get(url.clone()).send().await?;


    let status_code = response.status().as_u16();
    
    // --- КРИТИЧЕСКАЯ ЛОГИКА: Обработка HTTP-ошибок ---
    if !response.status().is_success() {
        // Если статус не успешный (404, 500 и т.д.), мы фиксируем это как ошибку в PageResult.
        let page_result = PageResult {
            url: url.clone(),
            status: Err(CrawlerError::BadResponse(status_code)), // Возвращаем ошибку!
            links_found: 0,
        };
        // Мы возвращаем Ok((PageResult с ошибкой, пустой список ссылок))
        return Ok((page_result, Vec::new())); 
    }


    let base_url = response.url().to_owned();
    let body_text = response.text().await?; // Используем ? для проброса ошибки чтения тела


    // --- Парсинг (Scraping) ---
    let document = Html::parse_document(&body_text);
    // !!! ВАЖНО: Здесь нужно использовать точный CSS-селектор Habr, 
    // например, "a.link" или более специфический класс статьи.
    let selector = Selector::parse("a").unwrap(); // Используем универсальный для примера


    let mut link_urls = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            // Разрешаем ссылку, используя base_url.join() и обрабатываем возможные ошибки парсинга URL
            match base_url.join(href) { 
                Ok(absolute_url) => {
                    // Проверка на принадлежность к домену (Scope Check)
                    if absolute_url.host_str() == Some(base_domain) {
                        link_urls.push(absolute_url);
                    }
                },
                Err(e) => {
                    eprintln!("Проблема с разрешением ссылки '{}': {}", href, e);
                }
            }
        }
    }


    let page_result = PageResult {
        url: url.clone(),
        status: Ok(status_code), // Успешный статус
        links_found: link_urls.len(),
    };


    Ok((page_result, link_urls))
}


// --- Главная функция (Краулер) ---
#[tokio::main] // Запускает асинхронную среду Tokio
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let client = Client::new();

    // !!! НОВЫЙ ЛИМИТ: Максимальное количество ссылок, которые мы хотим найти.
    const MAX_LINKS: usize = 10; 

    // Используем https:// для корректной работы с Habr
    let start_url = Url::parse("https://habr.com/ru/articles/")?; 
    let base_domain = start_url.host_str().expect("Не удалось определить домен");


    let mut visited: HashSet<Url> = HashSet::new();
    // Используем VecDeque для реализации BFS (Breadth-First Search)
    let mut to_visit: VecDeque<Url> = VecDeque::new(); 
    let mut results = Vec::new();

    to_visit.push_back(start_url.clone());
    visited.insert(start_url.clone());

    // !!! НОВЫЙ СЧЁТЧИК: Отслеживаем, сколько уникальных ссылок мы нашли и добавили в очередь.
    let mut links_collected_count = 0;


    while let Some(url) = to_visit.pop_front() { // pop_front для BFS
        // --- ВЕЖЛИВОСТЬ КРАУЛЕРА: Пауза между запросами ---
        sleep(Duration::from_millis(500)).await; 

        match visit_page(&client, &url, base_domain).await {
            Ok((page_result, links)) => {
                results.push(page_result);
                
                // Итерируемся по найденным ссылкам
                for link in links {
                    // 1. Проверяем лимит: Если мы уже нашли достаточно ссылок, прекращаем добавление новых.
                    if links_collected_count >= MAX_LINKS {
                        println!("\n[СТОП] Достигнут лимит в {} найденных ссылок.", MAX_LINKS);
                        break; 
                    }

                    // Если ссылка новая и ещё не посещена:
                    if visited.insert(link.clone()) { 
                        to_visit.push_back(link); // Добавляем в конец очереди для BFS
                        links_collected_count += 1; // Увеличиваем счётчик, так как нашли новую уникальную ссылку
                    }
                }

                // 2. Проверка лимита после обработки всех ссылок на странице:
                if links_collected_count >= MAX_LINKS {
                     println!("\n[СТОП] Достигнут лимит в {} найденных ссылок.", MAX_LINKS);
                     break; // Прерываем внешний цикл while (основной обход)
                }

            },
            Err(e) => {
                // Это ошибка, которая произошла ДО того, как мы получили ответ.
                results.push(PageResult {
                    url,
                    status: Err(e), 
                    links_found: 0,
                });
            }
        }
    }


    // Вывод результатов
    println!("\n=====================================");
    println!("=== Результаты проверки ===");
    for result in &results { 
        match &result.status {
            Ok(code) => println!("[{}] {} (ссылок: {})", code, result.url, result.links_found),
            Err(e) => println!("[ОШИБКА] {}: {}", result.url, e),
        }
    }
    println!("\n=====================================");
    // Теперь мы можем безопасно вызвать .len()
    println!("Всего проверено страниц: {}", results.len());


    Ok(()) // Успешное завершение main
}
