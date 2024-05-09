#[derive(Debug, PartialEq)]
enum Status{
    Running,
    Stopped,
    Done
}

static mut NEXT_PID: u64 = 0;

#[derive(Debug, PartialEq)]
struct Process {
    pid: u64,
    command: String,
    status: Status                                  //[Running, Stopped, Done]
}

impl Process {

    fn from_command(command: &str) -> Process{

        let pid;
        unsafe{
            pid = NEXT_PID;
            NEXT_PID += 1;
        }

        Process{pid, command: command.to_string(), status: Status::Running}
    }

    fn from_script(script: &str) -> Vec<Process>{
        // "echo mleko | toilet & ls;"
        // p1 - echo mleko
        // p2 - toilet
        // p3 - ls

        script.to_string().split(|c: char| ['|', '&', ';'].contains(&c)).filter(|s| s.len() > 0).map(|s| Self::from_command(s)).collect()

        // let command: Vec<&str> = script.split(|x| {x == '|' || x == '&' || x == ';'}).collect();
        // let processes: Vec<Process> = command.iter().map(|x| {Self::from_command(x)}).collect(); 
        // processes
    }

    fn shell() -> Process {
        Self::from_command("sh")
    }

    fn stop(&mut self){
        self.status = Status::Stopped;
    }

    fn start(&mut self){
        self.status = Status::Running;
    }

    fn finish(&mut self){
        self.status = Status::Done;
    }
}

fn main() {
    let command = "toilet";
    let script = "echo mleko | toilet & ls;";

    // let p0 = Process{pid: 0, command: "sh".to_string(), status: Status::Running};
    let mut p1 = Process::from_command(command);
    let mut p2 = Process::shell();

    println!("{:?}", p1);
    println!("{:?}", p2);
    
    let p3 = Process::from_script(script);
    let p4 = Process::from_command(command);
    p1.stop();
    p2.start();
    p2.finish();

    println!("{}", !(p1 == p2));
    println!("{}", !(p1 == p4));
    
    println!("{}", p3.len() == 3);
    for process in p3 {
        println!("{}: {}", process.pid, process.command);
    }
    
}