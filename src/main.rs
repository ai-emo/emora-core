use emora_core::{Brain, Stimulus, HabitBrain, World};

fn main() {
    let mut agent = HabitBrain::new();

    // 创建世界环境
    let mut world = World::new();
    world.add_food(10.0, 5.0, 20.0);
    world.add_food(30.0, -10.0, 15.0);
    world.add_danger(-20.0, 0.0, 30.0);

    agent.set_world(world);

    println!("=== 习惯生物测试 (阶段2: TD学习) ===");
    println!("初始状态: 能量={:.1}, 安全={:.1}", agent.energy, 50.0);

    // 降低能量触发饥饿
    agent.perceive(Stimulus::Food(-40.0));

    let mut step = 0;
    while agent.energy > 0.0 && step < 8 {
        agent.tick_habit();
        step += 1;
    }

    if agent.energy <= 0.0 {
        println!("\n能量耗尽，智能体死亡");
    } else {
        println!("\n测试完成");
        println!("最终记忆条目数: {}", agent.get_learner().get_short_term().all().len());
    }
}
