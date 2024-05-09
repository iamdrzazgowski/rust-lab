#[derive(Debug)]
enum Status{
    Running,
    Stopped,
    Done
}

static mut NEXT_PID: u64 = 0;

#[derive(Debug)]
struct Process {
    pid: u64,
    command: String,
    status: Status                                  //[Running, Stopped, Done]
}

impl Process {

    fn from_command(command: &str) -> Process{
        let pid = NEXT_PID;
        NEXT_PID += 1;
        Process{pid, command: command.to_string(), status: Status::Running}
    }

    fn shell() -> Process {
        Self::from_command("sh")
    }
}

fn main() {
    let command = "toilet";
    let script = "echo mleko | toilet & ls;";

    let p0 = Process{pid: 0, command: "sh".to_string(), status: Status::Running};
    let mut p1 = Process::from_command(command);
    let mut p2 = Process::shell();

    println!("{:?}", p1);
    println!("{:?}", p2);

    /*
    let p3 = Process::from_script(script);
    let mut p4 = Process::from_command(command);
    p1.stop();
    p2.start();
    p2.finish();
    println!("{}", !(p1 == p2));
    println!("{}", !(p1 == p4));
    println!("{}", p3.len() == 3);
    for process in p3 {
        println!("{}: {}", process.pid, process.command);
    }
    */
}