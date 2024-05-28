#![allow(unused_macros)]
//the bad way:
macro_rules! match_generic_params0 {
    // Match (GenericParam `,`)* GenericParam `,`?
    ($($param:ident),* $(,)?) => {
        // This branch will match the input if it conforms to the EBNF
        {
            $(
                println!("Matched param: {}", stringify!($param));
            )*
        }
    };
}
// better but still no good to have 2 args:
macro_rules! match_generic_params1 {
    // Match (GenericParam `,`)* GenericParam `,`?
    (
        $(
            $param:ident
            $(
              ,  $param2:ident
            )*
            $(,)?
        )?
    ) => {
        // This branch will match the input if it conforms to the EBNF
        {
            println!("Start");
            $(
                println!("Matched param: {}", stringify!($param));
                $(
                    println!("Matched param: {}", stringify!($param2));
                )*
            )?
            println!("end");
        }
    };
}
//ok this is good now with 'internal rules' aka https://veykril.github.io/tlborm/decl-macros/patterns/internal-rules.html
macro_rules! match_generic_params2 {
    // Match (GenericParam `,`)* GenericParam `,`?
    (
        $(
            $param:ident
            $(
              ,  $param2:ident
            )*
            $(,)?
        )?
    ) => {
        match_generic_params!(@transcribe $($param $(, $param2)*)*);
    };
    (@transcribe $($params:ident),*) => {
        // This branch will match the input if it conforms to the EBNF
        {
            println!("Start");
            $(
                println!("Matched param: {}", stringify!($params));
            )*
            println!("end");
        }
    };
}
//better to print them in one line:
macro_rules! match_generic_params {
    // Match (GenericParam `,`)* GenericParam `,`?
    (
        $(
            $param:ident
            $(
              ,  $param2:ident
            )*
            //optionally can end with one comma but only if there was an ident already!
            $(,)?
        )?
    ) => {
        match_generic_params!(@transcribe $($param $(, $param2)*)?);
    };
    (@transcribe ) => {
        println!("no args");
    };
    (@transcribe $($param:ident),*) => {
        // This branch will match the input if it conforms to the EBNF
        {
            println!("Start");
            //src: DanielKeep https://users.rust-lang.org/t/help-about-writing-macros/111830/2?u=correabuscar
            println!(
                concat!("Matched param: ",
                    $(
                        match_generic_params!(@replace $param => " {}")
                    ),*
                ),
                $(
                    stringify!($param),
                )*
            );
            println!("end");
        }
    };
    //src: DanielKeep https://users.rust-lang.org/t/help-about-writing-macros/111830/2?u=correabuscar
    (@replace $_discard:tt => $($tts:tt)*) => {
        $($tts)*
    };
}

fn main() {
    // Valid cases
    match_generic_params!();
    match_generic_params!(A);
    match_generic_params!(A,);
    match_generic_params!(A, B);
    match_generic_params!(A, B,);
    match_generic_params!(A, B, C);
    match_generic_params!(A, B, C,);

    // Invalid case (uncomment to see the error)
    //match_generic_params!(,); // fixedFIXME: This should result in an error

    // Invalid case (uncomment to see the error)
    //match_generic_params!(A,,); // This should result in an error
}

