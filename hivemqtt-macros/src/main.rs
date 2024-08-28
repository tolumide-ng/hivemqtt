use hivemqtt_macros::Length;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Length)]
    struct MQTTProp {
        #[byte = "2"]
        name: String
    }

    #[test]
    fn testing_bambi() {
        let xx = MQTTProp {name: String::from("tolumide")};
        println!("the xx {:#?}", xx);
        assert!(false);
    }



}