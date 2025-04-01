struct Reg {
    a: u64,
    b: u64, 
    c: u64,
}

impl Reg {
    fn new(a: u64, b: u64, c: u64) -> Reg {
        Reg { a, b, c }
    }
    fn set_a(&mut self, a: u64) { self.a = a; }
    fn get_a(&mut self) -> u64 { self.a }
    fn set_b(&mut self, b: u64) { self.b = b; }
    fn get_b(&mut self) -> u64 { self.b }
    fn set_c(&mut self, c: u64) { self.c = c; }
    fn get_c(&mut self) -> u64 { self.c }
}

fn division(a: u64, b: u64) -> u64 {
    let b = u64::pow(2, b as u32);
    a / b
}

fn execute(opc: u8, opd: u64, combo_opd: u64, reg: &mut Reg, ip: &mut usize) -> Option<u8> {
    match opc {
        // adv, result to a
        0 => {
            let a = division(reg.get_a(), combo_opd);
            reg.set_a(a);
        },
        // bxl, result to b
        1 => {
            let b = reg.get_b() ^ opd;
            reg.set_b(b);
        },
        // bst, result to b
        2 => {
            let b = combo_opd % 8;
            reg.set_b(b);
        },
        // jnz, if jumps, ip not increased so return early
        3 => {
            if reg.get_a() != 0 {
                *ip = opd as usize;
                return None;
            }
        },
        // bxc, result to b
        4 => {
            let b = reg.get_b() ^ reg.get_c();
            reg.set_b(b);
        },
        // out, return output value, don't forget to increase ip before returning
        5 => {
            let out = combo_opd % 8;
            *ip += 2;
            return Some(out as u8);
        },
        // bdv, result to b
        6 => {
            let b = division(reg.get_a(), combo_opd);
            reg.set_b(b);
        }
        // cdv, result to c
        7 => {
            let c = division(reg.get_a(), combo_opd);
            reg.set_c(c);
        }
        _ => panic!("Unknown cmd: {opc}"),
    }
    *ip += 2;
    None
}

pub fn run(a: u64, b: u64, c: u64, program: &Vec<u8>, target: &Vec<u8>) -> Vec<u8> {
    let a = 117440;
    let mut reg = Reg::new(a, b, c);
    let mut output = Vec::new();
    let mut ip = 0;
    loop {
        println!("A as binary is {:0b}", reg.get_a());
        if ip >= program.len() { break; }
        let opc = program[ip];
        let opd = program[ip + 1] as u64;
        let combo_opd = if opd == 4 { reg.get_a() }
                       else if opd == 5 { reg.get_b() }
                       else if opd == 6 { reg.get_c() }
                       else { opd };
        if let Some(out) = execute(opc, opd, combo_opd, &mut reg, &mut ip) {
            output.push(out);
        }
    }
    println!("Part 2: {}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    return output
}