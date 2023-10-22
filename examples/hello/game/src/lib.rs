use zepl::*;

#[service]
pub fn player(commands: &mut EntityCommands){
    commands.add_system(Setup, setup_player);
    commands.add_system(Update, update_player);
}

pub fn setup_player(base: &mut Entity, commands: &mut Commands, resources: &mut Resources){
    
}

pub fn update_player(base: &mut Entity, commands: &mut Commands, resources: &mut Resources){
    let input = resources.get::<InputEvent>().unwrap();

    if input.is_pressed(Key::A){
        println!("a pressed");
    }

    if input.is_just_pressed(Key::D){
        println!("d just pressed");
    }

    if input.is_just_released(Key::W){
        println!("w just released");
    }
}


#[main]
pub fn main(commands: &mut Commands, resources: &mut Resources){
    let scene_loader = resources.get::<SceneLoader>().unwrap();
    commands.change_scene(load_scene!(scene_loader, "scenes/main.scene"));
}

