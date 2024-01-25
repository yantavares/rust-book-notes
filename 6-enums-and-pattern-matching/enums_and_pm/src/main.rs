fn main() {
    enums();
    option_enum();
    matching_enums();
}

fn enums() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // _ (underline) indicated that it will not be used
    enum Message {
        _Quit,                       // Stores no data
        _Move { x: i32, y: i32 },    // Anon Struct
        _Write(String),              // Stores string
        _ChangeColor(i32, i32, i32), // Stores 3 i32
    }

    impl Message {
        fn some_function() {
            // enums can have methods
            println!("This is a method!");
        }
    }
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    let my_ip = IpAddrKind::V6(String::from("some ip"));

    Message::some_function();

    println!("This is IPV4 {:?} this is IPV6 {:?}", localhost, my_ip)
}

fn option_enum() {
    let x = 5;
    let y = Some(5);

    // ! let sum = x + y;

    let sum = x + y.unwrap_or(0); // Uses value, in case of none, returns 0
    println!("Sum: {}", sum);
}

fn matching_enums() {
    enum Coin {
        Penny,
        _Nickel,
        _Dime,
        _Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                return 1;
            }
            Coin::_Nickel => 5,
            Coin::_Dime => 10,
            Coin::_Quarter => 25,
        }
    }

    let penny = Coin::Penny;
    let penny_value = value_in_cents(penny);
    println!("Value of a penny: {} cents", penny_value)
}
