//#[allow(unreachable_code)]
//^ this would work but, I don't want to allow unreachable code for anything other than that specific statement!
fn main()
{
  std::process::exit(1);
  #[allow(unreachable_code)]
  unreachable!(); //FIXME: goal:make this not friggin' warn!
}
