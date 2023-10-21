use hashbrown::HashMap;

use crate::{Schedule, SystemType, System, Resources, Entity, Commands};

pub struct ScheduleGraph {
    nodes: HashMap<String, ScheduleNode>,
}

impl ScheduleGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new()
        }
    }

    pub fn add_system(&mut self, name: String, schedule: SystemType, system: Box<dyn System>){
        if let Some(schedule_node) = self.nodes.get_mut(&name){
            schedule_node.add_system(schedule, system)
        } else {
            let mut schedule_node = ScheduleNode::new();
            schedule_node.add_system(schedule, system);
            
            self.nodes.insert(name, schedule_node);
        }
    }

    pub fn run(&mut self, name: String, entity: &mut Entity, commands: &mut Commands, resources: &mut Resources){
        if let Some(schedule_node) = self.nodes.get_mut(&name){
            schedule_node.run(entity, commands, resources);
        }
    }
}

pub struct ScheduleNode{
    frame: usize,
    setup: Schedule, 
    update: Schedule
}


impl ScheduleNode{
    pub fn new() -> Self{
        Self { 
            frame: 0, 
            setup: Schedule::new(), 
            update: Schedule::new() 
        }
    }

    pub fn add_system(&mut self, schedule: SystemType, system: Box<dyn System>){
        match schedule{
            SystemType::Update => self.update.add_system(system),
            SystemType::Setup => self.setup.add_system(system),
        }
    }

    pub fn run(&mut self, entity: &mut Entity, commands: &mut Commands, resources: &mut Resources){
        if self.frame == 0{
            self.setup.run(entity, resources, commands)
        } else {
            self.update.run(entity, resources, commands);
        }
        self.frame += 1;
    }
}