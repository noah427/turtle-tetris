use serde::Deserialize;
use serde_json::Result;
use std::io;
use turtle;
use websocket::client as Client;

#[derive(Deserialize)]
struct Board {
    Grid: [[u8; 10]; 20],
}

fn main() {
    let mut turtle = turtle::Turtle::new();
    turtle.set_speed("instant");
    let mut address = String::new();
    println!("enter websocket address : ");
    io::stdin().read_line(&mut address).unwrap();
    let mut client = Client::ClientBuilder::new(&address).expect("bad address");

    let mut stream = client.connect_insecure().expect("couldn't connect");

    loop {
        let messages = stream.incoming_messages();

        for message in messages {
            let msg = message.unwrap();
            match msg {
                websocket::OwnedMessage::Text(txt) => {
                    let board: Board = serde_json::from_str(&txt).expect("not board struct");
                    let grid = board.Grid;

                    let mut x = 0;
                    let mut y = 0;

                    while x <= 400 && y <= 800 {

                        if x == 400 {
                            x = 0;
                            y += 40;
                        }

                        if y == 800 {
                            break;
                        }


                        match grid[y / 40][x / 40] {
                            0 => turtle.set_fill_color("white"),
                            _ => turtle.set_fill_color("black"),
                        }

                        turtle.pen_up();

                        turtle.go_to(turtle::Point {
                            x: x as f64,
                            y: y as f64,
                        });

                        turtle.pen_down();

                        turtle.begin_fill();

                        for _ in 0..4 {
                            turtle.forward(40 as f64);
                            turtle.right(90 as f64);
                        }

                        turtle.set_heading(0 as f64);

                        turtle.end_fill();

                        x += 40;
                    }
                }
                _ => {
                    println!("yikes unknown message")
                }
            }
        }
    }
}

