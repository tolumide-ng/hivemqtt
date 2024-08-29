use hivemqtt_macros::Length;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Length)]
    struct MQTTProp {
        #[byte = "2"]
        name: String,
        #[byte = "1"]
        age: Option<u8>,
        #[byte = 4]
        friends: Vec<String>,
        #[byte = 1]
        origin: Option<u32>,
    }

    #[test]
    fn testing_bambi() {
        let xx = MQTTProp {name: String::from("tolumide"), age: Some(19u8), friends: vec![String::from("Human")], origin: None};
        // println!("the xx {:#?}", xx);
        assert!(false);
    }



}