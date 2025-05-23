#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczeń, Luty, Marzec, Kwiecień, Maj, Czerwiec, Lipiec,
    Sierpień, Wrzesień, Październik, Listopad, Grudzień,
}


struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>,
}

impl Date {
    fn to_string(&self) -> String {
        let date = format!("{:02}-{:?}-{}", self.day, self.month, self.year);
        if self.has_time() {
            return format!("{} {}", 
                            date, 
                            self.time.as_ref().unwrap().to_string());
        }
        return date;
    }

    fn has_time(&self) -> bool {
        match self.time {
            None => return false,
            Some(_) => return true,
        };
    }

    fn set_time(&mut self, time: Time) {
        self.time = Some(time);
    }

    fn clear_time(&mut self) {
        self.time = None;
    }

    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();
        let day: u8 = parts[0].parse().unwrap();
        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczeń" => Month::Styczeń,
            "luty" => Month::Luty,
            "marzec" => Month::Marzec,
            "kwiecień" => Month::Kwiecień,
            "maj" => Month::Maj,
            "czerwiec" => Month::Czerwiec,
            "lipiec" => Month::Lipiec,
            "sierpień" => Month::Sierpień,
            "wrzesień" => Month::Wrzesień,
            "październik" => Month::Październik,
            "listopad" => Month::Listopad,
            "grudzień" => Month::Grudzień,
            _ => panic!("Nieprawidłowy miesiąc")
        };

        let year: u16 = parts[2].parse().unwrap();
        Self { day, month, year, time: None }
    }

}

struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    fn from_3(hour: u8, minute: u8, second: u8) -> Self {
        Self { hour, minute, second }
    }

    fn from_string(string: &str) -> Self {
        let parts: Vec<&str> = string.split(':').collect();
        let hour: u8 = parts[0].parse().unwrap();
        let minute: u8 = parts[1].parse().unwrap();
        let second: u8 = parts[2].parse().unwrap();

        Self {hour, minute, second}
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month
            && self.year == other.year && self.time == other.time
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.minute == other.minute && self.second==other.second
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }

        self.time.partial_cmp(&other.time)  
    }
}
impl Eq for Time {}
impl Eq for Date {}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}




impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.minute.partial_cmp(&other.minute) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }

        self.second.partial_cmp(&other.second)
    }    
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Priority {
    Low, Medium, High,
}

#[derive(PartialEq)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    due: Date,
}

impl Task {
    fn from_4(name: &str, description: &str,
         priority: Priority, due: Date) -> Self {
        Self { 
            name: name.to_string(), 
            description: description.to_string(), 
            priority, 
            due 
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.priority.partial_cmp(&other.priority) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        self.due.partial_cmp(&other.due)
    }
}


fn main() {
    let mut date1 = Date::from_string("13-Maj-2025", '-');
    let mut date2 = Date::from_3(13, Month::Maj, 2025);
    let mut date3 = Date::from_string("13-Maj-2025", '-');

    println!("{}", date1.month == Month::Maj);
    println!("{}", date1.month == date2.month);

    println!("{}", date1.to_string());
    println!("{}", date2.to_string());
    
    date1.set_time(Time::from_3(11, 0, 0));
    date2.set_time(Time::from_string("10:59:59"));
    date3.set_time(Time::from_string("11:00:01"));
    
    println!("{}", date1.to_string());
    println!("{}", date2.to_string());
    println!("{}", date3.to_string());

    println!("{}", date1 > date2);
    println!("{}", date1 < date3);

    date1.clear_time();
    println!("{}", date1.to_string());
    println!("{}", date1 < date2);
    println!("{}", date1 < date3);

    date3.clear_time();
    println!("{}", date1 == date3);

    let task1 = Task::from_4("t1", "opis1",
                                 Priority::High, date1);

    let task2 = Task::from_4("task2", "opis inny",
                                 Priority::High, date3);
    
    println!("{}", (task1 > task2) == false);
    println!("{}", (task1 < task2) == false);
    println!("{}", (task1 == task2) == false);
    println!("{}", (task1 != task2));

}
