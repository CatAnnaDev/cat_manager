use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use chrono::{Datelike, DateTime, Duration, Local, NaiveDate};
use rand::{random, Rng, thread_rng};
use crate::{bool_state};
use crate::cat::Gender::{Female, Male};
use crate::cat_name::{GENDER_FEMALE, GENDER_MALE};
use crate::color::ColorType;
use crate::race::Race;

pub fn get_cat_image() -> String {
    let mut tmp = vec![];
    let path = PathBuf::from("cat pngs");
    for x in fs::read_dir(&path).unwrap() {
        let entry = x.ok();
        let file_path = entry.unwrap().path();

        if file_path.is_file(){
            tmp.push(file_path);
        }
    }
    tmp.get(thread_rng().gen_range(0..tmp.len())).unwrap().to_str().unwrap().to_string()
}


#[derive(Eq, Default, PartialEq, Clone)]
pub enum Gender {
    #[default]
    Female,
    Male,
}

impl Display for Gender{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Female => "Female",
            Male => "Male"
        })
    }
}

impl Gender {
    pub fn get_random_gender() -> Gender {
        if random() { Female } else { Male }
    }

    pub fn get_random_name_and_gender() -> (&'static str, Gender) {
        let gender = Self::get_random_gender();
        match gender {
            Male => {(GENDER_MALE[thread_rng().gen_range(0..GENDER_MALE.len())], gender)}
            Female => {(GENDER_FEMALE[thread_rng().gen_range(0..GENDER_FEMALE.len())], gender)}
        }
    }
}

#[derive(Clone)]
pub struct CatInfo {
    pub cat_image_byte: String,
    pub arrived_date: NaiveDate,
    pub bd_date: NaiveDate,
    pub name: &'static str,
    pub age: u8,
    pub color_type: ColorType,
    pub race: Race,
    pub weight: f32,
    pub sleep: bool,
    pub health: f32,
    pub food: f32,
    pub gender: Gender,
    pub last_updated: DateTime<Local>,
}

fn generate_random_date_in_range(start_date: NaiveDate, end_date: NaiveDate) -> NaiveDate {
    let mut rng = thread_rng();
    let days_range = (end_date - start_date).num_days();
    let random_days = rng.gen_range(0..=days_range);
    start_date + Duration::days(random_days)
}

fn generate_dates() -> (NaiveDate, NaiveDate) {
    let today = Local::now().naive_utc().date();
    let earliest_birth_date = NaiveDate::from_ymd_opt(2010, 1, 1).unwrap();
    let earliest_arrival_date = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();

    let birth_date = generate_random_date_in_range(earliest_birth_date, today);

    let arrival_start_date = std::cmp::max(earliest_arrival_date, birth_date);
    let arrival_date = generate_random_date_in_range(arrival_start_date, today);

    (birth_date, arrival_date)
}

fn calculate_age(birth_date: NaiveDate, current_date: NaiveDate) -> u8 {
    let mut age = current_date.year() - birth_date.year();
    if (current_date.month() < birth_date.month()) || (current_date.month() == birth_date.month() && current_date.day() < birth_date.day()) {
        age -= 1;
    }
    age as u8
}

impl CatInfo {

    pub(crate) fn new_cat() -> Self{
        let (name, gender) = Gender::get_random_name_and_gender();
        let (birth_date, arrival_date) = generate_dates();
        Self{
            cat_image_byte: get_cat_image(),
            arrived_date: arrival_date,
            bd_date: birth_date,
            name,
            age: calculate_age(birth_date, Local::now().naive_utc().date()),
            color_type: random(),
            race: random(),
            weight: thread_rng().gen_range(0.5..7.0),
            sleep: false,
            health: 100.0,
            food: 100.0,
            gender,
            last_updated: Local::now(),
        }
    }

    pub(crate) fn spawn_new_cat(nb_cat: u8) -> Vec<Self> {
        let mut cat_vec = Vec::new();

        for _ in 0..nb_cat {
            let color: ColorType = random();
            let race: Race = random();
            let sleep = random();
            let health = thread_rng().gen_range(10.0..100.0);
            let (name, gender) = Gender::get_random_name_and_gender();
            let (birth_date, arrival_date) = generate_dates();
            cat_vec.push(CatInfo {
                cat_image_byte: get_cat_image(),
                arrived_date: arrival_date,
                bd_date: birth_date,
                name,
                age: calculate_age(birth_date, Local::now().naive_utc().date()),
                color_type: color,
                race,
                weight: thread_rng().gen_range(1.5..7.0),
                sleep,
                health,
                food: 100.0,
                gender,
                last_updated: Local::now(),
            });
        }
        cat_vec
    }

    pub(crate) fn feed(&mut self, weight: f32, health: f32) -> String {
        if !self.sleep {
            self.weight += weight; // 0.1
            self.health += health; // 5
            self.food += weight;  // Augmente la nourriture
            format!("{} a été nourri. Nouveau poids: {:.1} kg, Santé: {}, Nourriture: {:.1}", self.name, self.weight, self.health, self.food)
        }else {
            format!("{} dort et ne peut pas manger.", self.name)
        }
    }

    pub(crate) fn play(&mut self, weight: f32, health: f32) -> String {
        if !self.sleep {
            self.weight -= weight; // 0.05
            self.health += health; // 2
            format!("{} a joué. Nouveau poids: {:.1} kg, Santé: {}", self.name, self.weight, self.health)
        } else {
            format!("{} dort et ne peut pas jouer.", self.name)
        }
    }

    pub(crate) fn toggle_sleep(&mut self, health: f32) -> String {
        self.sleep = !self.sleep;
        if self.sleep {
            self.health += health; // 10
            format!("{} fait maintenant dodo. Santé: {}", self.name, self.health)
        } else {
            format!("{} est maintenant réveillé.", self.name)
        }
    }

    pub(crate) fn mate(&self, other: &Self) -> Result<Self, String> {
        let mut tmp = String::new();

        if self.gender.eq(&other.gender) || self.sleep || other.sleep {
            tmp.push_str(&*format!("\nCan't mate {} with {}\nbecause:", self.name, other.name));
            if self.gender.eq(&other.gender) { tmp.push_str("\n- Same Sexe"); }
            if self.sleep { tmp.push_str(&*format!("\n- {} sleep", self.name)); }
            if other.sleep { tmp.push_str(&*format!("\n- {} sleep", other.name)); }
            return Err(tmp);
        }

        //let name = format!("{}{}", &self.name[0..self.name.len() / 2], &other.name[other.name.len() / 2..]);
        let (name, gender) = Gender::get_random_name_and_gender();
        let color = if random() { self.color_type } else { other.color_type };
        let race = if random() { self.race } else { other.race };

        Ok(CatInfo {
            cat_image_byte: get_cat_image(),
            arrived_date: Local::now().naive_utc().date(),
            bd_date: Local::now().naive_utc().date(),
            name,
            age: 1,
            color_type: color,
            race,
            weight: 1.0,
            sleep: false,
            health: 100.0,
            food: 100.0,
            gender,
            last_updated: Local::now(),
        })
    }

    pub(crate) fn update(&mut self) -> Option<()> {

        if self.age >= 20{
            return  None
        }

        let now = Local::now();
        let duration = now.signed_duration_since(self.last_updated);

        if duration.num_minutes() >= 5 {
            self.age += (duration.num_minutes() / 5) as u8;
            self.health = (self.health - 5.0 * (duration.num_minutes() / 5) as f32).max(0.0);
            self.last_updated = now;
            println!("{} a vieilli. Nouvel âge: {}, Santé: {}", self.name, self.age, self.health);
        }

        if self.age >= 20{
            return  None
        }

        if self.food > 0.0 {
            self.food -= 0.5;
        } else {
            self.health = (self.health - 10.0).max(0.0);
        }

        if self.health > 0.0 {
            Some(self.health = (self.health - 2.0).max(0.0))
        }else {
            None
        }
    }

    pub(crate) fn minimal_info(&self)  -> String{
        format!("Name: {}\n- Genre: {}\n- Age: {}\n- Sleep: {}\n- Health: {:.2}\n- Food: {:.2}",
                self.name, self.gender, self.age, bool_state!("YES", "NO", self.sleep), self.health, self.food)
    }

}

impl Display for CatInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\n- Age: {} an(s)\n- Color: {}\n- Race: {}\n- Weight: {:.2} kg\n- Sleep: {}\n- Health: {:.2}\n- Food: {:.2}\n- Sexe: {}\n- Entrance: {}\n- Bd: {}",
            self.name, self.age, self.color_type, self.race, self.weight, bool_state!("YES", "NO", self.sleep), self.health, self.food, self.gender, self.arrived_date, self.bd_date,
        )
    }
}
