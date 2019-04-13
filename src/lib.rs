use rand::prelude::*;

include!(concat!(env!("OUT_DIR"), "/names.rs"));

pub enum Sex {
    Male,
    Female,
}

pub fn first_name(sex: Sex, seed: u16) -> &'static str {
    let mut rng = thread_rng();

    let array = match sex {
        Sex::Female => FIRSTNAME_FEMALE,
        Sex::Male => FIRSTNAME_MALE,
    };
    let len = array.len();
    let max: u32 = array[len - 1].0;
    let count: u32 = if seed == 1000 {
        4000
    } else {
        rng.gen_range(0, max)
    };
    get_at_count(array, count, 0, len)
}

pub fn surname(seed: u16) -> &'static str {
    let mut rng = thread_rng();

    let array = SURNAME;
    let len = array.len();
    let max: u32 = array[len - 1].0;
    let count: u32 = if seed == 1000 {
        4000
    } else {
        rng.gen_range(0, max)
    };
    get_at_count(array, count, 0, len)
}

pub fn sex(seed: u16) -> Sex {
    let mut rng = thread_rng();

    match rng.gen_range(0, 2) {
        0 => Sex::Female,
        _ => Sex::Male,
    }
}

fn get_at_count(array: &[(u32, &'static str)], count: u32, min: usize, max: usize) -> &'static str {
    let avg = (min + max) / 2;
    let (low, high) = match avg {
        0 => (0, array[avg].0),
        _ => (array[avg - 1].0, array[avg].0),
    };
    if count >= low && count < high {
        array[avg].1
    } else if count < low {
        get_at_count(&array, count, min, avg - 1)
    } else {
        get_at_count(&array, count, avg + 1, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(first_name(Sex::Female, 1000), "Lucy");
        assert_eq!(first_name(Sex::Male, 1000), "Jacob");
        assert_eq!(surname(1000), "WILSON");
    }

    #[test]
    fn some_names() {
        for x in (1..100) {
            let sex = sex(0);
            let firstname = first_name(sex, 0);
            let surname = surname(0);
            println!("{} {}", firstname, surname);
        }
    }
}
