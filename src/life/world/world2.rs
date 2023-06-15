// TODO (2021-11-25) refactor the way something is stored in the world.
// TODO (2021-11-25) refactor how to peek into the world.
// TODO (2021-11-27) handler: life to life manager, positional things by world.
//
//
// TODO (2023-06-13) world.dimensions() => world.area() => world.positions()
// TODO (2023-06-13) world.time() : usize / current time.
// TODO (2023-06-13) world.wusels() // wusel_ids
// TODO (2023-06-13) world.wusel_new() // wusel_ids
// TODO (2023-06-13) world.wusel_set(id, ...) // update data
// TODO (2023-06-13) world.wusel_get(id) // copy of wusel data to view.
// TODO (2023-06-13) world.interactive_items() // objects for wusels. (food, doors, ...)
// TODO (2023-06-13) world.noninteractive_items() // Contruction walls, stairs (also doors)
// TODO (2023-06-13) world.items_set(id) // update item.
// TODO (2023-06-13) world.items_get(id) // data.

use super::areas;
use super::wusels;

type WorldId = usize;

/// The world is a all containing data tuple.
/// It contains ongoing tasks, the actors and other items.
/// It also has the time, and positional bounds.
///
/// The world is then mainained by an external engine, updating time,
/// positions and objects.
///
pub struct World2 {
    id: WorldId,
    clock: u64,

    /// dimenensions and bounds of the map.
    area: areas::Area,

    /// living creatures in the world
    /// (wrapped with a position index.)
    wusels: Vec<(wusels::Wusel, usize)>,

    /// task of all undone tasks of every wusel.
    /// this is like a global task list.
    tasks: Vec<wusels::tasks::Task>,

    /// Wusel to Wusel Connection
    relations: Vec<wusels::relations::Relation>,

    /// Objects a wusel can interact with.
    /// These can be consumed, replaced or taken by any wusel.
    /// (wrapped with a position index)
    items: Vec<(char, bool, bool, usize)>,
    // (item, is pass trough, is moveable, position index)
    // u food, _ bed, L toilet
    /// Fixed Objects, a wusel cannot interact with,
    /// such as walls or trees.
    /// A Wusel needa to route around.
    /// They are player set or generated randomly to make a beautiful home.
    /// (wrapped with a position index)
    environment: Vec<(char, usize)>,
    // (environment, is pass trough, position index)
    // > stairs, + door, # wall, P tree
}

impl std::fmt::Display for World2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "World {} (time: {}, dimensions: {:?})",
            self.id, self.clock, self.area
        )
    }
}

// immutable getters
// copies of the actual values.
impl World2 {
    /// Create a new empty world (with random id)
    pub fn new_with_random_id(width: u32, depth: u32, height: u32) -> Self {
        Self::new(rand::random::<usize>(), width, depth, height)
    }

    /// Create a new empty world
    pub fn new(id: usize, width: u32, depth: u32, height: u32) -> Self {
        Self {
            id,
            clock: 0,
            area: areas::Area::new(areas::Position::ROOT, width, depth, height),
            wusels: vec![],
            tasks: vec![],
            relations: vec![],
            items: vec![],
            environment: vec![],
        }
    }

    /// Get immuatble world id
    pub fn id(&self) -> WorldId {
        self.id
    }

    /// Get immuatble world clock
    pub fn clock(&self) -> u64 {
        self.clock
    }

    /// Get immutable dimenensions and bounds of the map.
    pub fn area(&self) -> areas::Area {
        self.area
    }

    /// Get immuable copy of current wusels' data
    pub fn wusels(&self) -> Vec<(wusels::Wusel, usize)> {
        self.wusels.to_vec()
    }

    /// Get immutable copy of current tasks' data
    pub fn tasks(&self) -> Vec<wusels::tasks::Task> {
        self.tasks.to_vec()
    }

    /// Get immutable copy of current Wusel-to-Wusel Connections
    pub fn relations(&self) -> Vec<wusels::relations::Relation> {
        self.relations.to_vec()
    }

    /// Get immutable copy of current items
    pub fn items(&self) -> Vec<(char, bool, bool, usize)> {
        self.items.to_vec()
    }

    /// Get immuatble copy of current environmental objects
    pub fn environment(&self) -> Vec<(char, usize)> {
        self.environment.to_vec()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_create_world() {
        // setup
        let id = 1234usize;
        let width = 13;
        let depth = 19;
        let height = 7;

        // exec
        let world = super::World2::new(id, width, depth, height);

        // assert
        let area = world.area();
        assert_eq!(width, area.width());
        assert_eq!(depth, area.depth());
        assert_eq!(height, area.height());
        assert_eq!(id, world.id());

        // empty world.
        assert!(world.wusels().is_empty());
    }

    #[test]
    fn should_add_a_new_wusel() {
        // setup
        let world = super::World2::new(0, 2, 1, 1);

        // execution
        // TODO add wusel.

        // execution
        assert_eq!(1, world.wusels().len());
    }

    #[test]
    fn should_get_only_as_immutable_copy() {
        // setup
        let world = super::World2::new(0, 2, 1, 1);
        // TODO add wusel.

        // execution
        let mut ws0 = world.wusels();
        ws0[0].0.set_name("mdofied name".to_string());

        let ws1 = world.wusels();

        // assert: external did not influence.
        assert_eq!(ws0, ws1);
    }

    #[test]
    fn should_assign_task_to_one_wusel() {
        // setup
        let world = super::World2::new(0, 2, 1, 1);
        // TODO add wusel.
        let wusel_id = 0;

        // execution
        // TODO assign task

        // assert: external did not influence.
        assert_eq!(1, world.tasks().len());
        assert_eq!(wusel_id, world.tasks()[0].get_active_actor_id());
    }

    #[test]
    fn should_assign_task_to_multiple_wusel() {
        // setup
        let world = super::World2::new(0, 2, 1, 1);
        // TODO add wusel.
        let wusel_id = 0;

        // execution
        // TODO assign task

        // assert: external did not influence.
        assert_eq!(1, world.tasks().len());
        assert_eq!(wusel_id, world.tasks()[0].get_active_actor_id());

        // execution
        // TODO assign task

        // assert: external did not influence.
        assert_eq!(2, world.tasks().len());
        assert_eq!(wusel_id, world.tasks()[0].get_active_actor_id());
    }

    #[test]
    fn should_filter_one_wusel_tasks() {
        // setup
        let world = super::World2::new(0, 2, 1, 1);
        // TODO add wusel.
        // TODO add wusel.
        let wu_id0 = 0;

        // execution
        // TODO assign tasks (2@w0, 1@w1)

        // assert: external did not influence.
        assert_eq!(3, world.tasks().len());

        // TODO world.shorthand for that.
        let filtered_by_hand = world
            .tasks()
            .into_iter()
            .filter(|t| t.get_active_actor_id() == wu_id0)
            .collect::<Vec<super::wusels::tasks::Task>>();
        assert_eq!(filtered_by_hand.len(), world.tasks().len());
        // TODO (2023-06-15) assert all assignex to wu_id0 as active actor.
    }
}
