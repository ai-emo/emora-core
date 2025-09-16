use emora_core::{Brain, Stimulus};

fn main() {
    let mut agent = Brain::new();

    println!("初始: {:?}", agent);

    let mut step = 0;
    while agent.energy > 0.0 {
        println!("\n-- Tick {} --", step + 1);

        if step == 2 {
            agent.perceive(Stimulus::Threat(30.0));
        }

        agent.tick();
        step += 1;
    }
    println!("⚠️ 能量耗尽，智能体死亡");
}