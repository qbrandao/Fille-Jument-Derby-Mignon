use bevy::prelude::*;

// --- États du jeu ---
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    LaunchScreen,  // Écran de lancement
    MainMenu,      // Menu principal
    UmaList,       // Liste des Uma
}

#[derive(Component)]
struct Uma {
    name: String,
    // Ajoute d'autres propriétés ici (ex: vitesse, points de vie, sprite, etc.)
    sprite: Handle<Image>, // Pour afficher une image
}

#[derive(Component)]
struct UmaName(String);



// --- Composants pour les boutons ---
#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct CareerButton;

#[derive(Component)]
struct UmaListButton;

#[derive(Component)]
struct BackButton; // Pour revenir en arrière

// --- Ressource pour stocker les Uma ---
#[derive(Resource)]
struct UmaAssets {
    uma_sprites: Vec<Handle<Image>>, // Stocke les handles des sprites des Uma
}

impl UmaAssets {
    fn new() -> Self {
        Self {
            uma_sprites: Vec::new(),
        }
    }
}

// --- Style des boutons ---
fn button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

// --- Setup de l'écran de lancement ---
fn setup_launch_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::DARK_GREEN),
            ..default()
        },
        OnLaunchScreen,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Bienvenue dans Fille-Jument Derby Mignon",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            OnLaunchScreen,
        ));
        parent.spawn((
            ButtonBundle {
                style: button_style(),
                background_color: BackgroundColor(Color::GOLD),
                ..default()
            },
            StartButton,
            OnLaunchScreen,
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Commencer à jouer",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                OnLaunchScreen,
            ));
        });
    });
}

// --- Setup du menu principal ---
fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        },
        OnMainMenu,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Menu Principal",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            OnMainMenu,
        ));
        parent.spawn((
            ButtonBundle {
                style: button_style(),
                background_color: BackgroundColor(Color::GOLD),
                ..default()
            },
            CareerButton,
            OnMainMenu,
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Carrière",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                OnMainMenu,
            ));
        });
        parent.spawn((
            ButtonBundle {
                style: button_style(),
                background_color: BackgroundColor(Color::GOLD),
                ..default()
            },
            UmaListButton,
            OnMainMenu,
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Fille Jument",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                OnMainMenu,
            ));
        });
    });
}
// --- Setup de la liste des Uma ---
fn setup_uma_list(
    mut commands: Commands,
    uma_assets: Res<UmaAssets>,
) {
    // Crée un conteneur pour la liste
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::PURPLE),
            ..default()
        },
        OnUmaList,
    )).with_children(|parent| {
        // Titre
        parent.spawn((
            TextBundle::from_section(
                "Liste des Uma",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            OnUmaList,
        ));

        // Bouton "Retour"
        parent.spawn((
            ButtonBundle {
                style: button_style(),
                background_color: BackgroundColor(Color::GOLD),
                ..default()
            },
            BackButton,
            OnUmaList,
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Retour",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                OnUmaList,
            ));
        });

        // Liste des Uma (avec sprites et noms)
        let uma_data = vec![
            ("Dariz", 0),
            ("Calandagan", 1),
            ("Aventure", 2),
            ("Sosie", 3),
            ("Quisisana", 4),
            ("Gezora", 5),
        ];

        for (name, sprite_index) in uma_data {
            // Crée une entité Uma
            let uma_entity = spawn_uma(
                &mut commands,
                &uma_assets,
                name,
                Vec3::new(0.0, 0.0, 0.0), // Position (à ajuster)
                sprite_index,
            );

            // Affiche le nom de l'Uma dans la liste (UI)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..button_style()
                    },
                    background_color: BackgroundColor(Color::LIGHT_GREEN),
                    ..default()
                },
                UmaListItem { uma_entity }, // Composant pour lier le bouton à l'entité Uma
                OnUmaList,
            )).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        name,
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    OnUmaList,
                ));
            });
        }
    });
}

// --- Composant pour lier un bouton de la liste à une entité Uma ---
#[derive(Component)]
struct UmaListItem {
    uma_entity: Entity,
}

// --- Système pour gérer les clics sur les Uma dans la liste ---
fn uma_list_click_system(
    mut interaction_query: Query<(&Interaction, &UmaListItem), (Changed<Interaction>, With<Button>)>,
    uma_query: Query<&Uma>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, uma_list_item) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Récupère l'entité Uma associée
            if let Ok(uma) = uma_query.get(uma_list_item.uma_entity) {
                println!("Uma sélectionnée : {}", uma.name);
                // Ici, tu peux :
                // - Charger un écran de jeu avec cette Uma
                // - Afficher ses stats
                // - etc.
                // Exemple : Passer à un état "Gameplay" avec cette Uma
                // next_state.set(GameState::Gameplay);
            }
        }
    }
}
// --- Système pour charger les sprites des Uma ---
fn load_uma_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Exemple : charge des sprites pour chaque Uma
    // (Remplace par tes propres chemins de fichiers)
    let uma_sprites = vec![
        asset_server.load("uma/dariz.png"),
        asset_server.load("uma/calandagan.png"),
        asset_server.load("uma/aventure.png"),
        asset_server.load("uma/sosie.png"),
        asset_server.load("uma/quisisana.png"),
        asset_server.load("uma/gezora.png"),
    ];

    commands.insert_resource(UmaAssets { uma_sprites });
}

// --- Spawn une Uma avec ses composants ---
fn spawn_uma(
    commands: &mut Commands,
    uma_assets: &UmaAssets,
    name: &str,
    position: Vec3,
    sprite_index: usize,
) -> Entity {
    commands.spawn((
        Uma {
            name: name.to_string(),
            sprite: uma_assets.uma_sprites[sprite_index].clone(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            texture: uma_assets.uma_sprites[sprite_index].clone(),
            transform: Transform::from_translation(position),
            ..default()
        },
        UmaName(name.to_string()),
    )).id()
}

// --- Composants pour identifier les écrans ---
#[derive(Component)]
struct OnLaunchScreen;

#[derive(Component)]
struct OnMainMenu;

#[derive(Component)]
struct OnUmaList;

// --- Système pour gérer les clics sur les boutons ---
fn button_click_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, AnyOf<[StartButton, CareerButton, UmaListButton, BackButton]>), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::GOLD.with_a(0.5));
                if button_type.is::<StartButton>() {
                    next_state.set(GameState::MainMenu);
                } else if button_type.is::<UmaListButton>() {
                    next_state.set(GameState::UmaList);
                } else if button_type.is::<BackButton>() {
                    // Retour à l'écran précédent
                    match next_state.get() {
                        GameState::UmaList => next_state.set(GameState::MainMenu),
                        GameState::MainMenu => next_state.set(GameState::LaunchScreen),
                        _ => next_state.set(GameState::LaunchScreen),
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GOLD.with_a(0.8));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::GOLD);
            }
        }
    }
}
// --- Système pour nettoyer les écrans ---
fn cleanup_screen(
    mut commands: Commands,
    current_state: Res<State<GameState>>,
    launch_screen_query: Query<Entity, With<OnLaunchScreen>>,
    main_menu_query: Query<Entity, With<OnMainMenu>>,
    uma_list_query: Query<Entity, With<OnUmaList>>,
) {
    match current_state.get() {
        GameState::LaunchScreen => {
            for entity in &main_menu_query {
                commands.entity(entity).despawn_recursive();
            }
            for entity in &uma_list_query {
                commands.entity(entity).despawn_recursive();
            }
        }
        GameState::MainMenu => {
            for entity in &launch_screen_query {
                commands.entity(entity).despawn_recursive();
            }
            for entity in &uma_list_query {
                commands.entity(entity).despawn_recursive();
            }
        }
        GameState::UmaList => {
            for entity in &launch_screen_query {
                commands.entity(entity).despawn_recursive();
            }
            for entity in &main_menu_query {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// --- Configuration de l'application ---
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(UmaAssets::new())
        .add_systems(Startup, (load_uma_assets, setup_launch_screen))
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnEnter(GameState::UmaList), setup_uma_list)
        .add_systems(Update, (button_click_system,uma_list_click_system))
        .add_systems(Update, cleanup_screen)
        .run();
}

