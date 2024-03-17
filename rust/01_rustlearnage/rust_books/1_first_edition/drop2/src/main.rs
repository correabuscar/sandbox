#[derive(Debug)]
struct Echo<'a> {
    num: u32,
    dropcount: &'a mut u32,//a ref to the global drop count, or something
}

impl<'a> Drop for Echo<'a> {
    fn drop(&mut self) {
        *self.dropcount += 1;
        println!("Dropping {:?}", self);
    }
}

fn som_err<'a>(input: bool, drop_count: &'a mut u32) -> Result<Echo<'a>, Echo<'a>> {
    if input {
        //return 
            Ok(Echo{ num: 6, dropcount: 
                //&mut  //this was not necessary! thanks to bob_twinkles on https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners
                drop_count })
                //;
    } else {
        return Err(Echo{ num: 7, dropcount: //&mut //same as above!
            drop_count });
    }
}

fn main() {
    let mut count = 0;//global drop count
    println!("Started");

    let _ = Echo {
        num: 5,
        dropcount: &mut count,
    }; //this is dropped here...
    assert_eq!(1, count);
    println!("After"); //...before this.

    let mut loop_count=0;
    let mut input: bool=true;
    loop {
        loop_count+=1;
        println!("!! Loop{}",loop_count);
//        if let Ok(some)=som_err(input, &mut count) {
//            println!("got {:?}", some);
//            println!("{}", count); //XXX: cannot borrow `count` as immutable because it is also borrowed as mutable
//
//        }
//        let ret=som_err(input, &mut count);
//        let x=ret.unwrap();
//        drop(x);
//        drop(ret);//use of moved value: `ret`
        match som_err(input, &mut count) {
            Ok(some) => {
                println!("got {:?}", some);
//                println!("{}", count); //XXX: cannot borrow `count` as immutable because it is also borrowed as mutable
                //println!("drop count = {}", some.dropcount);//yeah i guess this can work, lol
                assert_eq!(1, *some.dropcount);//aka not dropped yet;
            },// TODO: but is it dropped here? (probably here)
            Err(_) => {
                println!("ignored err");
                //println!("{}", count); //XXX: cannot borrow `count` as immutable because it is also borrowed as mutable
                //FIXME: good freakin' luck reading this! |count| that is, here!
            },
        }//or here?
        assert!(2==count && loop_count==1 || 4==count && loop_count==2); //dropped by the time this is reached!
        match som_err(input, &mut count) {
            Ok(_) => {
                println!("ignored input");
                //TODO: assert here that drop count is still 2, but can't read it because borrowed
                //and using _ so can't access it that way!
            },
            Err(e) => {
                println!("got err {:?}", e);
                //println!("drop count = {}", e.dropcount);//yeah i guess this can work, lol
                assert_eq!(4, *e.dropcount);//aka not dropped yet;
            },//will be dropped here, me guessing
        }//instead of here
        assert!(3==count && loop_count==1 || 5==count && loop_count==2); //droped already
        input=!input;
        if loop_count>=2 {
            break;
        }
    }
    println!("End.");
}
