use druid::{
    widget::{
        Flex, Label, Align, Container, Padding, Split,
    },
    AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, UnitPoint, piet::Color, MenuDesc,
    MenuItem, Command, Target, Selector, Application::quit
};

use rand::Rng;

const WINDOW_TITLE: LocalizedString<GameState> = LocalizedString::new("Cat Game");
const WIDTH: f64 = 1500.0;
const HEIGHT: f64 = 900.0;

#[derive(Clone, Data, Lens)]
struct GameState {
    name: String,
}

pub fn cat_game() {
    println!("Starting Cat Game ...");

    // describe the main window
    let num_items: usize = 4;
    const MENU_COUNT_ACTION: Selector<usize> = Selector::new("menu-count-action");

    let my_menu: MenuDesc<GameState> = MenuDesc::platform_default();

    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((WIDTH, HEIGHT))
        .menu(my_menu);
    println!("Rood widget finished");

    // create the initial app state
    let initial_state = GameState {
        name: "World".into(),
    };

    // start the application
    println!("Starting the application ...");
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    println!("Quitting Cat Game ...");
}

struct Room {
    name: String,
    storage: [u8; 10], // 0 -> no item, 1 -> cat, 2 -> upgrade
    chance: u64,
    counter_time: u64,
    upgrades_bought: u8,
}

struct Shop {
    money: u128,
}

impl Room {
    fn new(name: String) -> Self {
        Self {
            name,
            storage: [0u8; 10],
            chance: rand::thread_rng().gen_range(1..101),
            counter_time: rand::thread_rng().gen_range(45..91),
            upgrades_bought: 0
        }
    }

    fn buy_upgrade(&mut self, shop: &mut Shop) -> bool {
        println!("Buying upgrade in room {:?}", self.name);
        return match shop.subtract_money(self.get_upgrade_prise() as u128) {
            true => {
                if self.add_to_storage(2) == false {
                    shop.add_money(self.get_upgrade_prise() as u128)
                }
                self.upgrades_bought += 1;
                println!("Bought upgrade in room {:?}", self.name);
                true }
            false => {
                println!("Failed buying upgrade, not enough money in room {:?}", self.name);
                false
            }
        }
    }

    fn add_chance(&mut self, amount: u64) {
        println!("Adding {} to chance in room {:?}", amount, self.name);
        match self.chance + amount {
            1..=100 => { self.chance += amount },
            _ => { self.chance = 100 },
        }
        println!("Completed adding {} to chance in room {:?}", amount, self.name);
    }

    fn get_upgrade_prise(&self) -> u16{
        (&self.upgrades_bought * 50 + 50) as u16
    }

    fn add_to_storage(&mut self, item: u8) -> bool{
        println!("Adding to storage in room {:?}", self.name);
        for value in self.storage.iter_mut() {
            match &value {
                0 => {
                    *value = item;
                    println!("Added {} to storage in room {:?}", item, self.name);
                    return true;
                },
                _ => continue,
            }
        }
        println!("Failed adding cat to storage because storage is full in room {:?}", self.name);
        return false
    }

    fn buy_cat(&mut self) -> bool{
        println!("Buying a cat in room {:?}", self.name);
        return if rand::thread_rng().gen_range(1..101) > self.chance {
            println!("Bought a cat in room {:?} but no cat added", self.name);
            false
        } else {
            self.add_to_storage(1);
            println!("Bought a cat in room {:?}", self.name);
            true
        }
    }
}

impl Shop {
    fn new() -> Self {
        Self {
            money: 0
        }
    }

    fn subtract_money(&mut self, amount: u128) -> bool{
        println!("Subtracting {} euro ...", amount);
        return if (self.money as i128) - (amount as i128) < 0 {
            println!("Failed subtracting money, not enough money to subtract");
            false
        } else {
            self.money -= amount;
            println!("Successfully subtracted {} euro", amount);
            true
        }
    }

    fn add_money(&mut self, amount: u128) {
        println!("Adding {} euro", amount);
        self.money += amount;
        println!("Successfully added {} euro", amount);
    }
}

fn build_root_widget() -> impl Widget<GameState> {
    println!("Building root widget ...");

    let fixed_cols = Padding::new(
        10.0,
        Container::new(
            Split::columns(
                Align::centered(Label::new("Shop    Settings")),
                Align::centered(Label::new("money: xxxx")),
            )
                .split_point(0.5),
        )
    );

    let draggable_cols = Padding::new(
        10.0,
        Container::new(
            Split::columns(
                Align::centered(Label::new("rooms")),
                Align::centered(Label::new("quick buy \nmost used items")),
            )
                .split_point(0.85)
                .draggable(true)
                .solid_bar(true)
                .min_size(60.0, 60.0),
        )
    );

    println!("Finishing root widget ...");
    Padding::new(
        10.0,
        Container::new(
            Split::rows(
                fixed_cols,
                draggable_cols,
            )
                .split_point(0.05)
                .bar_size(2.5)
                .min_bar_area(5.0)
                .draggable(true),
        )
            .border(Color::WHITE, 1.0),
    )
}
