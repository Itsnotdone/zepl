use wgpu::BindGroup;

pub struct BindGroupStorage{
    groups: Vec<BindGroup>
}

impl BindGroupStorage{
    pub fn new() -> Self{
        Self { groups: Vec::new() }
    }

    pub fn push(&mut self, bindgroup: BindGroup) -> u32{
        self.groups.push(bindgroup);
        (self.groups.len() - 1) as u32
    }

    pub fn get(&self, id: u32) -> &BindGroup{
        &self.groups[id as usize]
    }
}