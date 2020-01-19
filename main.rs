mod cricket {      //  create a module with sub module and function
    pub mod player {
        pub fn team_player (){
            println! ("In Cricket Team Total 11 player play the match");
        }
    } 
}

mod lib;
use crate::lib::mountain::largest::K2; // lib root address

fn main() {
cricket::player::team_player ();  // the the mod function 

K2 (); // call the libary in main function
}
