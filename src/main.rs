use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    style::Color,
    text::Line,
    widgets::{ListState, Paragraph},
    Frame, Terminal,
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

use reqwest::Client;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

use serde::{Deserialize, Serialize};

mod demo_screen;
mod main_screen;
mod start_screen;

mod about_me_page;
mod home_page;
mod projects_page;
mod resources_page;

mod helpers;

use helpers::{FIRST_BOOT_TEXT_LIST, SECOND_BOOT_TEXT_LIST};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let pinned_api_state = Rc::new(RefCell::new(
        None::<(Vec<GithubRepo>, bool, Option<String>)>,
    ));

    let latest_api_state = Rc::new(RefCell::new(
        None::<(Vec<GithubRepo>, bool, Option<String>)>,
    ));

    let state = Rc::new(RefCell::new(App::default()));
    {
        let mut app = state.borrow_mut();
        app.fetch_latest_repos(latest_api_state.clone());
        app.fetch_pinned_repos(pinned_api_state.clone());
    }

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.borrow_mut().handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    let pinned_api_state_for_render = pinned_api_state.clone();
    let latest_api_state_for_render = latest_api_state.clone();

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        let mut app = render_state.borrow_mut();

        // Handle pinned repos
        if let Some((repos, loading, error)) = pinned_api_state_for_render.borrow_mut().take() {
            app.github_pinned_repos = repos;
            app.repos_loading = loading;
            app.repos_error = error.clone();
        }

        // Handle latest repos
        if let Some((repos, loading, error)) = latest_api_state_for_render.borrow_mut().take() {
            app.latest_github_repos = repos;
            // Don't overwrite loading/error state
        }

        app.render(frame);
    });

    Ok(())
}

#[macro_export]
macro_rules! margin {
    ($horizontal:expr, $vertical:expr) => {
        ratatui::layout::Margin {
            horizontal: $horizontal,
            vertical: $vertical,
        }
    };
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
}

#[derive(Default, PartialEq)]
pub enum CurrentScreen {
    #[default]
    Start,
    Main,
    Demo,
}

// U GOTTA CHANGE THESE MANUALLY
// AFTER ADDING MORE PAGES

#[derive(Clone, Default, PartialEq)]
pub enum CurrentPage {
    #[default]
    Home,
    Projects,
    AboutMe,
    Resources,
}

#[derive(Clone)]
pub struct Page {
    title: String,
    page: CurrentPage,
}

impl Page {
    pub fn new(title: &str, page: CurrentPage) -> Self {
        Self {
            title: title.to_string(),
            page: page,
        }
    }
}

struct App {
    frame_count: usize,

    boot_index: usize,
    boot_text_id: usize,

    counter: u8,

    sidebar_state: ListState,
    current_screen: CurrentScreen,
    current_page: CurrentPage,

    pages: Vec<Page>,

    github_pinned_repos: Vec<GithubRepo>,
    latest_github_repos: Vec<GithubRepo>,
    repos_loading: bool,
    repos_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut sidebar_state = ListState::default();
        sidebar_state.select(Some(0));

        let pages = vec![
            Page::new("    Home", CurrentPage::Home),
            Page::new("    Projects", CurrentPage::Projects),
            Page::new("    About Me", CurrentPage::AboutMe),
            Page::new("    Resources", CurrentPage::Resources),
        ];

        Self {
            frame_count: 0,

            boot_index: 0,
            boot_text_id: 0,

            counter: u8::default(),

            sidebar_state: sidebar_state,
            current_screen: CurrentScreen::Main,
            current_page: CurrentPage::default(),

            pages: pages,

            github_pinned_repos: Vec::new(),
            latest_github_repos: Vec::new(),
            repos_loading: false,
            repos_error: None,
        }
    }
}

impl App {
    fn handle_events(&mut self, key_event: KeyEvent) {
        if self.current_screen == CurrentScreen::Main {
            match key_event.code {
                KeyCode::Up => {
                    self.sidebar_state.select_previous();
                }
                KeyCode::Down => {
                    self.sidebar_state.select_next();
                }
                _ => {}
            }
        } else if self.current_screen == CurrentScreen::Demo {
            match key_event.code {
                KeyCode::Left => self.counter = self.counter.saturating_sub(1),
                KeyCode::Right => self.counter = self.counter.saturating_add(1),
                _ => {}
            }
        }
    }

    fn fetch_latest_repos(
        &mut self,
        api_state: Rc<RefCell<Option<(Vec<GithubRepo>, bool, Option<String>)>>>,
    ) {
        self.repos_error = None;

        console::log_1(&"Getching latest repos".into());

        spawn_local(async move {
            match get_lates_repos_async().await {
                Ok(repos) => {
                    console::log_1(&format!("Successfully fetched {} repos", repos.len()).into());
                    for repo in &repos {
                        console::log_1(
                            &format!(
                                "🔹 {} ({} stars, {} forks) - {}",
                                repo.name, repo.stars, repo.forks, repo.language
                            )
                            .into(),
                        );
                    }
                    // Update shared state - this will be picked up in the render loop
                    *api_state.borrow_mut() = Some((repos, false, None));
                }
                Err(e) => {
                    console::log_1(&format!("Error fetching repos: {}", e).into());
                    *api_state.borrow_mut() = Some((Vec::new(), false, Some(e.to_string())));
                }
            }
        });
    }

    fn fetch_pinned_repos(
        &mut self,
        api_state: Rc<RefCell<Option<(Vec<GithubRepo>, bool, Option<String>)>>>,
    ) {
        if self.repos_loading {
            return;
        }

        self.repos_loading = true;
        self.repos_error = None;

        console::log_1(&"Starting to fetch GitHub repositories...".into());

        spawn_local(async move {
            match get_pinned_repos_async().await {
                Ok(repos) => {
                    console::log_1(&format!("Successfully fetched {} repos", repos.len()).into());
                    for repo in &repos {
                        console::log_1(
                            &format!(
                                "🔹 {} ({} stars, {} forks) - {}",
                                repo.name, repo.stars, repo.forks, repo.language
                            )
                            .into(),
                        );
                    }
                    // Update shared state - this will be picked up in the render loop
                    *api_state.borrow_mut() = Some((repos, false, None));
                }
                Err(e) => {
                    console::log_1(&format!("Error fetching repos: {}", e).into());
                    *api_state.borrow_mut() = Some((Vec::new(), false, Some(e.to_string())));
                }
            }
        });
    }

    fn render(&mut self, frame: &mut Frame) {
        let colors: Vec<Color> = vec![
            Color::Rgb(0, 19, 45),
            Color::Rgb(0, 38, 87),
            Color::Rgb(0, 55, 126),
        ];

        //MAIN
        if self.current_screen == CurrentScreen::Main {
            main_screen::render(self, frame, colors);

            // START
        } else if self.current_screen == CurrentScreen::Start {
            self.render_boot_screen(frame);

            // DEMO
        } else if self.current_screen == CurrentScreen::Demo {
            demo_screen::render(self, frame, colors);
        }
    }

    fn render_boot_screen(&mut self, frame: &mut Frame) {
        self.frame_count += 1;
        let advance_every = if self.boot_text_id == 0 { 8 } else { 1 }; // frames to wait

        if self.frame_count % advance_every == 0 {
            if self.boot_index >= FIRST_BOOT_TEXT_LIST.len() && self.boot_text_id == 0 {
                self.boot_text_id += 1;
                self.boot_index = 0;
                self.frame_count = 0;
            } else if self.boot_text_id == 1 && self.boot_index >= SECOND_BOOT_TEXT_LIST.len() {
                self.current_screen = CurrentScreen::Main;
                return;
            } else {
                self.boot_index += 1;
            }
        }

        let cur_boot_text: &[&str] = if self.boot_text_id == 0 {
            &FIRST_BOOT_TEXT_LIST
        } else {
            &SECOND_BOOT_TEXT_LIST
        };

        let view_height = frame.area().height as usize;
        let y_offset = if self.boot_index >= view_height {
            (self.boot_index - view_height) as u16
        } else {
            0
        };

        let par = Paragraph::new(
            cur_boot_text
                .iter()
                .enumerate()
                .filter(|(i, _)| *i <= self.boot_index)
                .map(|(_, text)| Line::from(*text))
                .collect::<Vec<_>>(),
        )
        .scroll((y_offset, 0))
        .style(ratatui::style::Style::default().fg(Color::White));

        frame.render_widget(par, frame.area());
    }
}

async fn get_lates_repos_async() -> Result<Vec<GithubRepo>, reqwest::Error> {
    const URL: &str = "https://api.github.com/users/berk-efe/repos?sort=updated&per_page=3";

    let client = Client::new();
    let response = client
        .get(URL)
        .header("Accept", "application/json")
        .send()
        .await?;

    let mut repos: Vec<GithubRepo> = Vec::new();
    let response_json: Value = response.json().await?;

    if let Some(json_data) = response_json.as_array() {
        for item in json_data {
            let new_repo = GithubRepo {
                author: "berk-efe".to_string(),
                name: item
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                description: item
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                language: item
                    .get("language")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                language_color: None,
                stars: 0,
                forks: 0,
            };

            repos.push(new_repo);
        }
    }

    Ok(repos)
}

async fn get_pinned_repos_async() -> Result<Vec<GithubRepo>, reqwest::Error> {
    const URL: &str = "https://pinned.berrysauce.dev/get/berk-efe";

    let client = Client::new();
    let response = client
        .get(URL)
        .header("Accept", "application/json")
        .send()
        .await?;

    let repos: Vec<GithubRepo> = response.json().await?;
    Ok(repos)
}
