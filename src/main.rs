mod instructions;
mod registers;
mod intermediate;

use instructions::*;
use registers::*;
use std::any::Any;
use std::io;
use std::io::BufRead;
use std::ops::Deref;

macro_rules! instr {
    (
        $(
            $op:ident $($arg:expr),* ;
        )*
    ) => {
        {
            let x: Vec<Box<dyn Instruction>> = vec![
                $(
                    Box::new( $op ( $($arg),* ) ),
                )*
            ];
            x
        }
    }
}

fn hi(val: i32) -> i32 {
    ((val as u32) >> 12) as i32
}
fn lo(val: i32) -> i32 {
    val & ((1 << 12) - 1)
}

const fn fib(mut n: i32) -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 0;
    n -= 1;

    loop {
        n -= 1;
        if n < 0 {
            return a;
        }

        let c = a + b;
        b = a;
        a = c;
    }
}

const _: () = {
    assert!(fib(1) == 1);
    assert!(fib(2) == 1);
    assert!(fib(3) == 2);
    assert!(fib(4) == 3);
    assert!(fib(5) == 5);
    assert!(fib(6) == 8);
};

fn main() {
    let instructions = instr!(
        li x10, (1<<31);

        li x11, 1;
        li x12, 0;

        addi x10, x10, -1;

        label "loop";

        addi x10, x10, -1;
        blt x10, x0, "end";

        add x13, x11, x12;
        mv x12, x11;
        mv x11, x13;
        bge x0, x0, "loop";

        label "end";
        ebreak;
    );

    for i in &instructions {
        let i: &dyn Instruction = i.deref();
        if let Some(l) = (i as &dyn Any).downcast_ref::<Label>() {
            println!("{}", l.name);
        }
    }

    let mut stdin = io::stdin().lock().lines();

    let mut core = CoreState::new(instructions.leak());

    loop {
        // core.print();
        // stdin.next();
        core.run_step();
        if core.stopped {
            // core.print();
            println!("ebreak");
            break;
        }
    }

    println!("{}", X11::load(&core));
}

struct CoreState {
    registers: [i32; 31],
    stopped: bool,
    pc: i32,
    instructions: &'static [Box<dyn Instruction>],
}

impl CoreState {
    pub fn new(instructions: &'static [Box<dyn Instruction>]) -> Self {
        Self {
            registers: [0; 31],
            stopped: false,
            pc: 0,
            instructions,
        }
    }
    #[rustfmt::skip]
    pub fn print(&self) {
        const WIDTH: usize = 4;

        println!("Core State\n");

        let  print_reg = |n: i32, r: i32| {
            print!("  {n:02}: {:#010X}", r)
        };

        let registers = [
             X0::load(self),  X1::load(self), X2::load(self),  X3::load(self),
             X4::load(self),  X5::load(self), X6::load(self),  X7::load(self),
             X8::load(self),  X9::load(self),X10::load(self), X11::load(self),
            X12::load(self), X13::load(self),X14::load(self), X15::load(self),
            X16::load(self), X17::load(self),X18::load(self), X19::load(self),
            X20::load(self), X21::load(self),X22::load(self), X23::load(self),
            X24::load(self), X25::load(self),X26::load(self), X27::load(self),
            X28::load(self), X29::load(self),X30::load(self), X31::load(self),
        ];

        for (n, r) in registers.into_iter().enumerate() {
            print_reg(n as i32, r);
            if (n + WIDTH + 1) % WIDTH == 0 {
                println!()
            }
        }

        println!();
    }
    pub fn run_step(&mut self) {
        let instruction = &self.instructions[self.pc as u32 as usize];
        instruction.run(self);
        self.pc += 1;
    }
    pub fn jump(&mut self, label: &str) {
        let dest = self.instructions.iter().position(|i| {
            let i = i.deref() as &dyn Any;
            i.downcast_ref::<Label>().is_some_and(|l| l.name == label)
        });

        match dest {
            Some(n) => self.pc = n as i32,
            None => panic!("Unknown label: {label}"),
        }
    }
}
