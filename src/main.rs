use std::io;

fn add(x : i32, y : i32) -> i32 {
	x + y
}

fn sub(x : i32, y : i32) -> i32 {
	x - y
}

fn mul(x : i32, y : i32) -> i32 {
	x * y
}

fn div(x : i32, y : i32) -> i32 {
	x / y
}

fn mdl(x : i32, y : i32) -> i32 {
	x % y
}

fn pwr(x : i32, y : i32) -> i32 {
	let mut sum : i32 = 1;
	let mut cnt : i32 = y;
	loop{
		sum = sum * x;
		cnt = cnt - 1;
		if cnt == 0 { break; }
	}
	sum
}

fn main() {
    println!("Sample calculator, written by Midoo in Rust.");
	loop{
		let mut decision = String::new();
		let mut _a = String::new();
		let mut _b = String::new();
		println!("\nOperator (+, -, *, /, %, ^), or 0 to exit:");
		std::io::stdin().read_line(&mut decision).ok()
			.expect("Failed to read line");
		if decision.trim() == "0"{
			break;
		}
		std::io::stdin().read_line(&mut _a).ok()
			.expect("Failed to read line");
		std::io::stdin().read_line(&mut _b).ok()
			.expect("Failed to read line");
		let a = _a.trim().parse::<i32>().expect("invalid input");
		let b = _b.trim().parse::<i32>().expect("invalid input");
		let result : i32 = match decision.trim(){
			"+" => add(a, b),
			"-" => sub(a, b),
			"*" => mul(a, b),
			"/" => div(a, b),
			"%" => mdl(a, b),
			"^" => pwr(a, b),
			_ => break,
		};
		println!("{} {} {} = {}", a, decision.trim(), b, result);
	}
}
