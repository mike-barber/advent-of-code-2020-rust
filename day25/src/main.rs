
const DIVISOR: i64 = 20201227;

fn transform_once(number: i64, subject_number: i64) -> i64 {
    let n = number * subject_number;
    n % DIVISOR
}

fn transform_number(subject_number: i64, loop_size: usize) -> i64 {
    let mut number = 1;
    for _ in 0..loop_size {
        number = transform_once(number, subject_number);
    }
    number
}

fn find_loop_size(subject_number: i64, target_public_key: i64) -> usize {
    let mut number = 1;
    for loop_number in 1.. {
        number = transform_once(number, subject_number);
        if number == target_public_key {
            return loop_number
        }
    }
    panic!("unexpected: no solution");
}


fn part1() {
    let card_pkey = 11349501;
    let door_pkey = 5107328;

    let initial_subject = 7;
    let card_loop_size = find_loop_size(initial_subject, card_pkey);
    let door_loop_size = find_loop_size(initial_subject, door_pkey);
    
    println!("card loop size {}", card_loop_size);
    println!("door loop size {}", door_loop_size);

    let enc_key_1 = transform_number(card_pkey, door_loop_size);
    let enc_key_2 = transform_number(door_pkey, card_loop_size);

    println!("encryption key 1: {}", enc_key_1);
    println!("encryption key 2: {}", enc_key_2);
}

fn main() {
    // part 1 
    part1();
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn example_correct() {
        let initial_subject_number = 7;
    
        let card_public_key = 5764801;
        let card_loop_size = 8;
        assert_eq!(
            card_public_key,
            transform_number(initial_subject_number, card_loop_size)
        );
    
        let door_public_key = 17807724;
        let door_loop_size = 11;
        assert_eq!(
            door_public_key,
            transform_number(initial_subject_number, door_loop_size)
        );
    
        let encryption_key_door = transform_number(door_public_key, card_loop_size);
        let encryption_key_card = transform_number(card_public_key, door_loop_size);
        assert_eq!(14897079, encryption_key_door);
        assert_eq!(14897079, encryption_key_card);
    }

    #[test]
    fn find_loop_size_correct() {
        let initial_subject_number = 7;
    
        let card_public_key = 5764801;
        let card_loop_size = find_loop_size(initial_subject_number, card_public_key);
        assert_eq!(card_loop_size, 8);

        let door_public_key = 17807724;
        let door_loop_size = find_loop_size(initial_subject_number, door_public_key);
        assert_eq!(door_loop_size, 11);
    }

}
