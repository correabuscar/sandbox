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
macro_rules! match_generic_params {
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

fn main() {
    // Valid cases
    match_generic_params!();
    match_generic_params!(A);
    match_generic_params!(A,);
    match_generic_params!(A, B);
    match_generic_params!(A, B,);

    // Invalid case (uncomment to see the error)
    //match_generic_params!(,); // fixedFIXME: This should result in an error

    // Invalid case (uncomment to see the error)
    //match_generic_params!(A,,); // This should result in an error
}

