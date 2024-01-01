use std::io;

fn main() {
    println!("Enter: ");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Failed to read");
    let length:u32 = match length.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Failed to parse");
            0 as u32
        }
    };
    // _solid_cube(length);
    // _increasing_cube(length);
    // _triangle(length);
    // _increasing_triangle(length);
    increasing_triangle_2(length);
}

fn _solid_cube(l: u32) -> i32 {
    for _i in 0..l {
        for _j in 0..l {
            print!("#")
        }
        println!("")
    }
    return 0;
}


// for l = 4 => 
//  #
//  # #
//  # # #
//  # # # #
fn _triangle(l: u32) -> i32 {
    for i in 1..l+1 {
        for _j in 0..i {
            print!("# ")
        }
        println!("")
    }
    return 0;
}


// for l = 4 => 
// 1 2 3 4
// 5 6 7 8
// 9 10 11 12
// 13 14 15 16
fn _increasing_cube(l: u32) -> i32 {
    for i in 0..l {
        for j in 0..l {
            print!("{} ", j + (l * i) + 1)
        }
        println!("")
    }
    return 0;
}


// for l = 4 => 
//  1
//  2 3
//  4 5 6
//  7 8 9 10
fn _increasing_triangle(l: u32) -> i32 {
    let mut c = 0;
    for i in 1..l+1 {
        for _j in 0..i {
            c += 1;
            print!("{c} ")
        }
        println!("")
    }
    return 0;
}
// for l = 4 => 
//  1
//  2 3
//  4 5 6
//  7 8 9 10
fn increasing_triangle_2(l: u32) -> i32 {
    for i in 1..l+1 {
        let start = (i * (i - 1 )) / 2 + 1;
        for j in 0..i {
            print!("{} ", start + j )
        }
        println!("")
    }
    return 0;
}

