mod nice;

pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(&gift_message);

    println!("{}", gift_message);

    let good_deeds = 10;
    let bad_deeds = 2;
    let kid = nice::Kid::new(String::from("Johnny"), good_deeds, bad_deeds);
    println!("{:?}", kid.niceness)
}

pub fn attach_message_to_present(message: &String) {
    println!("The present now has this message: {}", message);
}
