use std::{cell::RefCell, io, rc::Rc};

use ratatui::{style::Color, widgets::ListState, Frame, Terminal};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

use wasm_bindgen_futures::spawn_local;
use web_sys::console;

use serde::{Deserialize, Serialize};

mod demo_screen;
mod main_screen;

mod about_me_page;
mod home_page;
mod projects_page;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let api_state = Rc::new(RefCell::new(
        None::<(Vec<GitHubRepo>, bool, Option<String>)>,
    ));

    let state = Rc::new(RefCell::new(App::default()));
    {
        let mut app = state.borrow_mut();
        let api_state_clone = api_state.clone();
        app.fetch_pinned_repos(api_state_clone);
    }

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.borrow_mut().handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    let api_state_for_render = api_state.clone();
    terminal.draw_web(move |frame| {
        let mut app = render_state.borrow_mut();

        if let Some((repos, loading, error)) = api_state_for_render.borrow_mut().take() {
            app.github_repos = repos;
            app.repos_loading = loading;
            app.repos_error = error;
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
pub struct GitHubRepo {
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
    Start,
    #[default]
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
    counter: u8,

    sidebar_state: ListState,
    current_screen: CurrentScreen,
    current_page: CurrentPage,

    pages: Vec<Page>,

    github_repos: Vec<GitHubRepo>,
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
        ];

        Self {
            counter: u8::default(),

            sidebar_state: sidebar_state,
            current_screen: CurrentScreen::default(),
            current_page: CurrentPage::default(),

            pages: pages,

            github_repos: Vec::new(),
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

    fn fetch_pinned_repos(
        &mut self,
        api_state: Rc<RefCell<Option<(Vec<GitHubRepo>, bool, Option<String>)>>>,
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

        // DEMO
        } else if self.current_screen == CurrentScreen::Demo {
            demo_screen::render(self, frame, colors);
        }
    }
}

async fn get_pinned_repos_async() -> Result<Vec<GitHubRepo>, reqwest::Error> {
    const URL: &str = "https://pinned.berrysauce.dev/get/berk-efe";

    let client = reqwest::Client::new();
    let response = client
        .get(URL)
        .header("Accept", "application/json")
        .send()
        .await?;

    let repos: Vec<GitHubRepo> = response.json().await?;
    Ok(repos)
}

