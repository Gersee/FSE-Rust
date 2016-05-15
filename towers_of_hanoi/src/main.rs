/*
 * n gibt die Anzahl der Steine an, from den Startpunkt, to den Zielpunkt und via das Ausweichfeld
 */
fn move_(n: i32, from: i32, to: i32, via: i32) {
    if n > 0 {
        move_(n - 1, from, via, to);
        println!("Move disk from pole {} to pole {}", from, to);
        move_(n - 1, via, to, from);
    }
}

fn main() {
    let start = 3;
    let startpole = 1;
    let targetpole = 2;
    let movepole = 3;
    if start > 0 && startpole != targetpole && startpole != movepole && targetpole != movepole {
        println!("Start with #{} discs on pole {} with target pole {}", start, startpole, targetpole);
        move_(start, startpole,targetpole,movepole);
    } else {
        println!("Error with starting parameters!");
    }
}
