#[derive(Clone, Copy)]
enum Color {
    YELLOW,
    WHITE,
    GREEN,
    BLUE,
    RED,
    ORANGE,
}
//x_face refers to the face whose center is x color
//since the implementation will not incclude slice moves like m or m' the center is immutable (white face center is always white)
pub struct Cube {
    white_face: [Color; 9],
    yellow_face: [Color; 9],
    green_face: [Color; 9],
    blue_face: [Color; 9],
    red_face: [Color; 9],
    orange_face: [Color; 9],
}

fn color_to_text(color: Color) -> &'static str {
    match color {
        Color::YELLOW => "🟨",
        Color::WHITE => "⬜",
        Color::GREEN => "🟩",
        Color::BLUE => "🟦",
        Color::RED => "🟥",
        Color::ORANGE => "🟧",
    }
}

impl Cube {
    //returns a solved cube
    pub fn default() -> Self {
        Cube {
            white_face: [Color::WHITE; 9],
            yellow_face: [Color::YELLOW; 9],
            green_face: [Color::GREEN; 9],
            blue_face: [Color::BLUE; 9],
            red_face: [Color::RED; 9],
            orange_face: [Color::ORANGE; 9],
        }
    }
    //is not to be used for any releases and is for seeing the cube easily (debuggin/new implementations)
    pub fn print_terminal(&self) {
        let empty: &str = "      ";
        let mut lines: Vec<String> = vec![];
        let order = vec![Color::RED, Color::BLUE, Color::ORANGE, Color::GREEN];
        let mut offset = 0;

        //adds yellow face to lines (since it will be printed above the next 4 as the top face)
        while offset != 3 {
            let mut line = String::new();
            line.push_str(empty);
            line.push_str(&self.get_row(Color::YELLOW, offset));
            offset += 1;
            lines.push(line);
        }
        offset = 0;
        while offset != 3 {
            let mut line = String::new();
            for color in &order {
                line.push_str(&self.get_row(*color, offset))
            }
            lines.push(line);
            offset += 1;
        }
        offset = 0;
        //same code as yellow face but for white
        while offset != 3 {
            let mut line = String::new();
            line.push_str(empty);
            line.push_str(&self.get_row(Color::WHITE, offset));
            offset += 1;
            lines.push(line);
        }

        for line in lines{
            println!("{line}");
        }
    }

    //non public functions
    //gets a horizontal row of 3
    fn get_row(&self, center: Color, offset: usize) -> String {
        let face = match center {
            Color::YELLOW => self.yellow_face,
            Color::WHITE => self.white_face,
            Color::GREEN => self.green_face,
            Color::BLUE => self.blue_face,
            Color::RED => self.red_face,
            Color::ORANGE => self.orange_face,
        };
        let mut out = String::new();
        out.push_str(color_to_text(face[0 + offset * 3]));
        out.push_str(color_to_text(face[1 + offset * 3]));
        out.push_str(color_to_text(face[2 + offset * 3]));
        out
    }
}
