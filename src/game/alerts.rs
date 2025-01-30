use gemini_engine::{
    ascii::{Text, TextAlign},
    core::{CanDraw, Modifier, Vec2D},
};

const ALERT_LIFETIME: u16 = 20;

pub fn generate_alert_for_filled_lines(cleared_lines: i64) -> Option<(i64, String)> {
    match cleared_lines {
        1 => Some((100, String::from("Single!"))),
        2 => Some((300, String::from("Double!"))),
        3 => Some((500, String::from("Triple!"))),
        4 => Some((800, String::from("Tetris!"))),
        0 => None,
        _ => panic!("entered value should be between 0 and 4"),
    }
}

pub struct AlertDisplay {
    pub pos: Vec2D,
    alerts: Vec<(String, u16)>,
}

impl AlertDisplay {
    pub const fn new(pos: Vec2D) -> Self {
        Self {
            pos,
            alerts: vec![],
        }
    }

    pub fn push(&mut self, alert: &str) {
        self.alerts.push((String::from(alert), ALERT_LIFETIME));
    }

    pub fn handle_with_score(&mut self, score: &mut i64, score_and_alert: Option<(i64, String)>) {
        if let Some((add_score, alert)) = score_and_alert {
            *score += add_score;
            self.push(&alert);
        }
    }

    /// Will pick the first existing alert score pair and run `handle_with_score` on that
    pub fn priorised_alerts_with_score(
        &mut self,
        alert_score_pairs: &[Option<(i64, String)>],
        score: &mut i64,
    ) {
        for score_alert_pair in alert_score_pairs {
            if score_alert_pair.is_some() {
                self.handle_with_score(score, score_alert_pair.clone());
                break;
            }
        }
    }

    pub fn frame(&mut self) {
        if !self.alerts.is_empty() {
            let mut i = 0;
            loop {
                if self.alerts.get(i).is_none() {
                    break;
                }
                self.alerts[i].1 -= 1;
                if self.alerts[i].1 == 0 {
                    self.alerts.remove(i);
                    i = i.saturating_sub(1);
                }

                i += 1;
            }
        }
    }
}

impl CanDraw for AlertDisplay {
    fn draw_to(&self, canvas: &mut impl gemini_engine::core::Canvas) {
        self.alerts.iter().enumerate().for_each(|(i, (alert, _))| {
            Text::new(self.pos + Vec2D::new(0, i as i64), alert, Modifier::None)
                .with_align(TextAlign::Centered)
                .draw_to(canvas);
        });
    }
}

impl Default for AlertDisplay {
    fn default() -> Self {
        Self::new(Vec2D::ZERO)
    }
}
