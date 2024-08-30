pub(crate) mod ty_attr;

use hivemqtt_macros::Eleniyan;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Eleniyan)]
    struct MQTTProp {
        #[byte(6)]
        name: String,
        #[byte(3)]
        age: Option<u8>,
        #[byte(4)]
        friends: Vec<String>,
        #[byte(1)]
        origin: Option<u32>,
    }

    #[test]
    fn testing_bambi() {
        let xx = MQTTProp {name: String::from("tolumide"), age: Some(19u8), friends: vec![String::from("Human")], origin: None};
        // println!("the xx {:#?}", xx);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>>>>>>   {:#?}", xx.len());
        assert!(false);
    }



}