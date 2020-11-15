extern crate rand;

/** The main method of the wusel world. */
fn main() {

    let mut world: World = World::new(100, 100);

    println!("Created a new world: w:{w}, h:{h}", w = world.width, h = world.height);

    let mut w0 = Wusel::xx(String::from("1st"));
    let mut w1 = Wusel::xx(String::from("2nd"));
    let mut w2 = Wusel::xy(String::from("3rd"));

    println!("{} at {}\n", w0.name, w0.show_pos());

    println!("\n{} at {}\n", w0.name, w0.show_pos());

    w0.go(Direction::N); // 0,1
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 0); assert_eq!(w0.body.pos_y, 1);

    w0.go(Direction::E); // 1,1
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 1);

    w0.go(Direction::S); // 1,0
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 0);

    w0.go(Direction::W); // 0,0
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 0); assert_eq!(w0.body.pos_y, 0);

    w0.go(Direction::NE); // 1,1
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 1);

    w0.go(Direction::SE); // 2,0
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 2); assert_eq!(w0.body.pos_y, 0);

    w0.go(Direction::NW); // 1,1
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 1);

    w0.go(Direction::SW); // 0,0
    println!("{} at {}", w0.name, w0.show_pos());
    assert_eq!(w0.body.pos_x, 0); assert_eq!(w0.body.pos_y, 0);

    println!();

    println!("Wusel, {}, at {}", w0.name, w0.show_pos());
    println!("Wusel, {}, at {}", w1.name, w0.show_pos());
    println!("Wusel, {}, at {}", w2.name, w0.show_pos());

    w0.chat(&mut w1, true); // friendly good
    w0.flirt(&mut w2, true); // romantically good

    println!();

    w0.show_abilities();
    w0.show_overview();
    // w1.show_relations();
    // w2.show_relations();

    w0.mate_with(&mut w2, true); // romantically good

    println!();

    println!("{} is pregnant: {}", w0.name, w0.is_pregnant());
    println!("{} is pregnant: {}", w2.name, w2.is_pregnant());

    w0.show_abilities();
    // w1.show_relations();
    // w2.show_relations();

    for n in &Need::VALUES {
        w0.add_need(*n, 1);
    }
    w0.show_overview();

    let mut sushi = Food::new(String::from("Sushi"), 2, 1, 5, 2);
    println!("Food: {}", sushi.show());

    sushi.body.place_at(0, 1);

    w0.eat(&mut sushi);
    w0.show_overview();

    println!("Food: {}", sushi.show());

    w0.eat(&mut sushi);
    w0.show_overview();

    println!("Food: {}", sushi.show());

    w0.eat(&mut sushi);
    w0.show_overview();

    println!("Food: {}", sushi.show());

    world.tick();

    w0.tick();
    w1.tick();
    w1.tick();

    w1.improve(Ability::COOKING);
    w1.improve(Ability::COMMUNICATION);
    w1.improve(Ability::FITNESS);
    w1.improve(Ability::FINESSE);

    w0.show_overview();

}

static WORLD_HEIGHT: u32 = 100;
static WORLD_WIDTH: u32 = 100;

struct World {
    height: u32,
    width: u32,

    clock: usize, // time of the world.

    things: Vec<Thing>, // vector of things ?

    wusels: Vec<Wusel>, // vector of wusels?
}

impl World {
    fn new(height: u32, width: u32) -> Self {
        return Self{
            height: height, width: width,
            clock: 0,
            things: vec![],
            wusels: vec![],
        };
    }

    const TICKS_A_DAY: usize = 2880; // 24h by 0.5 minutes

    /** Increase clock and proceed decay of all things and relations. */
    fn tick(self: &mut Self) {

        self.clock += 1;

        /* A new day is over: Forward the day structure to the world. */
        let new_day: bool = self.clock % Self::TICKS_A_DAY == 0;

        /* Decay on every object and living. */
        for i in 0 .. self.wusels.len() {
            let w = &mut self.wusels[i];
            w.tick();
            if new_day { w.add_new_day() }
        }

        for _ in &self.things { /* decay of things over time. */  }
    }
}

/** Direction in the world. */
#[derive(Debug)]
enum Direction {
    NW, N, NE,
    W,      E,
    SW, S, SE,
}

struct Thing {
    // position
    pos_x: u32, // unsigned int 32
    pos_y: u32, // unsigned int 32

}

impl Thing {
    fn new(x: u32, y: u32) -> Self {
        Self { pos_x: x, pos_y: y }
    }

    /** Show position of npc. */
    fn show_pos(self: &Self) -> String {
        return format!("(x: {} | y: {})", self.pos_x, self.pos_y);
    }

    /** Move the thing in certain direction. */
    fn go(self: &mut Self, direction: Direction) {
        /* Mutable variable. Will contain the possible new position. */
        let mut _x: i64 = self.pos_x as i64;
        let mut _y: i64 = self.pos_y as i64;

        /* Determine changes. */
        match direction {
            Direction::N =>  { _x +=  0; _y +=  1},
            Direction::NE => { _x +=  1; _y +=  1},
            Direction::NW => { _x += -1; _y +=  1},
            Direction::S =>  { _x +=  0; _y += -1},
            Direction::SE => { _x +=  1; _y += -1},
            Direction::SW => { _x += -1; _y += -1},
            Direction::E =>  { _x +=  1; _y +=  0},
            Direction::W =>  { _x += -1; _y +=  0},
        }

        if _x > (WORLD_HEIGHT as i64) {
            println!("Already at Top");
        } else if _x < 0 {
            println!("Already at Bottom");
        } else {
            /* Apply to real position. */
            self.pos_x = _x as u32;
        }

        if _y > (WORLD_WIDTH as i64) {
            println!("Already at East Border");
        } else if _y < 0 {
            println!("Already at West Border");
        } else {
            /* Apply to real position. */
            self.pos_y = _y as u32;
        }
    }

    /** Place the thing on a new Position. */
    fn place_at(self: &mut Self, new_x: u32, new_y: u32) {
        self.pos_x = if new_x <= WORLD_HEIGHT { new_x } else { WORLD_HEIGHT };
        self.pos_y = if new_y <= WORLD_WIDTH { new_y } else { WORLD_WIDTH };
    }

}

/** Something a Wusel can consume. */
struct Food {
    name: String,

    body: Thing,

    available: f32, // 1.0f whole, 0.0f gone.
    bites: u32, // eats an xth part per minute of this food. => partition_per_bite

    // per bite.
    needed_energy: u32, // needed energy to eat; eg. eating a cup of Water or eating raw meat
    satisfied_food: u32, // satiesfies hunger need.
    satisfied_water: u32, // satiesfies water need.
}

impl Food {
    fn new(name: String,
           bites: u32,
           energy_per_bite: u32,
           food_per_bite: u32,
           water_per_bite: u32
    ) -> Self {
        Self {
            name: name,

            body: Thing::new(0, 0), // start at center.

            available: 1.0f32, // start fully.
            bites: bites, // => 0.5 per minute

            needed_energy: energy_per_bite,
            satisfied_food: food_per_bite,
            satisfied_water: water_per_bite,
        }
    }

    fn show(self: &Self) -> String {
        return if self.available <= 0f32 {
            format!("Eaten {} (fully gone)", self.name)
        } else if self.available >= 0.999999 {
            format!("{}", self.name)
        } else {
            format!("{} {}", self.available, self.name)
        }
    }

    /** Get the size of one bite. */
    fn bite_part(self: &Self) -> f32 {
        1.0 / self.bites as f32
    }

    /** Take a bite: Decrease availability by one bite.
     * @return true, if the bite was successful. */
    fn take_a_bite(self: &mut Self) -> bool {
        if self.available > 0.0 {
            self.available -= self.bite_part(); // remove a part.
            true
        } else {
            false
        }
    }
}

/** Pregancy: Not pregnant or pregnant with unfinished Wusel. */
enum Pregnant {
    // an optional pregnancy.
    YES(u8), // dayes until arrival.
    NO,
}

/** Life state of a Wusel.
 * All but alive leads to a not living state, though a ghost may wander and interact. */
#[derive(Copy, Clone, PartialEq)]
enum Life {
    ALIVE,
    #[allow(unused)]
    DEAD,
    #[allow(unused)]
    GHOST,
}

/** A need, the wusel needs to satisfy to survive. */
#[derive(Copy, Clone, PartialEq)]
enum Need {
    WATER, FOOD, WARMTH, SLEEP, HEALTH, LOVE, FUN,
}

impl Need {
    /** Custom iteratable values. */
    const VALUES: [Self; 7] = [
        Self::WATER,  Self::FOOD, Self::WARMTH, Self::SLEEP,
        Self::HEALTH, Self::LOVE, Self::FUN];

    const DEFAULT_NEED_DECAY_PER_MINUTE: [u32; 7] = [
        1, 1, 1, 1, 0/*health*/, 1, 1,
        ];

    fn name(&self) -> &str {
        return match self {
            Self::WATER => "water", Self::FOOD   => "food"  ,
            Self::WARMTH => "warmth", Self::SLEEP => "sleep",
            Self::HEALTH => "health", Self::LOVE   => "love",
            Self::FUN   => "fun",
        }
    }

    fn get_default_decay(&self) -> u32 {
        for i in 0 .. Self::VALUES.len() {
            if self == &Self::VALUES[i] {
                return Self::DEFAULT_NEED_DECAY_PER_MINUTE[i];
            }
        }
        return 0; // default: no decay.
    }
}

/** An ability, the wusel can learn to improve their lifestyle. */
#[derive(Copy, Clone, PartialEq)]
enum Ability {
    COOKING,
    COMMUNICATION,
    FITNESS,
    FINESSE,
}

impl Ability {
    fn name(&self) -> &str {
        return match self {
            Self::COOKING => "cooking",
            Self::COMMUNICATION => "communication",
            Self::FITNESS => "fitness",
            Self::FINESSE => "finesse",
        }
    }
}

/** Wusel.
 * Bundle of information on a certain position and abilities.
 * Abilities: Luck, Strength, Intelligence, Communication.
 */
struct Wusel {
    id: usize,

    // name
    name: String,

    body: Thing,

    female: bool, // female => able to bear children, male => able to inject children
    pregnant: Pregnant,

    life: Life, // alive | dead | ghost
    lived_days: u32, // last lived day.

    needs: Vec<(Need, u32)>,

    // abilities.
    abilities: Vec<(Ability, u32)>, // ability levels.
}

// https://stackoverflow.com/a/32936288/14029561

impl Wusel {
    /** Create a new Wusel with name. */
    fn new(name: String, female: bool) -> Self {
        let mut new = Self {
            id: 0, // XXX (2020-11-15) THREADED identifer
            name: name,

            body: Thing::new(0, 0),

            female: female,
            pregnant: Pregnant::NO,

            life: Life::ALIVE,
            lived_days: 0,

            needs: vec![],
            abilities: vec![],
        };

        /* Initiate all known needs to 0, critical. */
        for n in &Need::VALUES { new.needs.push((*n, 0)); }

        return new;
    }

    /** Create a new female Wusel. */
    fn xx(name: String) -> Self { Self::new(name, true) }

    /** Create a new male Wusel. */
    fn xy(name: String) -> Self { Self::new(name, false) }

    /** Show position of its body. */
    fn show_pos(self: &Self) -> String {
        return self.body.show_pos();
    }

    /** Move position of its body. */
    fn go(self: &mut Self, direction: Direction) { self.body.go(direction) }

    /** Tick one unit.
     * @return if the wusel is still alive in the end. */
    fn tick(self: &mut Self) -> bool {

        /* If in action, need changes may also apply, eg. eating. */
        // TODO

        /* Decrease every value by DEFAULT_NEED_DECAY_PER_MINUTE * minutes. */
        for i in 0 .. self.needs.len() {
            let (n, v) = self.needs[i];
            let decay = n.get_default_decay();

            self.needs[i] = (n, if v < decay { 0 } else { v - decay });
        }

        return self.is_alive()
    }

    /** Count a new day to the lived lifed. */
    fn add_new_day(self: &mut Self) {
        if self.is_alive() {
            /* Age one day. */
            self.lived_days += 1;

            /* Decay all abilities by one point. */
            for i in 0 .. self.abilities.len() {
                let (abi, val) = self.abilities[i];
                self.abilities[i] = (abi, val - 1);
            }
        }
    }

    /** Check, if this Wusel is alive. */
    fn is_alive(self: &Self) -> bool {
        return match self.life {
            Life::ALIVE => true, // all but alive are not alive.
            _ => false,
        }
    }

    /** Get Wusel's age. */
    fn age(self: &Self) -> u32 {
        self.lived_days
    }

    fn show_overview(self: &Self) {
        /* Show name. */
        print!("{}", self.name);

        /* Show Gender.. [\u2640 (9792) female, \u2642 (9794) male]. */
        print!(" {}\n", match self.female {
            true => "\u{2640}",
            _ => "\u{2642}",
        });

        /* Show age. */
        print!("{age} days ", age = self.age());

        /* Show life and age. */
        match self.life {
            Life::ALIVE => print!("(alive)"),
            Life::DEAD => print!("(dead)"),
            Life::GHOST => print!("(ghost)"),
        }

        print!("\n");

        /* Show needs. */
        println!("--- NEEDS: {:-<20}", "");
        self.show_needs();

        /* Show abilities. */
        println!("--- ABILITIES: {:-<16}", "");
        self.show_abilities();

        /* Show relations. */
        // TODO (2020-11-16) show relations.
    }

    /** Show all assigned needs. */
    fn show_needs(self: &Self) {
        for (n, v) in self.needs.iter() {
            println!("{name:>15} {value:5} {last:.>bar_len$} ",
                name = n.name(),
                value = v,
                bar_len = *v as usize, last = "");
        }
    }

    /** Print the Wusel's abilities. */
    fn show_abilities(self: &Self) {
       for (ability, value) in &self.abilities {
           println!("{a:>15} {v:5} {bar:*<v$}",
                    a = ability.name(),
                    v = *value as usize,
                    bar = "");
       }
    }

    /** Get the value for a need.
     * This may append the needs with a new default value, if the need is not
     * yet inserted. */
    fn get_need(self: &mut Self, need: Need) -> u32 {
        /* Find the need and return the value. */
        let size: usize = self.needs.len();
        for i in 0..(size) {
            let (n, v) = self.needs[i];
            if n == need { return v; }
                // return assigned value
        }
        /* If not found: Append with default Need value. */
        let default: u32 = 0;
        self.needs.push((need, default));
        return default
    }

    /** Set the value for a need.
     * This may append the needs with the new given value. */
    fn set_need(self: &mut Self, need: Need, new_value: u32) {
        /* Find the need and change the value. */
        let size: usize = self.needs.len();
        for i in 0..(size) {
            let (n, _) = self.needs[i];
            if n == need {
                self.needs[i] = (n, new_value); // update the value.
                return; // done
            }
        }
        /* If not found: Append with default Need value. */
        self.needs.push((need, new_value));
    }

    /** Change the value for a need relatively.
     * This may create a new value, with default input changed by the change value.
     * @return the new value.*/
    fn add_need(self: &mut Self, need: Need, change_value: u32) -> u32 {
        // 3x iterating. O(3n + c)
        let current = self.get_need(need) as i64; // get current value (or default)

        let mut changed = current + (change_value as i64);
        if changed < 0 { changed = 0; } // don't go below 0.

        self.set_need(need, changed as u32); // change the value.

        return self.get_need(need); // return final need's value.
    }

    /** Eat Food, take one bite.
     * This may use up the food. It may update the needs.
     */
    fn eat(self: &mut Self, food: &mut Food) {
        /* take a bite. */
        let successfully_fed: bool = food.take_a_bite();

        if successfully_fed {
            self.add_need(Need::SLEEP, food.needed_energy);
            self.add_need(Need::FOOD, food.satisfied_food);
            self.add_need(Need::WATER, food.satisfied_water);
        }
    }

    /** Improve the given ability by one point. */
    fn improve(self: &mut Self, ability: Ability) {
        /* Improve the given ability. */
        for i in 0 .. (self.abilities.len()) {
            let (a, v) = self.abilities[i];
            if ability == a {
                self.abilities[i] = (a, v + 1);
                return;
            }
        }
        /* If the given ability is not yet learned, add it to the abilities. */
        self.abilities.push((ability, 1));
    }

    /** Talk with an NPC.
     * @param npc Non-Playable Character, this Wusel talks with.
     * @param romantically If the talk meant to be romantically, the romantic value changed as well.
     * @param good If the talk was good, the relation has improved, otherwise worsen.
     */
    fn interact_with(self: &mut Self, other: &mut Self, romantically: bool, good: bool) {
        let val: i32;

        if good {
            self.improve(Ability::COMMUNICATION);
            val = 1;
        } else {
            val = -1;
        }

        /* Also change romance, if intended. */

        /* Change romance, if intended. */
        // TODO (2020-11-16) Relation and romance...
    }

    /** Talk with an NPC romantically.
     * @param npc Non-Playable Character, this Wusel talks with.
     * @param good If the talk was good, the relation has improved, otherwise worsen.
     */
    fn flirt(self: &mut Self, npc: &mut Self, good: bool) {
        self.interact_with(npc, true, good);
    }

    /** Talk with an NPC friendly.
     * @param npc Non-Playable Character, this Wusel talks with.
     * @param good If the talk was good, the relation has improved, otherwise worsen.
     */
    fn chat(self: &mut Self, npc: &mut Self, good: bool) {
        self.interact_with(npc, false, good);
    }

    /** Try to mate with the given NPC.
     * This may lead to pregnancy.
     */
    fn mate_with(self: &mut Self, npc: &mut Self, good: bool) {
        /* Mating is accepted, if romantically and friendly feelings are high. */
        let accepted: bool = good; // TODO (2020-11-15)  npc.romantic_value > 0 && npc.friendly_value > 0 && good;

        /* Pregnancy is allowed, if the mating genders are different, and randomly. */
        let impregnating: bool = self.female != npc.female;

        /* Abort, if mating is not accepted. */
        if !accepted { return; }

        /* This increases the friendship and romantic feelings.
         * It also improves the communication.
         * (5x more worth than normal flirt). */
        let factor = 2;
        for _ in 0u16..factor { self.flirt(npc, true); }

        if impregnating {
            let pregnancy_length = 7;

            if self.female {
                self.pregnant = Pregnant::YES(pregnancy_length);
            } else {
                npc.pregnant = Pregnant::YES(pregnancy_length);
            }
        }
    }

    /** Check, if the Wusel is pregnant. */
    fn is_pregnant(self: &Self) -> bool {
        match self.pregnant {
            Pregnant::NO => return false,
            _ => return true,
        }
    }
}
