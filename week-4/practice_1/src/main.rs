fn main() {
 let name = "Aisha Lawal";   
 let uni:&str ="Pan-Atlantic University";
let addr:&str = "km 52 Lekki-Epe Expressway, Ibeju-lekki,Lagos";
println!("Name: {}",name);
println!("Uniersity : {},\nAddress: {}",uni,addr);


let department:&'static str = "Computer Science";
let school:&'static str ="School of Science and Technology";
println!("Departmen : {}, \nSchool: {}",department,school);

}