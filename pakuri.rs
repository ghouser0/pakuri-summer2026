// pakuri struct



struct Pakuri {
    let mut name: String;
    let mut species: String;
    let mut level: i32;
    let mut hp: i32;
    let mut cp: i32;


    pub new(name: String, species: String) -> Self {
        // idk girl !
    }

    pub with(name: String, species: String, level: i32) -> Self {
        // idk what to return here
    }

    pub name(&self) -> &String {
        return name;
    }

    pub species(&self) -> &String {
        return species;
    }

    pub hp(&self) -> i32 {
        // calculate hp
        return hp;
    }

    pub cp(&self) -> i32 {
        // calculate cp
        return cp;
    }

    pub level(&self) -> i32 {
        return level;
    }

    pub set_level(&self) -> i32 {
        // set level field
        return level;
    }
}