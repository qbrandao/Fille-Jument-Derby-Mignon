use bevy::prelude::*;

// ============================================================
// États du jeu
// ============================================================
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    LaunchScreen,
    MainMenu,
    UmaList,
    Career,
    CareerEvent,
    CareerRaceResult,
    CareerEnd,
}

// ============================================================
// Composants Uma
// ============================================================
#[derive(Component)]
struct Uma {
    name: String,
}

#[derive(Component)]
struct UmaName(String);

#[derive(Component, Clone)]
struct UmaStats {
    speed:   i32,
    stamina: i32,
    power:   i32,
    guts:    i32,
    wit:     i32,
}

impl UmaStats {
    fn for_uma(name: &str) -> Self {
        match name {
            "Dariz" => Self { speed: 112, stamina: 102, power: 115, guts: 109, wit: 112 },
            "Calandagan" => Self { speed: 108, stamina: 123, power: 114, guts: 102, wit: 103 },
            "Aventure"   => Self { speed: 115, stamina: 106, power: 109, guts: 115, wit: 105 },
            _ => Self { speed: 110, stamina: 110, power: 110, guts: 110, wit: 110 },
        }
    }

    fn apply_training(&mut self, stat: TrainingStat) {
        match stat {
            TrainingStat::Speed   => self.speed   += 5,
            TrainingStat::Stamina => self.stamina += 5,
            TrainingStat::Power   => self.power   += 5,
            TrainingStat::Guts    => self.guts    += 5,
            TrainingStat::Wit     => self.wit     += 5,
        }
    }

    fn apply_event_bonus(&mut self, stat: TrainingStat) {
        match stat {
            TrainingStat::Speed   => self.speed   += 8,
            TrainingStat::Stamina => self.stamina += 8,
            TrainingStat::Power   => self.power   += 8,
            TrainingStat::Guts    => self.guts    += 8,
            TrainingStat::Wit     => self.wit     += 8,
        }
    }
}

// ============================================================
// Ressource Carrière
// ============================================================
#[derive(Clone, Copy, Debug, PartialEq)]
enum TrainingStat {
    Speed,
    Stamina,
    Power,
    Guts,
    Wit,
}

impl TrainingStat {
    fn label(&self) -> &'static str {
        match self {
            TrainingStat::Speed   => "Vitesse",
            TrainingStat::Stamina => "Endurance",
            TrainingStat::Power   => "Puissance",
            TrainingStat::Guts    => "Cran",
            TrainingStat::Wit     => "Esprit",
        }
    }

    fn from_index(i: u32) -> Self {
        match i % 5 {
            0 => TrainingStat::Speed,
            1 => TrainingStat::Stamina,
            2 => TrainingStat::Power,
            3 => TrainingStat::Guts,
            _ => TrainingStat::Wit,
        }
    }
}

#[derive(Resource)]
struct CareerState {
    uma_name:           String,
    stats:              UmaStats,
    turn:               u32,
    max_turns:          u32,
    wins:               u32,
    pending_event:      Option<TrainingStat>,
    last_race_message:  Option<String>,
}

impl Default for CareerState {
    fn default() -> Self {
        Self {
            uma_name: String::new(),
            stats: UmaStats { speed: 0, stamina: 0, power: 0, guts: 0, wit: 0 },
            turn: 1,
            max_turns: 72,
            wins: 0,
            pending_event: None,
            last_race_message: None,
        }
    }
}

impl CareerState {
    fn new(uma_name: &str, stats: UmaStats) -> Self {
        Self {
            uma_name: uma_name.to_string(),
            stats,
            turn: 1,
            max_turns: 72,
            wins: 0,
            pending_event: None,
            last_race_message: None,
        }
    }
}

// ============================================================
// Composants boutons — navigation
// ============================================================
#[derive(Component)] struct StartButton;
#[derive(Component)] struct CareerButton;
#[derive(Component)] struct UmaListButton;
#[derive(Component)] struct BackButton;

// Boutons carrière
#[derive(Component)] struct TrainSpeedButton;
#[derive(Component)] struct TrainStaminaButton;
#[derive(Component)] struct TrainPowerButton;
#[derive(Component)] struct TrainGutsButton;
#[derive(Component)] struct TrainWitButton;
#[derive(Component)] struct RaceButton;
#[derive(Component)] struct EventContinueButton;
#[derive(Component)] struct RaceResultContinueButton;
#[derive(Component)] struct EndCareerButton;

// ============================================================
// Ressource sprites Uma
// ============================================================
#[derive(Resource)]
struct UmaAssets {
    uma_sprites: Vec<Handle<Image>>,
}

impl UmaAssets {
    fn new() -> Self { Self { uma_sprites: Vec::new() } }
}

// ============================================================
// Marqueurs d'écran
// ============================================================
#[derive(Component)] struct OnLaunchScreen;
#[derive(Component)] struct OnMainMenu;
#[derive(Component)] struct OnUmaList;
#[derive(Component)] struct OnCareer;
#[derive(Component)] struct OnCareerEvent;
#[derive(Component)] struct OnCareerRaceResult;
#[derive(Component)] struct OnCareerEnd;

#[derive(Component)]
struct UmaListItem { uma_entity: Entity }

// ============================================================
// Couleurs & styles
// ============================================================
const BUTTON_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);
const TRAIN_COLOR:  Color = Color::srgb(0.2, 0.6, 1.0);
const RACE_COLOR:   Color = Color::srgb(1.0, 0.4, 0.2);
const BG_CAREER:    Color = Color::srgb(0.08, 0.08, 0.15);

fn button_node() -> Node {
    Node {
        width: Val::Px(220.0), height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(8.0)),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        ..default()
    }
}

fn wide_button_node() -> Node {
    Node {
        width: Val::Px(320.0), height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        ..default()
    }
}

// ============================================================
// Setup caméra
// ============================================================
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ============================================================
// Écran de lancement
// ============================================================
fn setup_launch_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            align_items: AlignItems::Center, justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column, ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
        OnLaunchScreen,
    )).with_children(|p| {
        p.spawn((Text::new("Bienvenue dans Fille-Jument Derby Mignon"),
            TextFont { font_size: 30.0, ..default() }, TextColor(Color::WHITE), OnLaunchScreen));
        p.spawn((Button, button_node(), BackgroundColor(BUTTON_COLOR), StartButton, OnLaunchScreen))
            .with_children(|p| { p.spawn((Text::new("Commencer à jouer"),
                TextFont { font_size: 20.0, ..default() }, TextColor(Color::BLACK))); });
    });
}

// ============================================================
// Menu principal
// ============================================================
fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            align_items: AlignItems::Center, justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column, ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
        OnMainMenu,
    )).with_children(|p| {
        p.spawn((Text::new("Menu Principal"),
            TextFont { font_size: 30.0, ..default() }, TextColor(Color::WHITE), OnMainMenu));
        p.spawn((Button, button_node(), BackgroundColor(BUTTON_COLOR), CareerButton, OnMainMenu))
            .with_children(|p| { p.spawn((Text::new("Carrière"),
                TextFont { font_size: 20.0, ..default() }, TextColor(Color::BLACK))); });
        p.spawn((Button, button_node(), BackgroundColor(BUTTON_COLOR), UmaListButton, OnMainMenu))
            .with_children(|p| { p.spawn((Text::new("Fille Jument"),
                TextFont { font_size: 20.0, ..default() }, TextColor(Color::BLACK))); });
    });
}

// ============================================================
// Liste des Uma
// ============================================================
fn setup_uma_list(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            align_items: AlignItems::Center, justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(20.0)), ..default()
        },
        BackgroundColor(Color::srgb(0.5, 0.0, 0.5)),
        OnUmaList,
    )).with_children(|p| {
        p.spawn((Text::new("Choisissez votre Uma"),
            TextFont { font_size: 30.0, ..default() }, TextColor(Color::WHITE), OnUmaList));
        p.spawn((Button, button_node(), BackgroundColor(BUTTON_COLOR), BackButton, OnUmaList))
            .with_children(|p| { p.spawn((Text::new("Retour"),
                TextFont { font_size: 20.0, ..default() }, TextColor(Color::BLACK))); });

        let uma_data = vec![("Dariz", 0usize), ("Calandagan", 1), ("Aventure", 2)];
        for (name, _sprite_index) in uma_data {
            // Spawn une entité Uma sans sprite (les assets image sont optionnels pour la carrière)
            let uma_entity = p.commands().spawn((
                Uma { name: name.to_string() },
                UmaStats::for_uma(name),
                UmaName(name.to_string()),
            )).id();
            p.spawn((
                Button,
                Node {
                    width: Val::Px(300.0), height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center, align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.56, 0.93, 0.56)),
                UmaListItem { uma_entity },
                OnUmaList,
            )).with_children(|p| { p.spawn((Text::new(name),
                TextFont { font_size: 20.0, ..default() }, TextColor(Color::BLACK))); });
        }
    });
}

// ============================================================
// Écran principal de carrière
// ============================================================
fn stats_text(s: &UmaStats) -> String {
    format!(
        "VIT {:>3}  END {:>3}  PUI {:>3}  CRN {:>3}  ESP {:>3}",
        s.speed, s.stamina, s.power, s.guts, s.wit
    )
}

fn setup_career(mut commands: Commands, career: Res<CareerState>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center, justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(20.0)), ..default()
        },
        BackgroundColor(BG_CAREER),
        OnCareer,
    )).with_children(|p| {
        // En-tête
        p.spawn((Text::new(format!("Carrière de {}", career.uma_name)),
            TextFont { font_size: 28.0, ..default() }, TextColor(Color::WHITE), OnCareer));
        p.spawn((Text::new(format!("Tour {}/{}", career.turn, career.max_turns)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)), OnCareer));
        p.spawn((Text::new(format!("Victoires : {}", career.wins)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)), OnCareer));
        p.spawn((Text::new(stats_text(&career.stats)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.7, 1.0, 0.7)), OnCareer));

        p.spawn((Node { height: Val::Px(16.0), ..default() }, OnCareer));

        // Entraînement
        p.spawn((Text::new("── Entraînement ──"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.5, 0.8, 1.0)), OnCareer));

        let trainings: &[(&str, u8)] = &[
            ("Vitesse   +5", 0),
            ("Endurance +5", 1),
            ("Puissance +5", 2),
            ("Cran      +5", 3),
            ("Esprit    +5", 4),
        ];
        for (label, idx) in trainings {
            let label = *label;
            let idx = *idx;
            let mut btn = p.spawn((Button, button_node(), BackgroundColor(TRAIN_COLOR), OnCareer));
            match idx {
                0 => { btn.insert(TrainSpeedButton); }
                1 => { btn.insert(TrainStaminaButton); }
                2 => { btn.insert(TrainPowerButton); }
                3 => { btn.insert(TrainGutsButton); }
                _ => { btn.insert(TrainWitButton); }
            }
            btn.with_children(|p| { p.spawn((Text::new(label),
                TextFont { font_size: 18.0, ..default() }, TextColor(Color::WHITE))); });
        }

        // Course
        p.spawn((Node { height: Val::Px(10.0), ..default() }, OnCareer));
        p.spawn((Text::new("── Course ──"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(1.0, 0.6, 0.3)), OnCareer));
        p.spawn((Button, wide_button_node(), BackgroundColor(RACE_COLOR), RaceButton, OnCareer))
            .with_children(|p| { p.spawn((Text::new("Participer à une course"),
                TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
    });
}

// ============================================================
// Écran événement
// ============================================================
fn setup_career_event(mut commands: Commands, career: Res<CareerState>) {
    let stat = career.pending_event.unwrap();
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center, justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(40.0)), ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.05, 0.2)),
        OnCareerEvent,
    )).with_children(|p| {
        p.spawn((Text::new("Evenement !"),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::srgb(1.0, 0.9, 0.2)), OnCareerEvent));
        p.spawn((Text::new(format!("{} gagne +8 {} !", career.uma_name, stat.label())),
            TextFont { font_size: 24.0, ..default() }, TextColor(Color::WHITE), OnCareerEvent));
        p.spawn((Button, wide_button_node(), BackgroundColor(BUTTON_COLOR), EventContinueButton, OnCareerEvent))
            .with_children(|p| { p.spawn((Text::new("Continuer"),
                TextFont { font_size: 22.0, ..default() }, TextColor(Color::BLACK))); });
    });
}

// ============================================================
// Écran résultat de course
// ============================================================
fn setup_career_race_result(mut commands: Commands, career: Res<CareerState>) {
    let msg = career.last_race_message.clone().unwrap_or_default();
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center, justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(40.0)), ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.15, 0.05)),
        OnCareerRaceResult,
    )).with_children(|p| {
        p.spawn((Text::new("Resultat de course"),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)), OnCareerRaceResult));
        p.spawn((Text::new(msg),
            TextFont { font_size: 22.0, ..default() }, TextColor(Color::WHITE), OnCareerRaceResult));
        p.spawn((Button, wide_button_node(), BackgroundColor(BUTTON_COLOR), RaceResultContinueButton, OnCareerRaceResult))
            .with_children(|p| { p.spawn((Text::new("Continuer"),
                TextFont { font_size: 22.0, ..default() }, TextColor(Color::BLACK))); });
    });
}

// ============================================================
// Écran fin de carrière
// ============================================================
fn setup_career_end(mut commands: Commands, career: Res<CareerState>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center, justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(40.0)), ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.05, 0.05)),
        OnCareerEnd,
    )).with_children(|p| {
        p.spawn((Text::new("Fin de Carriere !"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)), OnCareerEnd));
        p.spawn((Text::new(format!("{} a termine sa carriere !", career.uma_name)),
            TextFont { font_size: 24.0, ..default() }, TextColor(Color::WHITE), OnCareerEnd));
        p.spawn((Text::new(format!("Victoires : {}", career.wins)),
            TextFont { font_size: 22.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)), OnCareerEnd));
        p.spawn((Text::new(stats_text(&career.stats)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.7, 1.0, 0.7)), OnCareerEnd));
        p.spawn((Button, wide_button_node(), BackgroundColor(BUTTON_COLOR), EndCareerButton, OnCareerEnd))
            .with_children(|p| { p.spawn((Text::new("Retour au menu"),
                TextFont { font_size: 22.0, ..default() }, TextColor(Color::BLACK))); });
    });
}

// ============================================================
// Chargement assets & spawn Uma
// ============================================================
fn load_uma_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UmaAssets {
        uma_sprites: vec![
            asset_server.load("uma/dariz.png"),
            asset_server.load("uma/calandagan.png"),
            asset_server.load("uma/aventure.png"),
        ],
    });
}

fn spawn_uma(
    commands: &mut Commands,
    uma_assets: &UmaAssets,
    name: &str,
    position: Vec3,
    sprite_index: usize,
) -> Entity {
    commands.spawn((
        Uma { name: name.to_string() },
        UmaStats::for_uma(name),
        Sprite { image: uma_assets.uma_sprites[sprite_index].clone(), color: Color::WHITE, ..default() },
        Transform::from_translation(position),
        Visibility::Hidden,
        UmaName(name.to_string()),
    )).id()
}

// ============================================================
// RNG minimal (LCG)
// ============================================================
#[derive(Resource)]
struct RngSeed(u64);

fn rand_stat(seed: &mut u64) -> TrainingStat {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    TrainingStat::from_index((*seed >> 33) as u32)
}

fn roll_event(seed: &mut u64) -> Option<TrainingStat> {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let roll = ((*seed >> 33) % 100) as u32;
    if roll < 90 { Some(rand_stat(seed)) } else { None }
}

// ============================================================
// Systèmes boutons — navigation générale
// ============================================================
fn button_click_system(
    mut query: Query<(
        &Interaction, &mut BackgroundColor,
        Option<&StartButton>, Option<&UmaListButton>,
        Option<&BackButton>, Option<&CareerButton>,
    ), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, start, uma_list, back, career_btn) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                color.0 = color.0.with_alpha(0.5);
                if start.is_some()       { next_state.set(GameState::MainMenu); }
                else if uma_list.is_some()  { next_state.set(GameState::UmaList); }
                else if back.is_some()   { next_state.set(GameState::MainMenu); }
                else if career_btn.is_some() { next_state.set(GameState::UmaList); }
            }
            Interaction::Hovered => { color.0 = color.0.with_alpha(0.8); }
            Interaction::None    => { color.0 = color.0.with_alpha(1.0); }
        }
    }
}

// ============================================================
// Systèmes boutons — carrière
// ============================================================
fn uma_list_click_system(
    query: Query<(&Interaction, &UmaListItem), (Changed<Interaction>, With<Button>)>,
    uma_query: Query<(&Uma, &UmaStats)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    for (interaction, item) in &query {
        if *interaction == Interaction::Pressed {
            if let Ok((uma, stats)) = uma_query.get(item.uma_entity) {
                let seed = (time.elapsed_secs_f64() * 1_000_000.0) as u64;
                commands.insert_resource(RngSeed(seed));
                commands.insert_resource(CareerState::new(&uma.name, stats.clone()));
                next_state.set(GameState::Career);
            }
        }
    }
}

fn career_training_system(
    query: Query<(
        &Interaction,
        Option<&TrainSpeedButton>, Option<&TrainStaminaButton>,
        Option<&TrainPowerButton>, Option<&TrainGutsButton>, Option<&TrainWitButton>,
    ), (Changed<Interaction>, With<Button>)>,
    mut career: ResMut<CareerState>,
    mut rng: ResMut<RngSeed>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, sp, st, po, gu, wi) in &query {
        if *interaction != Interaction::Pressed { continue; }
        let stat =
            if sp.is_some() { TrainingStat::Speed }
            else if st.is_some() { TrainingStat::Stamina }
            else if po.is_some() { TrainingStat::Power }
            else if gu.is_some() { TrainingStat::Guts }
            else if wi.is_some() { TrainingStat::Wit }
            else { continue };
        career.stats.apply_training(stat);
        end_of_turn(&mut career, &mut rng, &mut next_state);
    }
}

fn career_race_system(
    query: Query<&Interaction, (Changed<Interaction>, With<RaceButton>)>,
    mut career: ResMut<CareerState>,
    mut rng: ResMut<RngSeed>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction != Interaction::Pressed { continue; }
        career.wins += 1;
        let bonus = rand_stat(&mut rng.0);
        career.stats.apply_training(bonus);
        career.last_race_message = Some(format!(
            "{} remporte la course !\n+5 {} en recompense.",
            career.uma_name, bonus.label()
        ));
        next_state.set(GameState::CareerRaceResult);
    }
}

fn career_race_result_continue_system(
    query: Query<&Interaction, (Changed<Interaction>, With<RaceResultContinueButton>)>,
    mut career: ResMut<CareerState>,
    mut rng: ResMut<RngSeed>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction != Interaction::Pressed { continue; }
        career.last_race_message = None;
        end_of_turn(&mut career, &mut rng, &mut next_state);
    }
}

fn career_event_continue_system(
    query: Query<&Interaction, (Changed<Interaction>, With<EventContinueButton>)>,
    mut career: ResMut<CareerState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction != Interaction::Pressed { continue; }
        if let Some(stat) = career.pending_event.take() {
            career.stats.apply_event_bonus(stat);
        }
        advance_turn(&mut career, &mut next_state);
    }
}

fn career_end_system(
    query: Query<&Interaction, (Changed<Interaction>, With<EndCareerButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}

// ============================================================
// Helpers fin de tour
// ============================================================
fn end_of_turn(
    career: &mut CareerState,
    rng: &mut RngSeed,
    next_state: &mut NextState<GameState>,
) {
    if let Some(stat) = roll_event(&mut rng.0) {
        career.pending_event = Some(stat);
        next_state.set(GameState::CareerEvent);
    } else {
        advance_turn(career, next_state);
    }
}

fn advance_turn(career: &mut CareerState, next_state: &mut NextState<GameState>) {
    if career.turn >= career.max_turns {
        next_state.set(GameState::CareerEnd);
    } else {
        career.turn += 1;
        next_state.set(GameState::Career);
    }
}

// ============================================================
// Nettoyage des écrans
// ============================================================
// Un système de cleanup générique paramétré par le marqueur d'écran
fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query { commands.entity(e).despawn(); }
}

// ============================================================
// Main
// ============================================================
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(UmaAssets::new())
        .insert_resource(RngSeed(12345))
        .insert_resource(CareerState::default())
        .add_systems(Startup, (setup_camera, load_uma_assets, setup_launch_screen))
        .add_systems(OnEnter(GameState::MainMenu),         setup_main_menu)
        .add_systems(OnEnter(GameState::UmaList),          setup_uma_list)
        .add_systems(OnEnter(GameState::Career),           setup_career)
        .add_systems(OnEnter(GameState::CareerEvent),      setup_career_event)
        .add_systems(OnEnter(GameState::CareerRaceResult), setup_career_race_result)
        .add_systems(OnEnter(GameState::CareerEnd),        setup_career_end)
        .add_systems(OnExit(GameState::LaunchScreen),    cleanup::<OnLaunchScreen>)
        .add_systems(OnExit(GameState::MainMenu),         cleanup::<OnMainMenu>)
        .add_systems(OnExit(GameState::UmaList),          cleanup::<OnUmaList>)
        .add_systems(OnExit(GameState::Career),           cleanup::<OnCareer>)
        .add_systems(OnExit(GameState::CareerEvent),      cleanup::<OnCareerEvent>)
        .add_systems(OnExit(GameState::CareerRaceResult), cleanup::<OnCareerRaceResult>)
        .add_systems(OnExit(GameState::CareerEnd),        cleanup::<OnCareerEnd>)
        .add_systems(Update, (
            button_click_system,
            uma_list_click_system,
            career_training_system,
            career_race_system,
            career_race_result_continue_system,
            career_event_continue_system,
            career_end_system,
        ))
        .run();
}