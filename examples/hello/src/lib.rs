set_debug_dir!("target");

use zepl::*;
  
#[system(Setup)]
pub fn setup_player(world: &mut World, resources: &mut Resources) {
    let handle = resources.add_resource(Texture::from_bytes(include_bytes!("../assets/fred.png")));

    let base = world.iter_mut().next().unwrap();

    base.add_component(Sprite::new(handle));
}

#[system(Update)]
pub fn update_player(world: &mut World, resources: &mut Resources) {
    let input = resources.get::<InputEvent>().unwrap();
    if input.is_just_pressed(Key::P){
        let handle = resources.add_resource(Texture::from_bytes(include_bytes!("../assets/mushroom.jpg")));

        for base in world.iter_mut(){
            if let Some(sprite) = base.get_mut::<Sprite>(){
                sprite.texture = handle.clone();
            }
        }
        
    }
    for base in world.iter_mut(){
        let input = resources.get::<InputEvent>().unwrap();
        if input.is_pressed(Key::A) {
            base.get_mut::<Transform2d>().unwrap().position.x -= 2.0;
        } 

        if input.is_pressed(Key::D) {
            base.get_mut::<Transform2d>().unwrap().position.x += 2.0;
        }

        if input.is_pressed(Key::W) {
            base.get_mut::<Transform2d>().unwrap().position.y += 2.0;
        }

        if input.is_pressed(Key::S) {
            base.get_mut::<Transform2d>().unwrap().position.y -= 2.0;
        }

        if input.is_pressed(Key::R) {
            base.get_mut::<Transform2d>().unwrap().rotation -= 5.0;
        }
    }
    
}

#[main]
pub fn main(commands: &mut Commands, resources: &mut Resources) {
    let scene_loader = resources.get::<SceneLoader>().unwrap();
    commands.change_scene(load_scene!(scene_loader, "scenes/main.scene"));
} 
