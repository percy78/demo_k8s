fn main() {
	let sum = 5 + 10;
    
        let difference = 95.5 - 4.3;

	let product = 4 * 30;
	let quotient = 56.7 / 32.2;
	let remainder = 43 % 5;
	let t = true;
	let c = 'z';
	let z: char = 'ℤ'; // with explicit type annotation
	let f: bool = false; // with explicit type annotation
	let x = 5;
	let heart_eyed_cat = '😻';
	let smile = ':)';
    
	{
	    let x = x * 2;
	    println!("The value of x in the inner scope is: {x}");
	}
    
	println!("The value of sum is: {sum}");
	println!("The value of difference is: {difference}");
	println!("The value of quotient is: {quotient}");
	println!("The value of product is: {product}");
	println!("The value of reminder is: {remainder}");
	println!("The value of t is: {t}");
	println!("The value of f is: {f}");
	println!("The value of z is: {c}");
	println!("The value of z: is: {z:}");
	println!("The value of hear eyed cat is: {heart_eyed_cat}");
	println!("The value of smile is: {smile}");
	    
    }
