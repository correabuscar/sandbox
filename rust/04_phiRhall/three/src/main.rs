//thanks to <danieldg> on #rust irc.mozilla.org for suggesting MPI (an HPC framework)
extern crate mpi;

use std::sync::Arc;
use std::sync::Weak;

use {mpi::topology::Communicator, mpi::traits::*};

//static mut UNIVERSE: Option<&mpi::environment::Universe> = None; //fail, can't refer to something I've created in main() (does not live long enough)
static mut UNIVERSE: Option<Weak<mpi::environment::Universe>> = None;
// ^ thanks to <Mutabah> on #rust irc.mozilla.org for the Arc Weak idea! https://doc.rust-lang.org/std/sync/struct.Arc.html#breaking-cycles-with-weak https://doc.rust-lang.org/std/sync/struct.Weak.html

macro_rules! code_duplication_avoider {
    ($rank:ident) => {{
        let arc_weak = unsafe {
            UNIVERSE
                .take()
                .expect("Tried to use macro 'mpi_println' before UNIVERSE got set!")
        };
        let arc_strong = arc_weak.upgrade().unwrap();
        $rank = arc_strong.world().rank();
        unsafe { UNIVERSE = Some(arc_weak) };
    }};
}

macro_rules! mpi_println {
    // why 'ident' ? https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
    () => {{ //yup, have to handle this separately! due to format!() not accepting no args!
        {
            let rank;
            code_duplication_avoider!(rank);
            println!("{}:{}"
                     ,mpi::environment::processor_name().unwrap()
                     ,rank
            );
        }
    }};
    ($($arg:tt)*) => ({
        {
            let rank;
            code_duplication_avoider!(rank);
            println!("{}:{} {}"
                     ,mpi::environment::processor_name().unwrap()
                     ,rank
                     ,format!($($arg)*)
            ); //thanks to <UndeadLeech> on #rust irc.mozilla.org for the how-to-prepend idea!
        } //inner block for temp vars
    })
}

fn main() {
    //let universe = mpi::initialize().unwrap();
    let universe = mpi::initialize_with_threading(mpi::Threading::Multiple)
        .map(|x| x.0)
        .unwrap(); //mpi::environment::Universe
                   //universe = 1;
                   //assert!(mpi::is_initialized());//ah it's not public!
    let world = universe.world(); //mpi::topology::SystemCommunicator
    let stuff: std::option::Option<mpi::environment::Universe> = Some(universe);
    let blah = stuff.unwrap(); // moving out of local binding 'stuff' works, but not from static item UNIVERSE(seen below)
    let universe = blah;
    let arc_strong = Arc::new(universe);
    let arc_weak = Arc::downgrade(&arc_strong);
    //mpi_println!("test");
    unsafe {
        assert!(UNIVERSE.is_none());
        //UNIVERSE = Some(&universe);
        UNIVERSE = Some(arc_weak);
        // error[E0133]: use of mutable static is unsafe and requires unsafe function or block
        // note: mutable statics can be mutated by multiple threads: aliasing violations or data
        // races will cause undefined behavior
    }
    //    mpi_println!("test2");
    //    mpi_println!("test3");
    //let _univ = unsafe { UNIVERSE.unwrap() }; //same error here, but UNIVERSE.take().unwrap()
    //works!

    //world = 1;//mpi::topology::SystemCommunicator
    //WORLD = Some(world);
    let size = world.size();
    let rank = world.rank();

    //    mpi_println!(
    //        "Processor name is '{:?}'",
    //        mpi::environment::processor_name().unwrap()
    //    );
    if size != 2 {
        panic!(
            "Size of MPI_COMM_WORLD must be 2, but is {}! Try `mpirun -n 2 -- {:?}`",
            size,
            std::env::current_exe().ok().unwrap()
        );
    }

    match rank {
        0 => {
            let msg = vec![4.0f64, 8.0, 15.0];
            world.process_at_rank(rank + 1).send(&msg[..]);
        }
        1 => {
            let (msg, status) = world.any_process().receive_vec::<f64>();
            mpi_println!(
                "Process {} got message {:?}.\nStatus is: {:?}",
                rank,
                msg,
                status
            );
        }
        _ => unreachable!(),
    }

    //second example:
    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);

    let mut x;
    if world.rank() == root_rank {
        x = 2_u64.pow(10);
        mpi_println!("Root broadcasting value: {}.", x);
    } else {
        x = 0_u64;
    }
    root_process.broadcast_into(&mut x);
    mpi_println!("Rank {} received value: {}.", world.rank(), x);
    assert_eq!(x, 1024);
    mpi_println!();

    let mut a;
    let n = 4;
    if world.rank() == root_rank {
        a = (1..).map(|i| 2_u64.pow(i)).take(n).collect::<Vec<_>>();
        mpi_println!("Root broadcasting value: {:?}.", &a[..]);
    } else {
        a = std::iter::repeat(0_u64).take(n).collect::<Vec<_>>();
    }
    root_process.broadcast_into(&mut a[..]);
    mpi_println!("Rank {} received value: {:?}.", world.rank(), &a[..]);
    assert_eq!(&a[..], &[2, 4, 8, 16]);

    // The following is no longer needed now that I'm using Arc Weak:
    // thanks to <stephaneyfx> for .take() [even though I found it at the same time] on
    // #rust-beginners irc.mozilla.org
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
    //    let _univ = unsafe {
    //        assert!(UNIVERSE.is_some());
    //        // to avoid `mpirun has exited due to process rank 0 with PID 0 on node Z575 exiting improperly.`
    //        // this happens because drop() isn't called for static stuffs https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=90d0a900c21fcebcabcb48dc45986cdf
    //        // this is intended, confirmed by <sfackler>
    //        // FIXME: this workaround is crap because if something exits uncleanly, UNIVERSE won't get
    //        // drop() called; so drop() would get called at end of main only because of this here:
    //        UNIVERSE.take().unwrap()
    //    };
}
