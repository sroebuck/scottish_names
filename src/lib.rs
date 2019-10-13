use rand::prelude::*;

include!(concat!(env!("OUT_DIR"), "/names.rs"));

/// Sex type used as a parameter to determine whether the names generated should be
/// male or female.
#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

/// Return a random Sex of Male or Female.
///
/// ```
/// use scottish_names::*;
///
/// let s = sex();
/// println!("Random sex is {:?}", s);
/// ```
pub fn sex() -> Sex {
    let mut rng = thread_rng();

    match rng.gen_range(0u8, 2u8) {
        0 => Sex::Female,
        _ => Sex::Male,
    }
}

/// Return a random first name of the given Sex, e.g. `Sex::Female` or `Sex::Male`.
///
/// ```
/// use scottish_names::*;
///
/// let n = first_name(Sex::Male);
/// println!("First name is {}", n);
/// ```
pub fn first_name(sex: Sex) -> &'static str {
    first_name_with_seed(sex, 0)
}

/// Return a random surname in all caps.
///
/// ```
/// use scottish_names::*;
///
/// let n = surname();
/// println!("Surname is {}", n);
/// ```
pub fn surname() -> &'static str {
    surname_with_seed(0)
}

type ArrayType = &'static [(FrequencyCount, &'static str)];
type FrequencyCount = u32;

fn first_name_with_seed(sex: Sex, seed: u16) -> &'static str {
    let (array, length) = match sex {
        Sex::Female => (FIRSTNAME_FEMALE, FIRSTNAME_FEMALE_LEN),
        Sex::Male => (FIRSTNAME_MALE, FIRSTNAME_MALE_LEN),
    };
    name(array, length, seed)
}

fn surname_with_seed(seed: u16) -> &'static str {
    name(SURNAME, SURNAME_LEN, seed)
}

fn name(array: ArrayType, length: usize, seed: u16) -> &'static str {
    let mut rng = thread_rng();

    let max: FrequencyCount = array[length - 1].0;
    let count: FrequencyCount = if seed == 1000 {
        4000
    } else {
        rng.gen_range(0, max)
    };
    get_at_count(array, count, 0, length)
}

fn get_at_count(array: ArrayType, count: FrequencyCount, min: usize, max: usize) -> &'static str {
    let avg = min + (max - min) / 4;
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
        assert_eq!(first_name_with_seed(Sex::Female, 1000), "Lucy");
        assert_eq!(first_name_with_seed(Sex::Male, 1000), "Jacob");
        assert_eq!(surname_with_seed(1000), "Wilson");
    }

    #[test]
    fn some_names() {
        for _ in 1..100 {
            let sex = sex();
            let firstname = first_name(sex);
            let surname = surname();
            println!("{} {}", firstname, surname);
        }
    }
}
