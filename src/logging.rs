//! Session logging, an Asciiverse addition: `RUST_LOG=debug blackguard
//! 2>blackguard.log` records every key as it arrives, before it is
//! handled, the changes that matter (a new deal, win or death, the help
//! opening and closing), and a snapshot of the table at most once a second
//! while something changes.

use std::time::{Duration, Instant};

use ratatui::crossterm::event::{Event, KeyEvent};

use crate::app::App;
use crate::core::{Card, GameState, Status};

/// Logs an input event at the point it is received.
pub fn input(ev: &Event) {
    match ev {
        Event::Key(KeyEvent {
            code,
            kind,
            modifiers,
            ..
        }) => {
            log::debug!("key {code:?} {kind:?} {modifiers:?}")
        }
        Event::Mouse(m) => log::debug!("mouse {:?} at {},{}", m.kind, m.column, m.row),
        Event::Resize(w, h) => log::info!("resize {w}x{h}"),
        _ => {}
    }
}

fn card(c: Card) -> String {
    format!("{}{}", c.rank_label(), c.suit.glyph())
}

fn cards(cs: &[Card]) -> String {
    cs.iter().map(|&c| card(c)).collect::<Vec<_>>().join(" ")
}

fn table(g: &GameState) -> String {
    let weapon = g.weapon.as_ref().map_or("none".to_string(), |w| {
        format!("{} slain [{}]", card(w.card), cards(&w.slain))
    });
    format!(
        "seed {} rule {} hp {} room [{}] weapon {} deck {} discard {} rooms {} slain {} turns {} status {:?}",
        g.seed,
        g.rule.as_str(),
        g.health,
        cards(&g.room),
        weapon,
        g.deck.len(),
        g.discard.len(),
        g.rooms_cleared,
        g.monsters_slain,
        g.turns,
        g.status
    )
}

/// Watches the app between events.
pub struct Watch {
    seed: Option<u64>,
    turns: u32,
    status: Option<Status>,
    help: bool,
    last: String,
    at: Instant,
}

impl Watch {
    pub fn new() -> Watch {
        Watch {
            seed: None,
            turns: 0,
            status: None,
            help: false,
            last: String::new(),
            at: Instant::now() - Duration::from_secs(1),
        }
    }

    pub fn observe(&mut self, app: &App) {
        if !log::log_enabled!(log::Level::Info) {
            return;
        }
        let g = &app.game;
        // a new seed, or the same one dealt again (r, t)
        if self.seed != Some(g.seed) || g.turns < self.turns {
            log::info!("deal seed {} rule {}", g.seed, g.rule.as_str());
            self.seed = Some(g.seed);
        }
        self.turns = g.turns;
        if self.status != Some(g.status) {
            log::info!("status {:?} hp {}", g.status, g.health);
            self.status = Some(g.status);
        }
        if self.help != app.show_help {
            log::info!("help {}", if app.show_help { "open" } else { "closed" });
            self.help = app.show_help;
        }
        let now = table(g);
        if now != self.last && self.at.elapsed() >= Duration::from_secs(1) {
            log::debug!("snapshot selected {} {}", app.selected, now);
            self.last = now;
            self.at = Instant::now();
        }
    }
}
