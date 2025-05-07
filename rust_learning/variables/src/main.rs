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
	let smile = '🥵';
        let tuple_of_tuples: (i32, f64, u8) = (500, 6.4, 1);
	let tup = (500, 6.4, 1);
	let (_a, _b, _c) = tup;
        let q: (i32, f64, u8) = (800, 7.3, 9);
        let _eight_hundred = q.0;
        let _seven_point_three = q.1;
        let _nine = q.2;
	let h = [1, 2, 3, 4, 5];
	let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

	let jay = [10, 20, 3, 4, 5];
	let first = jay[0];
	let second = jay[1];
	


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
	println!("tuple of tuples: {:?}", tuple_of_tuples);
	println!("The value of b is: {_b}");
	println!("The value of q is: {_seven_point_three}");
	println!("The value of h is: {h:?}");
	println!("The value of months is: {months:?}");
	println!("The value of months[1] is: {}", months[0]);
	println!("The value of months[2] is: {}", months[1]);
	println!("The value of months[3] is: {}", months[2]);
	println!("The value of months[4] is: {}", months[3]);
	println!("The value of months[5] is: {}", months[4]);
	println!("The value of months[6] is: {}", months[5]);			
	println!("The value of months[7] is: {}", months[6]);			
	println!("The value of months[8] is: {}", months[7]);			
	println!("The value of months[9] is: {}", months[8]);			
	println!("The value of months[10] is: {}", months[9]);			
	println!("The value of months[11] is: {}", months[10]);			
	println!("The value of months[12] is: {}", months[11]);			

	println!("The value of first is: {first}");
	println!("The value of second is: {second}");

	    
    }
