# Rust Math Battle Game - Architecture Document

## Project Structure

```
math_battle_game/
├── Cargo.toml
├── index.html
├── Trunk.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── game/
│   │   ├── mod.rs
│   │   ├── states.rs
│   │   ├── systems.rs
│   │   └── events.rs
│   ├── entities/
│   │   ├── mod.rs
│   │   ├── player.rs
│   │   ├── boss.rs
│   │   └── battle.rs
│   ├── math/
│   │   ├── mod.rs
│   │   ├── problems.rs
│   │   └── difficulty.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── battle_ui.rs
│   │   └── components.rs
│   └── resources/
│       ├── mod.rs
│       ├── game_state.rs
│       └── assets.rs
```

## Core Architecture

### 1. Game States (ECS State Management)

```rust
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    BossSelection,
    Battle,
    Victory,
    Defeat,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BattleState {
    #[default]
    Waiting,
    ChoosingAttack,
    SolvingProblem,
    ProcessingResult,
    EnemyTurn,
    BattleEnd,
}
```

### 2. Entity Components

```rust
// Player creature component
#[derive(Component)]
pub struct PlayerCreature {
    pub name: String,
    pub creature_type: CreatureType,
    pub level: u32,
    pub experience: u32,
    pub experience_to_next: u32,
}

// Boss component with strategy pattern
#[derive(Component)]
pub struct Boss {
    pub name: String,
    pub boss_type: BossType,
    pub math_strategy: Box<dyn MathStrategy>,
    pub difficulty: Difficulty,
}

// Health component (shared by player and boss)
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub maximum: i32,
}

// Combat stats component
#[derive(Component)]
pub struct CombatStats {
    pub attack_power: i32,
    pub defense: i32,
    pub speed: f32,
}
```

### 3. Math Strategy Pattern

```rust
pub trait MathStrategy: Send + Sync {
    fn generate_problem(&self, difficulty: &Difficulty) -> MathProblem;
    fn get_problem_type_name(&self) -> &'static str;
    fn calculate_damage_modifier(&self, correct: bool, speed: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct MathProblem {
    pub question: String,
    pub answer: i32,
    pub problem_type: String,
    pub difficulty_level: u8,
}

// Concrete strategy implementations
pub struct AdditionStrategy;
pub struct SubtractionStrategy;
pub struct MultiplicationStrategy;
pub struct DivisionStrategy;
pub struct MixedStrategy; // For advanced bosses

impl MathStrategy for AdditionStrategy {
    fn generate_problem(&self, difficulty: &Difficulty) -> MathProblem {
        let (min, max) = difficulty.get_range();
        let a = fastrand::i32(min..=max);
        let b = fastrand::i32(min..=max);
        
        MathProblem {
            question: format!("{} + {}", a, b),
            answer: a + b,
            problem_type: "Addition".to_string(),
            difficulty_level: difficulty.level(),
        }
    }
    
    fn get_problem_type_name(&self) -> &'static str { "Addition" }
    
    fn calculate_damage_modifier(&self, correct: bool, speed: f32) -> f32 {
        if correct {
            1.0 + (1.0 / speed.max(1.0)) * 0.1 // Speed bonus
        } else {
            0.0
        }
    }
}
```

### 4. Boss Factory Pattern

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossType {
    AdditionWolf,
    SubtractionBear,
    MultiplicationDragon,
    DivisionWizard,
    AlgebraLord, // Mixed operations
}

pub struct BossFactory;

impl BossFactory {
    pub fn create_boss(boss_type: BossType) -> (Boss, Health, CombatStats) {
        match boss_type {
            BossType::AdditionWolf => (
                Boss {
                    name: "Arithmetic Wolf".to_string(),
                    boss_type,
                    math_strategy: Box::new(AdditionStrategy),
                    difficulty: Difficulty::Easy,
                },
                Health { current: 100, maximum: 100 },
                CombatStats { attack_power: 15, defense: 5, speed: 1.2 },
            ),
            BossType::MultiplicationDragon => (
                Boss {
                    name: "Times Table Dragon".to_string(),
                    boss_type,
                    math_strategy: Box::new(MultiplicationStrategy),
                    difficulty: Difficulty::Medium,
                },
                Health { current: 150, maximum: 150 },
                CombatStats { attack_power: 25, defense: 10, speed: 0.8 },
            ),
            // ... other boss configurations
        }
    }
    
    pub fn get_available_bosses() -> Vec<BossType> {
        vec![
            BossType::AdditionWolf,
            BossType::SubtractionBear,
            BossType::MultiplicationDragon,
            BossType::DivisionWizard,
        ]
    }
}
```

### 5. Combat System

```rust
pub struct CombatSystem;

impl CombatSystem {
    pub fn player_attack(
        commands: &mut Commands,
        player_query: &Query<&CombatStats, With<PlayerCreature>>,
        boss_query: &mut Query<(&Boss, &mut Health), Without<PlayerCreature>>,
        attack_type: AttackType,
        user_answers: &[i32],
        problems: &[MathProblem],
        answer_times: &[f32],
    ) -> CombatResult {
        // Combat logic here
        // Calculate damage based on correctness, speed, and attack type
        // Return result for UI feedback
    }
}

#[derive(Debug)]
pub enum AttackType {
    Basic,     // 1 problem
    Combo,     // 3 problems
    Special,   // Boss-specific mechanics
}

#[derive(Debug)]
pub struct CombatResult {
    pub damage_dealt: i32,
    pub all_correct: bool,
    pub streak_bonus: i32,
    pub speed_bonus: f32,
}
```

### 6. Resource Management

```rust
#[derive(Resource)]
pub struct GameResources {
    pub score: u32,
    pub streak: u32,
    pub current_boss: Option<BossType>,
    pub battle_log: Vec<String>,
}

#[derive(Resource)]
pub struct BattleContext {
    pub current_problems: Vec<MathProblem>,
    pub user_answers: Vec<String>,
    pub answer_start_times: Vec<f32>,
    pub current_attack_type: Option<AttackType>,
    pub question_index: usize,
}
```

### 7. Plugin Architecture

```rust
pub struct MathBattlePlugin;

impl Plugin for MathBattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_state::<BattleState>()
            .add_plugins((
                GameStatePlugin,
                BattlePlugin,
                UIPlugin,
                MathPlugin,
            ))
            .add_event::<BattleEvent>()
            .add_event::<UIEvent>()
            .insert_resource(GameResources::default())
            .insert_resource(BattleContext::default());
    }
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_attack_selection.run_if(in_state(BattleState::ChoosingAttack)),
                process_math_input.run_if(in_state(BattleState::SolvingProblem)),
                calculate_combat_result.run_if(in_state(BattleState::ProcessingResult)),
                enemy_turn.run_if(in_state(BattleState::EnemyTurn)),
            ))
            .add_systems(OnEnter(GameState::Battle), setup_battle)
            .add_systems(OnExit(GameState::Battle), cleanup_battle);
    }
}
```

### 8. Future Extensibility

To add new boss types:

1. **Create new MathStrategy implementation**:
```rust
pub struct FractionStrategy;
impl MathStrategy for FractionStrategy {
    // Implementation for fraction problems
}
```

2. **Add to BossType enum**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossType {
    // existing variants...
    FractionPhoenix,
}
```

3. **Update BossFactory**:
```rust
BossType::FractionPhoenix => (
    Boss {
        name: "Fraction Phoenix".to_string(),
        boss_type,
        math_strategy: Box::new(FractionStrategy),
        difficulty: Difficulty::Hard,
    },
    // stats...
),
```

### 9. Key Cargo.toml Dependencies

```toml
[dependencies]
bevy = { version = "0.12", features = ["webgl2"] }
web-sys = "0.3"
wasm-bindgen = "0.2"
fastrand = "2.0"
serde = { version = "1.0", features = ["derive"] }

[dependencies.bevy]
version = "0.12"
default-features = false
features = [
    "bevy_winit",
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_sprite",
    "bevy_text",
    "bevy_ui",
    "webgl2",
    "png",
]
```

## Benefits of This Architecture

1. **Scalable**: Easy to add new boss types and math strategies
2. **Maintainable**: Clear separation of concerns
3. **Testable**: Each component can be unit tested
4. **Performance**: Leverages Bevy's ECS for efficient updates
5. **Modular**: Plugins can be developed independently
6. **Type-safe**: Rust's type system prevents many runtime errors

## Next Steps

1. Implement the core ECS components and systems
2. Create basic UI systems using Bevy UI
3. Implement the math strategy pattern
4. Add boss factory and combat system
5. Create WASM build configuration with Trunk
6. Add visual effects and animations
7. Implement save/load functionality