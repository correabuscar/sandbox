#![allow(dead_code)]

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {
        use outermost::inside::inner_function;//required for
        inner_function();//this

        //::outermost::inside::secret_function();//function `secret_function` is private
    }
    fn blah() {
        ::outermost::inside::inner_function();//no 'use' needed!
        inside::inner_function();
        //inner_function();//this won't work w/o a 'use'
        //inside::secret_function();//function `secret_function` is private
    }

    mod inside {
        pub fn inner_function() {
        use outermost::middle_function;//required,
            middle_function();//for this!
            ::outermost::middle_secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    //outermost::middle_secret_function();//function `middle_secret_function` is private
    //outermost::inside::inner_function();//module `inside` is private
    //outermost::inside::secret_function();//error[E0603]: module `inside` is private
}

fn main(){}
