pub mod client;
pub mod network;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod outermost {
    pub fn middle_function() {
        middle_secret_function();
    }

    fn middle_secret_function() {
        inside::inner_function();
    }

    mod inside {
        pub fn inner_function() {
            secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();    
}