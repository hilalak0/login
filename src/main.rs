fn main() {


    let mut database: Vec<User> = Vec::new();

    register("Hilal".to_string(), "hilalak@gmail.com".to_string(),
     "hilal123".to_string(), &mut database);


     println!("Veritabanındaki kullanıcılar {:?}",database);


     login("hilalak@gmail.com".to_string(), "hilal123".to_string(), &mut database);

  
}


 
#[derive(Clone,Debug)]
struct User {
    username:String,
    email:String,
    password:String

}




fn register(username:String,email:String,password:String,database : &mut Vec<User> ) {


    if !email.contains("@") {

        println!("Email formatı yanlış gereçke bir email değil")
        
    }
    else {

        let user = User{
            username:username,
            email:email,
            password:password
        };
        let user1 = user.clone();
        database.push(user1);

        println!("Sisteme başarılı kayıt oldunuz {:?}",user.email)
        
    }
    
}

fn login (email:String,password:String,database : &mut Vec<User>){
     for data in database {

        if email == data.email && password == data.password{

            println!("Giriş başarılı hoşgeldiniz {}",data.username)
            
        }
        else {
            println!("Bilgileriniz hatalı sisteme giriş yapamadınız")
        }
         
     }
}
