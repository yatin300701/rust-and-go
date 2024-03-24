#[derive(Debug)]
struct User{
    name:String,
    id:i16,
}
struct Point(i16,i16);

fn main() {
    let first_user = User {
        id:1,
        name:String::from("Yatin")
    };
    println!("firstUser name is {} and his id id {}",first_user.name,first_user.id);

    let mut mutable_secon_user = User{
        name : String::from("Sukhi"),
        id:2
    };

    println!("Second user name is {}",mutable_secon_user.name);

    // changing name

    mutable_secon_user.name = String::from("Sukheeee");

    println!("Second user name changed to {}",mutable_secon_user.name);

    // can change name of immutable user ??
    // first_user.name = String::from("Yatraj");  -> error thrown ,change first_user to mut

    let sachin = initialise_user(String::from("Sachin"),4);

    println!("sachin id is {}",sachin.id);

    let akash = reuse_other_user_struct(&sachin,String::from("Akash"));
    println!("Akash id is same ({}) as Sachin's  ({})",akash.id,sachin.id);

    // touples
    let center_of_graph = Point(0,0);
    let top_right_point_of_graph = Point(10,10);

    println!("The top right point in graph with center ({},{}) is ({},{})",center_of_graph.0,center_of_graph.1,top_right_point_of_graph.0,top_right_point_of_graph.1);

    // print struct
    println!("Let's print struct sukhi {:?}",mutable_secon_user);

    // associated block
    mutable_secon_user.print_name();

}

fn initialise_user(name:String,id:i16)->User{
    // if same key as parameters 
    User{
        name,
        id
    }
}

fn reuse_other_user_struct(user:&User,name:String)->User{
     // reuse other struct to initialise new struct
    User{
        name,
        ..(*user)
    }
}

impl User{
    fn print_name(&self){
        println!("Current user name is {}",self.name);
    }

}