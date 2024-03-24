fn main() {
    
    println!("New project for ownership");
    let s = String::from("Hello");
    let copy_s = s; // move

    // copy string

    let mut copy_str = copy_s.clone();

    // give and take ownership
    copy_str  = give_and_take_ownership(copy_str);

    println!("string is {}",copy_str);
    
    // reference
    use_and_throw(&copy_str);

    let _str_ref = &copy_str; // immutable ref
    let ref_str = & mut copy_str;

    ref_str.push_str("ha");
  



    // println!("{} is same as {} ", & mut copy_str,ref_str); cann't borrow mut ref more than one time
}

fn give_and_take_ownership(s:String)->String{
    s
}
fn use_and_throw(st: &String){
    println!("string is {}",st);

}
