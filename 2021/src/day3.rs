use std::io::{self,BufRead};

fn main() {
    let stdin = io::stdin();
    let v : Vec<Vec<u32>> = stdin
	.lock()
	.lines()
	.map(|s|
	     s.unwrap()
	     .chars()
	     .filter_map( |ch| ch.to_digit(10))
	     .collect::<Vec<u32>>()
	)
	.collect();

    let n : u32 = v.len().try_into().unwrap();
    let half : u32 = n / 2;
    let vlen : usize = v[0].len();

    let mut counts : Vec<u32> = Vec::new();
    counts.resize(vlen, 0);

    v.iter().for_each(|vv| {
	vv.iter()
	    .enumerate()
	    .for_each(|(i, val)| {
		counts[i] += val;
	    });
    });

    let bits : Vec<u32> = counts.iter()
	.map(|x| if x > &half { 1 } else { 0 })
	.collect();

    let gamma : u32 = bits.iter().fold(0, |acc, val| 2 * acc + val);
    let epsilon : u32 = bits.iter().fold(0, |acc, val| 2 * acc + (!*val & 0x1));

    println!("gamma: {}\tepsilon: {}\ttotal: {}",
	     gamma, epsilon, gamma * epsilon);

    let mut o2_bits : Vec<u32> = Vec::new();
    (0..vlen).for_each(|idx| {
	let mut count = 0;
	let mut sum = 0;
	v.iter()
	    .filter(|vv|
		    o2_bits
		    .iter()
		    .enumerate()
		    .all(|(bit,val)| vv[bit] == *val)
	    )
	    .for_each(|vv| {
		count += 1;
		sum += vv[idx];
	    });
	o2_bits.push(if sum >= (count-sum) {1} else {0});
    });

    let o2_rating = v.iter()
	.filter(|vv|
		o2_bits
		.iter()
		.enumerate()
		.all(|(bit,val)| vv[bit] == *val))
	.last()
	.unwrap()
	.iter()
	.fold(0, |acc, val| 2 * acc + val);


    let mut co2_bits : Vec<u32> = Vec::new();
    (0..vlen).for_each(|idx| {
	let count : u32 = v.iter()
	    .filter(|vv| co2_bits.iter().enumerate()
		    .all(|(bit,val)| vv[bit] == *val))
	    .count().try_into().unwrap();
	if 1 < count {
	    let sum : u32 = v.iter()
		.filter(|vv| co2_bits.iter().enumerate()
			.all(|(bit,val)| vv[bit] == *val))
		.map(|vv| vv[idx])
		.sum();
	    co2_bits.push(if sum < (count-sum) {1} else {0});
	}
    });

    let co2_rating = v.iter()
	.filter(|vv|
		co2_bits
		.iter()
		.enumerate()
		.all(|(bit,val)| vv[bit] == *val))
	.last()
	.unwrap()
	.iter()
	.fold(0, |acc, val| 2 * acc + val);


    println!("o2_rating: {}\tco2_rating: {}, total: {}",
	     o2_rating, co2_rating, o2_rating * co2_rating);

}
