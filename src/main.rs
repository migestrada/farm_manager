use std::io;
mod menu;
mod farm;

use crate::menu::show_menu;
use crate::farm::Farm;

fn main() {
    let mut farm: Farm = Farm {
        current_sotarge: 0
    };

    loop {
        show_menu();

        farm.show_info();

        let mut selected_menu = String::new();
        io::stdin().read_line(&mut selected_menu).expect("Failed to read line");
        let selected_menu: u32 = match selected_menu.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match selected_menu {
            1=> {
                farm.add_ckicken();
            },
            2=> {
                farm.sub_ckicken();
            },
            3=> {
                break;
            }
            _=> {
                continue;
            }
        }
    }
}
