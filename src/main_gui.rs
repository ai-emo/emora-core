//! 电子生物GUI - 可爱版

use eframe::egui;
use emora_core::{Brain, HabitBrain, Stimulus, World};

struct CreatureApp {
    brain_type: BrainType,
    reflex_brain: Brain,
    habit_brain: HabitBrain,
    world: World,
    running: bool,
    speed: f32,
    ticks: usize,
    trace: Vec<(f32, f32)>,
    dead: bool,
    death_reason: String,
    creature_scale: f32,
    anim_timer: f32,
    last_update: std::time::Instant,
    update_interval: f32,
}

#[derive(PartialEq, Clone)]
enum BrainType { Reflex, Habit }

impl Default for CreatureApp {
    fn default() -> Self {
        let mut world = World::new();
        world.add_food(80.0, 50.0, 25.0);
        world.add_food(150.0, 120.0, 20.0);
        world.add_food(50.0, 180.0, 30.0);
        world.add_food(300.0, 80.0, 25.0);
        world.add_food(350.0, 200.0, 30.0);
        world.add_danger(200.0, 80.0, 40.0);
        world.add_danger(30.0, 100.0, 35.0);
        world.add_danger(280.0, 250.0, 45.0);

        let mut habit_brain = HabitBrain::new();
        habit_brain.set_world(world.clone());
        habit_brain.perceive(Stimulus::Food(-35.0));

        let mut reflex_brain = Brain::new();
        reflex_brain.set_world(world.clone());
        reflex_brain.perceive(Stimulus::Food(-35.0));

        CreatureApp {
            brain_type: BrainType::Habit,
            reflex_brain,
            habit_brain,
            world,
            running: false,
            speed: 1.0,
            ticks: 0,
            trace: Vec::new(),
            dead: false,
            death_reason: String::new(),
            creature_scale: 1.0,
            anim_timer: 0.0,
            last_update: std::time::Instant::now(),
            update_interval: 0.5,
        }
    }
}

impl CreatureApp {
    fn step(&mut self) {
        if self.dead { return; }
        self.ticks += 1;
        self.creature_scale = 1.25;
        self.anim_timer += 0.5;

        let (pos, energy, safety) = match self.brain_type {
            BrainType::Reflex => {
                self.reflex_brain.tick_reflex();
                // 同步大脑的world到GUI
                self.world.foods = self.reflex_brain.world.foods.clone();
                self.world.dangers = self.reflex_brain.world.dangers.clone();
                (self.reflex_brain.position, self.reflex_brain.energy, 50.0f32)
            }
            BrainType::Habit => {
                self.habit_brain.tick_habit();
                // 同步大脑的world到GUI
                self.world.foods = self.habit_brain.world.foods.clone();
                self.world.dangers = self.habit_brain.world.dangers.clone();
                (self.habit_brain.position, self.habit_brain.energy, 50.0f32)
            }
        };

        if energy <= 0.0 { self.dead = true; self.death_reason = "Starved...".to_string(); self.running = false; }
        else if safety <= 0.0 { self.dead = true; self.death_reason = "Ouch!".to_string(); self.running = false; }

        // 食物自动重生（当少于3个时）
        if self.world.foods.len() < 3 {
            use rand::Rng;
            let mut rng = rand::rng();
            let x = rng.random_range(20.0..380.0);
            let y = rng.random_range(20.0..380.0);
            let e = rng.random_range(15.0..35.0);
            self.world.add_food(x, y, e);
            // 同步到大脑
            match self.brain_type {
                BrainType::Reflex => self.reflex_brain.world.add_food(x, y, e),
                BrainType::Habit => self.habit_brain.world.add_food(x, y, e),
            }
        }

        self.trace.push(pos);
        if self.trace.len() > 400 { self.trace.remove(0); }
    }
}

impl eframe::App for CreatureApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.creature_scale = self.creature_scale * 0.92 + 1.0 * 0.08;
        self.anim_timer += 0.03;

        // 使用计时器控制更新频率，不阻塞主线程
        if self.running && !self.dead {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(self.last_update).as_secs_f32();
            if elapsed >= self.update_interval / self.speed {
                self.step();
                self.last_update = now;
            }
            ctx.request_repaint();
        }

        // 顶部
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("~ Electronic Creature ~").size(24.0).color(egui::Color32::from_rgb(255, 150, 200)));
                ui.add_space(8.0);
            });
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Brain: ").color(egui::Color32::GRAY));
                ui.selectable_value(&mut self.brain_type, BrainType::Reflex, "Reflex");
                ui.selectable_value(&mut self.brain_type, BrainType::Habit, "Habit");
                ui.separator();
                if ui.button(if self.running { "Pause" } else { "Play" }).clicked() { self.running = !self.running; }
                if ui.button("Step").clicked() { self.step(); }
                if ui.button("Reset").clicked() { *self = Self::default(); }
                ui.separator();
                ui.add(egui::Slider::new(&mut self.speed, 0.5..=8.0).text("Speed"));
                ui.separator();
                ui.label(egui::RichText::new(format!("Steps: {}", self.ticks)).small().color(egui::Color32::GRAY));
            });
            ui.add_space(5.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (energy, pad, position, safety) = match self.brain_type {
                BrainType::Reflex => (
                    self.reflex_brain.energy,
                    &self.reflex_brain.current_pad,
                    self.reflex_brain.position,
                    self.reflex_brain.safety
                ),
                BrainType::Habit => (
                    self.habit_brain.energy,
                    &self.habit_brain.current_pad,
                    self.habit_brain.position,
                    self.habit_brain.safety
                ),
            };

            let avail = ui.available_size();
            let map_w = (avail.x - 60.0).max(450.0);
            let map_h = (avail.y - 180.0).max(380.0);
            let map_rect = egui::Rect::from_min_size(ui.available_rect_before_wrap().min + egui::Vec2::new(30.0, 10.0), egui::Vec2::new(map_w, map_h));
            let painter = ui.painter_at(map_rect);

            // 背景 - 柔和的草地色
            painter.rect_filled(map_rect, 12.0, egui::Color32::from_rgb(30, 60, 35));

            // 网格
            for i in 0..=10 {
                let x = map_rect.min.x + (map_w / 10.0) * i as f32;
                let y = map_rect.min.y + (map_h / 10.0) * i as f32;
                let color = if i % 2 == 0 { egui::Color32::from_rgba_unmultiplied(80, 120, 80, 30) } else { egui::Color32::from_rgba_unmultiplied(60, 100, 60, 20) };
                painter.line_segment([egui::pos2(x, map_rect.min.y), egui::pos2(x, map_rect.max.y)], egui::Stroke::new(0.5, color));
                painter.line_segment([egui::pos2(map_rect.min.x, y), egui::pos2(map_rect.max.x, y)], egui::Stroke::new(0.5, color));
            }

            // 危险 - 粉色警示
            let pulse = (self.anim_timer * 2.0).sin() * 0.15 + 0.85;
            for (x, y, t) in &self.world.dangers {
                let px = map_rect.min.x + (*x / 400.0) * map_w;
                let py = map_rect.min.y + (*y / 400.0) * map_h;
                painter.circle_filled(egui::pos2(px, py), 35.0 * pulse, egui::Color32::from_rgba_unmultiplied(200, 50, 100, 15));
                painter.circle_filled(egui::pos2(px, py), 25.0 * pulse, egui::Color32::from_rgba_unmultiplied(220, 80, 130, 35));
                painter.circle_filled(egui::pos2(px, py), 15.0, egui::Color32::from_rgba_unmultiplied(230, 100, 150, 120));
                painter.circle_filled(egui::pos2(px, py), 8.0, egui::Color32::from_rgb(220, 100, 150));
                painter.text(egui::pos2(px, py - 22.0), egui::Align2::CENTER_TOP, format!("-{}", *t as i32), egui::FontId::proportional(12.0), egui::Color32::from_rgb(255, 150, 180));
            }

            // 食物 - 粉色糖果
            let float = (self.anim_timer * 1.5).sin() * 2.0;
            for (x, y, e) in &self.world.foods {
                let px = map_rect.min.x + (*x / 400.0) * map_w;
                let py = map_rect.min.y + (*y / 400.0) * map_h + float;
                let r = 5.0 + (*e / 30.0) * 4.0;
                painter.circle_filled(egui::pos2(px, py), r + 6.0, egui::Color32::from_rgba_unmultiplied(255, 180, 200, 40));
                painter.circle_filled(egui::pos2(px, py), r + 3.0, egui::Color32::from_rgba_unmultiplied(255, 150, 180, 80));
                painter.circle_filled(egui::pos2(px, py), r, egui::Color32::from_rgb(255, 120, 160));
                // 高光
                painter.circle_filled(egui::pos2(px - r*0.3, py - r*0.3), r*0.35, egui::Color32::from_rgba_unmultiplied(255, 220, 240, 200));
                painter.text(egui::pos2(px, py - r - 10.0), egui::Align2::CENTER_TOP, format!("+{}", *e as i32), egui::FontId::proportional(11.0), egui::Color32::WHITE);
            }

            // 轨迹
            for (i, (x, y)) in self.trace.iter().enumerate() {
                let px = map_rect.min.x + (*x / 400.0) * map_w;
                let py = map_rect.min.y + (*y / 400.0) * map_h;
                let t = i as f32 / self.trace.len() as f32;
                let alpha = (t * t * 180.0) as u8;
                let size = 1.0 + t * 2.5;
                painter.circle_filled(egui::pos2(px, py), size, egui::Color32::from_rgba_unmultiplied(200, 150, 255, alpha));
            }

            // 可爱的生物！
            let px = map_rect.min.x + (position.0 / 400.0) * map_w;
            let py = map_rect.min.y + (position.1 / 400.0) * map_h;

            // 情绪颜色 - 粉色系
            let mood = if pad.pleasure > 0.3 && pad.arousal > 0.5 { egui::Color32::from_rgb(255, 200, 100) }
                else if pad.pleasure < -0.3 && pad.arousal > 0.5 { egui::Color32::from_rgb(200, 100, 150) }
                else if pad.pleasure < -0.3 { egui::Color32::from_rgb(150, 130, 200) }
                else if pad.pleasure > 0.3 { egui::Color32::from_rgb(150, 220, 150) }
                else if pad.pleasure > 0.0 { egui::Color32::from_rgb(200, 220, 150) }
                else { egui::Color32::from_rgb(200, 180, 200) };

            let base_r = 18.0 * self.creature_scale;

            // 身体光晕
            painter.circle_filled(egui::pos2(px, py), base_r + 14.0, mood.linear_multiply(0.15));
            painter.circle_filled(egui::pos2(px, py), base_r + 8.0, mood.linear_multiply(0.35));
            painter.circle_filled(egui::pos2(px, py), base_r + 3.0, mood.linear_multiply(0.6));

            // 身体
            painter.circle_filled(egui::pos2(px, py), base_r, mood);
            painter.circle_filled(egui::pos2(px, py), base_r, mood);

            // 腮红
            let blush_r = base_r * 0.35;
            painter.circle_filled(egui::pos2(px - base_r*0.5, py + base_r*0.1), blush_r, egui::Color32::from_rgba_unmultiplied(255, 150, 150, 100));
            painter.circle_filled(egui::pos2(px + base_r*0.5, py + base_r*0.1), blush_r, egui::Color32::from_rgba_unmultiplied(255, 150, 150, 100));

            // 眼睛 - 大眼睛
            let eye_r = base_r * 0.22;
            let eye_y = py - base_r * 0.1;
            let eye_spacing = base_r * 0.4;
            // 眼睛背景
            painter.circle_filled(egui::pos2(px - eye_spacing, eye_y), eye_r + 2.0, egui::Color32::BLACK);
            painter.circle_filled(egui::pos2(px + eye_spacing, eye_y), eye_r + 2.0, egui::Color32::BLACK);
            // 眼珠
            painter.circle_filled(egui::pos2(px - eye_spacing, eye_y), eye_r, egui::Color32::WHITE);
            painter.circle_filled(egui::pos2(px + eye_spacing, eye_y), eye_r, egui::Color32::WHITE);
            // 瞳孔
            let pupil_r = eye_r * 0.5;
            painter.circle_filled(egui::pos2(px - eye_spacing + 1.0, eye_y), pupil_r, egui::Color32::BLACK);
            painter.circle_filled(egui::pos2(px + eye_spacing + 1.0, eye_y), pupil_r, egui::Color32::BLACK);
            // 高光
            painter.circle_filled(egui::pos2(px - eye_spacing - 1.0, eye_y - 1.0), pupil_r * 0.4, egui::Color32::WHITE);
            painter.circle_filled(egui::pos2(px + eye_spacing - 1.0, eye_y - 1.0), pupil_r * 0.4, egui::Color32::WHITE);

            // 嘴巴 - 根据情绪变化
            let mouth_y = py + base_r * 0.35;
            let mouth_w = base_r * 0.25;
            if pad.pleasure > 0.3 {
                // 开心 - 笑容
                painter.line_segment([egui::pos2(px - mouth_w, mouth_y - 2.0), egui::pos2(px, mouth_y + 3.0)], egui::Stroke::new(2.0, egui::Color32::BLACK));
                painter.line_segment([egui::pos2(px, mouth_y + 3.0), egui::pos2(px + mouth_w, mouth_y - 2.0)], egui::Stroke::new(2.0, egui::Color32::BLACK));
            } else if pad.pleasure < -0.3 {
                // 不开心 - 嘴角向下
                painter.line_segment([egui::pos2(px - mouth_w, mouth_y + 2.0), egui::pos2(px, mouth_y - 3.0)], egui::Stroke::new(2.0, egui::Color32::BLACK));
                painter.line_segment([egui::pos2(px, mouth_y - 3.0), egui::pos2(px + mouth_w, mouth_y + 2.0)], egui::Stroke::new(2.0, egui::Color32::BLACK));
            } else {
                // 普通 - 微笑
                painter.line_segment([egui::pos2(px - mouth_w * 0.8, mouth_y), egui::pos2(px + mouth_w * 0.8, mouth_y)], egui::Stroke::new(2.0, egui::Color32::BLACK));
            }

            // 能量条
            let bw = 40.0;
            let bh = 5.0;
            let bar_x = px - bw/2.0; let bar_y = py + base_r + 15.0;
            painter.rect_filled(egui::Rect::from_min_size(egui::pos2(bar_x, bar_y), egui::Vec2::new(bw, bh)), 2.0, egui::Color32::from_rgb(30, 25, 30));
            let ec = if energy < 25.0 { egui::Color32::from_rgb(240, 100, 130) } else if energy < 50.0 { egui::Color32::from_rgb(240, 180, 100) } else { egui::Color32::from_rgb(150, 220, 130) };
            painter.rect_filled(egui::Rect::from_min_size(egui::pos2(bar_x, bar_y), egui::Vec2::new(bw * energy / 100.0, bh)), 2.0, ec);

            // 状态面板
            ui.add_space(map_h + 12.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.set_width(map_w);
                    ui.label(egui::RichText::new("Energy:").size(13.0).color(egui::Color32::GRAY));
                    ui.add(egui::ProgressBar::new(energy / 100.0).fill(ec).desired_width(80.0));
                    ui.label(egui::RichText::new(format!("{:>4.0}%", energy)).size(13.0).color(ec));
                    ui.separator();
                    // 安全条
                    let sc = if safety < 25.0 { egui::Color32::from_rgb(240, 100, 130) } else if safety < 50.0 { egui::Color32::from_rgb(240, 180, 100) } else { egui::Color32::from_rgb(150, 200, 220) };
                    ui.label(egui::RichText::new("Safety:").size(13.0).color(egui::Color32::GRAY));
                    ui.add(egui::ProgressBar::new(safety / 100.0).fill(sc).desired_width(80.0));
                    ui.label(egui::RichText::new(format!("{:>4.0}%", safety)).size(13.0).color(sc));
                    ui.separator();
                    // 感受显示（基于PAD值）
                    let feeling = get_feeling_text(pad.pleasure, pad.arousal);
                    let feeling_color = get_feeling_color(pad.pleasure, pad.arousal);
                    ui.label(egui::RichText::new(feeling).size(14.0).color(feeling_color));
                    ui.separator();
                    ui.label(egui::RichText::new(format!("Food: {}", self.world.foods.len())).size(13.0).color(egui::Color32::GRAY));
                });
            });

            // 死亡
            if self.dead {
                let overlay = map_rect.expand(40.0);
                painter.rect_filled(overlay, 25.0, egui::Color32::from_rgba_unmultiplied(20, 10, 20, 220));
                let blink = (self.anim_timer * 3.0).sin().abs();
                let title_color = egui::Color32::from_rgba_unmultiplied(255, 100, 150, (200.0 + blink * 55.0) as u8);
                painter.text(map_rect.center() + egui::Vec2::new(0.0, -25.0), egui::Align2::CENTER_CENTER, "X_X", egui::FontId::proportional(36.0), title_color);
                painter.text(map_rect.center() + egui::Vec2::new(0.0, 20.0), egui::Align2::CENTER_CENTER, &self.death_reason, egui::FontId::proportional(18.0), egui::Color32::from_rgb(200, 150, 180));
                painter.text(map_rect.center() + egui::Vec2::new(0.0, 55.0), egui::Align2::CENTER_CENTER, "Press Reset", egui::FontId::proportional(14.0), egui::Color32::GRAY);
            }
        });
    }
}

fn get_feeling_text(pleasure: f32, arousal: f32) -> String {
    if pleasure > 0.3 && arousal > 0.5 { "Feeling good!" }
    else if pleasure > 0.3 { "Content" }
    else if pleasure < -0.3 && arousal > 0.5 { "Anxious" }
    else if pleasure < -0.3 { "Uncomfortable" }
    else if arousal > 0.7 { "Alert" }
    else if arousal < 0.3 { "Relaxed" }
    else { "Neutral" }
    .to_string()
}

fn get_feeling_color(pleasure: f32, arousal: f32) -> egui::Color32 {
    if pleasure > 0.3 && arousal > 0.5 { egui::Color32::from_rgb(255, 200, 100) }
    else if pleasure > 0.3 { egui::Color32::from_rgb(150, 220, 150) }
    else if pleasure < -0.3 && arousal > 0.5 { egui::Color32::from_rgb(255, 100, 150) }
    else if pleasure < -0.3 { egui::Color32::from_rgb(200, 100, 150) }
    else if arousal > 0.7 { egui::Color32::from_rgb(255, 180, 150) }
    else if arousal < 0.3 { egui::Color32::from_rgb(150, 200, 220) }
    else { egui::Color32::from_rgb(200, 200, 200) }
}

fn main() {
    let opts = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 720.0]).with_min_inner_size([800.0, 600.0]), ..Default::default() };
    let _ = eframe::run_native("Electronic Creature", opts, Box::new(|_cc| Ok(Box::new(CreatureApp::default()))));
}
