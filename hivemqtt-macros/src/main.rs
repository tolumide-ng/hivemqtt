pub(crate) mod data_size;

use hivemqtt_macros::DataSize;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, DataSize)]
    struct MQTTProp {
        #[bytes(6)]
        name: String,
        #[bytes(3)]
        age: Option<u8>,
        // #[bytes(4)]
        #[bytes(wl_4)]
        friends: Vec<String>,
        // #[bytes(wl_1)]
        origin: Option<u32>,
        #[bytes(kv_2)]
        relations: Vec<(String, String)>
    }

    #[test]
    fn testing_bambi() {
        let xx = MQTTProp {name: String::from("tolumide"), age: Some(19u8), friends: vec![String::from("Human")], origin: None,
    relations: vec![(String::from("father"), String::from("Daddy")), (String::from("mother"), String::from("Mummy"))]};
        // println!("the xx {:#?}", xx);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>>>>>>   {:#?}", xx.len());
        assert!(false);
    }



}