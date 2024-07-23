pub struct Eat {
    pub id: u32,
    pub name: String,
    pub food_value: f32,
}

pub struct Inventory {
    pub slot: Vec<Eat>,
}

impl Inventory {
    pub fn get_inventory() -> Self {
        Self{
            slot: Vec::<Eat>::new(),
        }
    }

    pub fn fill_inventory(&mut self, amount: u32) {
        for x in 0..amount {
            self.slot.push(Eat {
                id: x,
                name: "CatEat".to_string(),
                food_value: 2.0,
            })
        }
    }
}