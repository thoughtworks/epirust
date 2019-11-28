use rand::seq::SliceRandom;

pub fn get_random_element_from<T: Copy>(vector: &Vec<T>, fallback_element: T) -> T {
    let choice = vector.choose(&mut rand::thread_rng());
    match choice {
        Some(x) => return *x,
        None => return fallback_element
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn get_random_element_from(){
        let vec = vec![1,2,3,4];
        let random = super::get_random_element_from(&vec, 5);

        assert_eq!(vec.contains(&random), true);
    }

    #[test]
    fn get_fallback_element(){
        let vec = vec![];
        let random = super::get_random_element_from(&vec, 5);

        assert_eq!(random, 5);
    }
}