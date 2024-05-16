#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn from_u8(num: u8) -> Month {
        match num {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::January,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>
}

use std::str::FromStr;
impl Date {

    fn to_string(&self) -> String{
        let date = format!("{:02}.{:02}.{:04}", self.day, self.month as u8 + 1, self.year);
        
        date
    }

    fn from_3(day: u8, month: Month, year: u16) -> Date {
        Date{day, month, year, time: None}
    }

    fn from_string(string: &str, delim: char) -> Date {
        let string = string.to_string();
        let mut tokens = string.split(delim);
        let day = u8::from_str(tokens.next().unwrap()).expect("");
        let month = Month::from_u8(u8::from_str(tokens.next().unwrap()).expect(""));
        let year = u16::from_str(tokens.next().unwrap()).expect("");
        Date{day, month, year, time: None}
    }

    fn has_time(&self) -> bool {
        self.time.is_some()
    }

    fn set_time(&mut self, time: &Time) {
        self.time = Some(*time);
    }

    fn clear_time(&mut self) {
        self.time = None;
    }

}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn to_string(&self) -> String{
        let time = format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second);
        
        time
    }

    fn from_3(hour: u8, minute: u8, second: u8) -> Time {
        Time{hour, minute, second}
    }

    fn from_string(string: &str, delim: char) -> Time {
        let string = string.to_string();
        let mut tokens = string.split(delim);
        let hour = u8::from_str(tokens.next().unwrap()).expect("");
        let minute = u8::from_str(tokens.next().unwrap()).expect("");
        let second = u8::from_str(tokens.next().unwrap()).expect("");
        Time{hour, minute, second}
    }
}

fn main(){
    let date = Date::from_3(16, Month::May, 2024);
    println!("{:?}", date);

    let date2 = Date{day: 12, month: Month::May, year: 2002, time: None};
    println!("{:?}", date2.to_string());

    let d2 = Date::from_string("11.11.2023", '.');
    println!("{:?}", d2);

    let t1 = Time{hour: 12,minute: 34,second: 23};
    println!("{:?}", t1.to_string());

    let t2 = Time::from_string("12:45:12", ':');
    println!("{:?}", t2);

    let t3 = Time::from_3(17,43,52);
    println!("{:?}", t3);


    let mut d4 = Date::from_3(16, Month::May, 2024);
    println!("{:?}", d4);
    d4.set_time(&t1);
    println!("{:?}", d4);
}