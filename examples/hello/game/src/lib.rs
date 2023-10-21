use zepl::*;

#[service]
pub fn player(commands: &mut EntityCommands){
    commands.add_system(Setup, setup_player);
    commands.add_system(Update, update_player);
}

pub fn setup_player(base: &mut Entity, commands: &mut Commands, resources: &mut Resources){
    println!("setup player");
}

pub fn update_player(base: &mut Entity, commands: &mut Commands, resources: &mut Resources){
    println!("update player");
}


#[main]
pub fn main(commands: &mut Commands, resources: &mut Resources){
    let scene_loader = resources.get::<SceneLoader>().unwrap();
    commands.change_scene(load_scene!(scene_loader, "scenes/main.scene"));
}

