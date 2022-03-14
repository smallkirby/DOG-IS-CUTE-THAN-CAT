use thiserror::Error;

#[derive(Error, Debug)]
pub enum DogCompileError {
  #[error("Invalid imm value: {0}")]
  InvalidImm(String),
  #[error("Invalid opecode: {0}")]
  InvalidOpc(String),
  #[error("Invalid line: {0}")]
  InvalidLine(String),
}

#[derive(Error, Debug)]
pub enum DogRuntimeError {
  #[error("Invalid Instruction: pc={0}")]
  InvalidInstruction(u64),
  #[error("Pop request when stack is empty")]
  EmptyStackError(),
  #[error("Invalid register dereference: {0}")]
  InvalidRegisterError(u64),
  #[error("PC out of range: {0}")]
  InvalidPcError(u64),
}

#[derive(Debug)]
struct DogRegs {
  srg: u64,
  r1: u64,
  r2: u64,
  r3: u64,
  r4: u64,
  r5: u64,
  pc: u64,
}

impl DogRegs {
  pub fn new() -> Self {
    Self {
      srg: 0,
      r1: 0,
      r2: 0,
      r3: 0,
      r4: 0,
      r5: 0,
      pc: 0,
    }
  }
}

impl DogRegs {
  pub fn mov(&mut self, src: u64, dst: u64) {
    let val = match src {
      0 => self.srg,
      1 => self.r1,
      2 => self.r2,
      3 => self.r3,
      4 => self.r4,
      5 => self.r5,
      _ => panic!("{:?}", DogRuntimeError::InvalidRegisterError(src)),
    };
    match dst {
      0 => self.srg = val,
      1 => self.r1 = val,
      2 => self.r2 = val,
      3 => self.r3 = val,
      4 => self.r4 = val,
      5 => self.r5 = val,
      _ => panic!("{:?}", DogRuntimeError::InvalidRegisterError(dst)),
    };
  }
}

#[derive(Debug, Clone, Copy)]
enum DogInst {
  Push(u64),
  Pop(u64),
  Add(u64),
  Mul(u64),
  Print(u64),
  Input(u64),
  Jmp(u64),
  Mov(u64, u64),
  Exit(),
  Invalid,
}

impl DogInst {
  fn parse_line(line: &str) -> Result<Self, DogCompileError> {
    let parts: Vec<&str> = line.split('🐱').collect();
    if parts.len() != 2 {
      return Err(DogCompileError::InvalidLine(line.into()));
    }
    let opc_str = parts[0];
    let opr_str = parts[1];
    if !opr_str.chars().all(|c| c == '🐕') {
      return Err(DogCompileError::InvalidImm(opr_str.into()));
    }
    let opr = opr_str.chars().count() as u64;

    Ok(match opc_str {
      "🐕" => Self::Push(opr),
      "🐶" => Self::Pop(opr),
      "🐕🐕" => Self::Add(opr),
      "🐕🐶🐕" => Self::Mul(opr),
      "🐶🐶🐶" => Self::Print(opr),
      "🐶🐶🐶🐶" => Self::Input(opr),
      "🐶🐶🐶🐕🐶" => Self::Jmp(opr),
      "🐕🐶" => {
        let src = opr & 0xFF00;
        let dst = opr & 0x00FF;
        Self::Mov(src, dst)
      }
      "🐕🐕🐕🐕🐕🐕🐕🐕🐕🐕" => Self::Exit(),
      _ => Self::Invalid,
    })
  }

  pub fn parse(prog: &str) -> Vec<Self> {
    let lines = prog.split('\n');
    let mut insts = vec![];

    for line in lines {
      match Self::parse_line(line) {
        Ok(inst) => insts.push(inst),
        Err(e) => panic!("{e:?}"),
      }
    }

    insts
  }
}

#[derive(Debug)]
struct DogVm {
  regs: DogRegs,
  stack: Vec<u64>,
  insts: Vec<DogInst>,
}

impl DogVm {
  pub fn new() -> Self {
    Self {
      regs: DogRegs::new(),
      stack: Vec::new(),
      insts: vec![],
    }
  }

  pub fn load(&mut self, prog: String) {
    self.insts = DogInst::parse(&prog);
  }

  fn run_one(&mut self, inst: &DogInst) {
    match inst {
      DogInst::Push(imm) => {
        self.stack.push(*imm);
      }
      DogInst::Pop(imm) => {
        for _ in 0..*imm {
          self.regs.srg = match self.stack.pop() {
            Some(v) => v,
            None => panic!("{:?}", DogRuntimeError::EmptyStackError()),
          };
        }
      }
      DogInst::Add(imm) => {
        self.stack.push(self.regs.r1 + *imm);
      }
      DogInst::Mul(imm) => {
        self.stack.push(self.regs.r1 * *imm);
      }
      DogInst::Print(imm) => {
        let mut s: Vec<u8> = vec![];
        for _ in 0..*imm {
          self.regs.srg = match self.stack.pop() {
            Some(v) => v,
            None => panic!("{:?}", DogRuntimeError::EmptyStackError()),
          };
          let c = self.regs.srg as u8;
          s.push(c);
        }
        print!("{}", String::from_utf8(s).unwrap());
      }
      DogInst::Input(_) => {
        unimplemented!()
      }
      DogInst::Jmp(imm) => {
        self.regs.pc = imm - 1;
      }
      DogInst::Mov(src, dst) => {
        self.regs.mov(*src, *dst);
      }
      DogInst::Exit() => {
        std::process::exit(0);
      }
      DogInst::Invalid => {
        panic!("{:?}", DogRuntimeError::InvalidInstruction(self.regs.pc));
      }
    }
    self.regs.pc += 1;
  }

  pub fn run(&mut self) {
    loop {
      if self.regs.pc >= self.insts.len() as u64 {
        panic!("{:?}", DogRuntimeError::InvalidPcError(self.regs.pc));
      }
      let inst = self.insts[self.regs.pc as usize];
      self.run_one(&inst);
    }
  }
}

fn read_lines() -> String {
  let mut lines: Vec<String> = vec![];
  loop {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
      Ok(n) => {
        if n == 0 {
          break;
        } else {
          lines.push(line.trim().into());
        }
      }
      Err(e) => panic!("{e:?}"),
    };
  }

  lines.join("\n")
}

fn main() {
  let mut vm = DogVm::new();
  let prog = read_lines();
  vm.load(prog);
  vm.run();
}
