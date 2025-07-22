use std::{cell::RefCell, rc::Rc};

use ratatui::{
    prelude::Stylize,
    style::Style,
    text::{Line, Span},
    widgets::{ListState, Paragraph, Scrollbar, ScrollbarState},
};
use ratzilla::event::{KeyCode, KeyEvent};
use serde::{Deserialize, Serialize};

use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Start,
    Main,
    Demo,
    Exiting,
}

#[derive(PartialEq)]
pub enum CurrentPage {
    Home,
    Projects,
    AboutMe,
}

pub struct Page {
    pub title: String,
    pub page_type: CurrentPage,
}

impl Page {
    pub fn new(title: String, page_type: CurrentPage) -> Self {
        Self {
            title: title,
            page_type: page_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRepo {
    pub author: String,
    pub name: String,
    pub description: String,
    pub language: String,
    #[serde(rename = "languageColor")]
    pub language_color: Option<String>,
    pub stars: u32,
    pub forks: u32,

    pub url: String,
}

impl GithubRepo {
    pub fn new(name: &str, desc: &str, lang: &str) -> Self {
        let url = format!("https://github.com/berk-efe/{}", name.to_string());

        Self {
            author: "berk-efe".to_string(),
            name: name.to_string(),
            description: desc.to_string(),
            language: lang.to_string(),
            language_color: None,
            stars: 0,
            forks: 0,

            url: url,
        }
    }
}

pub struct App {
    pub counter: u8,
    pub current_screen: CurrentScreen,
    pub sidebar_state: ListState,

    pub current_page: CurrentPage,
    pub pages: Vec<Page>,

    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,

    pub boot_text_id: usize,
    pub boot_index: usize,
    pub frame_count: usize,

    pub title: String,

    pub github_repos: Vec<GithubRepo>,
    pub repos_loading: bool,
    pub repos_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut sidebar_state = ListState::default();
        sidebar_state.select(Some(0));

        let pages = vec![
            Page::new("  Home".to_string(), CurrentPage::Home),
            Page::new("  Projects".to_string(), CurrentPage::Projects),
            Page::new("  About Me".to_string(), CurrentPage::AboutMe),
        ];

        let mut app = App {
            counter: 0,
            current_screen: CurrentScreen::Main,
            sidebar_state,

            current_page: CurrentPage::Home,
            pages: pages,

            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: usize::default(),

            boot_text_id: usize::default(),
            boot_index: usize::default(),
            frame_count: usize::default(),

            title: String::from("\n  Berk Efe Keskin v1.0"),
            github_repos: Vec::new(),
            repos_loading: false,
            repos_error: None,
        };

        app.fetch_github_repos();

        app
    }
}

impl App {
    pub fn handle_events(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => match key_event.code {
                KeyCode::Up => {
                    self.sidebar_state.select_previous();
                }
                KeyCode::Down => {
                    self.sidebar_state.select_next();
                }
                KeyCode::Left => {}
                KeyCode::Right => {}
                KeyCode::Char('d') => self.current_screen = CurrentScreen::Demo,
                _ => {}
            },
            CurrentScreen::Demo => match key_event.code {
                KeyCode::Left => self.counter = self.counter.saturating_sub(1),
                KeyCode::Right => self.counter = self.counter.saturating_add(1),
                KeyCode::Char('m') => self.current_screen = CurrentScreen::Main,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn fetch_github_repos(&mut self) {
        if self.repos_loading {
            return;
        }

        self.repos_loading = true;
        self.repos_error = None;

        let app_ref = Rc::clone(self);

        console::log_1(&"Starting to fetch pinned repositories...".into());

        spawn_local(async move {
            match get_pinned_repos().await {
                Ok(repos) => {
                    console::log_1(&format!("Successfully fetched {} repos:", repos.len()).into());
                    for repo in &repos {
                        self.github_repos.push(GithubRepo::new(
                            &repo.name,
                            &repo.description,
                            &repo.language,
                        ));

                        console::log_1(
                            &format!(
                                "🔹 {} ({} stars, {} forks) - {}",
                                repo.name, repo.stars, repo.forks, repo.language
                            )
                            .into(),
                        );
                    }
                }
                Err(e) => {
                    console::log_1(&format!("Error fetching repos: {}", e).into());
                }
            }
        });

        self.repos_loading = false;
    }
}

async fn get_pinned_repos() -> Result<Vec<GithubRepo>, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://pinned.berrysauce.dev/get/berk-efe")
        .header("Accept", "application/json")
        .send()
        .await?;

    // Deserialize directly to Vec<GithubRepo>
    let repos: Vec<GithubRepo> = response.json().await?;

    Ok(repos)
}
