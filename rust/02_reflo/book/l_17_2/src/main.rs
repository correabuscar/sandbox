pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn average2(&self) -> &f64 {
        &self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut a: AveragedCollection = AveragedCollection {
        average: 0_f64,
        list: vec![],
    };
    let moo;
    a.add(1);
    a.add(10);
    {
        let zref = a.average2();
        println!("{}", zref);
        moo = *zref as i32 + 1; //6
    }
    a.add(moo);
    println!("moo={}", moo);
    a.add(moo);
    //println!("{}", zref);
    println!("{}", a.average2());
}
