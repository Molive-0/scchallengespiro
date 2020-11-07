use turtle::Turtle;
use turtle::Point;

struct Circle {
    rad: f64,
    speed: f64,
    rot: f64,
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_pen_size(3.0);
    turtle.use_radians();
    turtle.hide();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let mut circle_vec = vec![];
    let mut speed:f64 = 0.0;
    for circle in buffer.split(";") {
        let mut split = circle.trim().split(",");
        let rad = str::parse::<f64>(split.next().unwrap()).unwrap();
        speed += str::parse::<f64>(split.next().unwrap()).unwrap();
        let c = Circle{rad:rad,speed:speed,rot:0.0};
        circle_vec.push(c);
    }

    turtle.pen_up();
    turtle.go_to(get_pos(&mut circle_vec)*250.0);
    turtle.pen_down();
    loop {
        turtle.go_to(get_pos(&mut circle_vec)*250.0);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn get_pos(circle_vec: &mut Vec<Circle>) -> Point {
    let mut p = Point {x:0.0,y:0.0};
    for circle in circle_vec {
        p.x += circle.rot.sin()*circle.rad;
        p.y += circle.rot.cos()*circle.rad;
        circle.rot += circle.speed;
    }
    p
}