fn main() {
    // 001
    // Multiples of 3 and 5
    // If we list all the natural numbers below 10 that are multiples of 3 or 5, 
    // we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    println!("pe_001 ans: {}", pe_001(1_000));

    // 002
    // Even Fibonacci numbers
    // Each new term in the Fibonacci sequence is generated by adding the previous two terms. 
    // By starting with 1 and 2, the first 10 terms will be:
    // 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    // By considering the terms in the Fibonacci sequence whose values do not exceed four million, 
    // find the sum of the even-valued terms.
    println!("pe_002 ans: {}", pe_002(4_000_000));  
    
    // 003
    // The prime factors of 13195 are 5, 7, 13 and 29.
    // What is the largest prime factor of the number 600851475143 ?
    println!("pe_003 ans: {}", pe_003(600_851_475_143));

    // 004
    // A palindromic number reads the same both ways. The largest palindrome made from the product 
    // of two 2-digit numbers is 9009 = 91 × 99.
    // Find the largest palindrome made from the product of two 3-digit numbers.
    println!("pe_004 ans: {}", pe_004());

    // 005
    // 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 
    // without any remainder.
    // What is the smallest positive number that is evenly divisible by 
    // all of the numbers from 1 to 20?
    println!("pe_005 ans: {}", pe_005());

    // 006
    // The sum of the squares of the first ten natural numbers is,
    // 1^2+2^2+...+10^2=385
    // The square of the sum of the first ten natural numbers is,
    // (1+2+...+10)^2=55^2=3025
    // Hence the difference between the sum of the squares of the first ten natural numbers and 
    // the square of the sum is 3025−385=2640.
    // Find the difference between the sum of the squares of the first one hundred natural numbers 
    // and the square of the sum.
    println!("pe_006 ans: {}", pe_006(100));

    // 007
    // By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, 
    // we can see that the 6th prime is 13.
    // What is the 10 001st prime number?
    println!("pe_007 ans: {}", pe_007(10_001));

    // 008
    // The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
    let num = "73167176531330624919225119674426574742355349194934\
               96983520312774506326239578318016984801869478851843\
               85861560789112949495459501737958331952853208805511\
               12540698747158523863050715693290963295227443043557\
               66896648950445244523161731856403098711121722383113\
               62229893423380308135336276614282806444486645238749\
               30358907296290491560440772390713810515859307960866\
               70172427121883998797908792274921901699720888093776\
               65727333001053367881220235421809751254540594752243\
               52584907711670556013604839586446706324415722155397\
               53697817977846174064955149290862569321978468622482\
               83972241375657056057490261407972968652414535100474\
               82166370484403199890008895243450658541227588666881\
               16427171479924442928230863465674813919123162824586\
               17866458359124566529476545682848912883142607690042\
               24219022671055626321111109370544217506941658960408\
               84580156166097919133875499200524063689912560717606\
               05886116467109405077541002256983155200055935729725\
               71636269561882670428252483600823257530420752963450";
    // Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. 
    // What is the value of this product?
    println!("pe_008 ans: {}", pe_008(num, 13));

    // 009
    // A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
    // a^2 + b^2 = c^2
    // For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
    // There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    // Find the product abc.
    println!("pe_009 ans: {}", pe_009(1000));

    // 010
    // The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    // Find the sum of all the primes below two million.
    println!("pe_010 ans: {}", pe_010(2_000_000));


}


fn pe_001(n: u32) -> u32 {  
    let mut sum = 0;
    for n in 0..n {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n
        }
    };
    sum
}


fn pe_002(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 1;
    let mut tmp;
    loop {
        tmp = fibonacci(i);
        if tmp >= n { break };

        if tmp % 2 == 0 { sum += tmp };
        i += 1;
    };
    sum
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n-2) + fibonacci(n-1)
    }
}


fn pe_003(n: u64) -> u64 {
    let mut devidend = n;
    let mut divisor = 2;
    let mut lpf = 0;
    while devidend > 1 {
        if devidend % divisor == 0 {
            lpf = divisor;
            devidend /= divisor;
        }
        else {
            divisor += 1
        }
    }
    lpf
}


fn pe_004 () -> u64 {
    let mut n = 999;
    let mut palindromic_number: u64 = 999999;
    while n >= 100 {        
        // 6 桁の双子数を作るために 3 桁の数字を文字列化→逆転→元の文字列と結合→数値に戻す を行う
        let s = n.to_string();
        let r = s.chars().rev().collect::<String>();
        let s = format!("{}{}", s, r);
        palindromic_number = s.parse().unwrap();

        let mut divisor = 999;
        while divisor >= 100 {
            if palindromic_number % divisor == 0 {
                let quatient = palindromic_number / divisor;
                if (quatient >= 100) && (quatient < 1000) {
                    n = 0;
                    break
                }
            }
            divisor -= 1;
        };
        n -= 1;
    };
    palindromic_number
}


use std::collections::HashMap;

fn pe_005() -> u64 {
    let mut map = HashMap::new();
    let mut n = 2;
    while n <= 20 {
        let mut devidend = n;
        let mut divisor: u64 = 2;
        let mut c = 0;
        while devidend > 1 {
            if devidend % divisor == 0 {
                c += 1;
                let count = map.entry(divisor).or_insert(0);
                if c > *count {
                    *count = c;
                }                
                devidend /= divisor;
            }
            else {
                c = 0;
                divisor += 1
            }
        }
        n += 1;
    };
    
    let mut ret = 1;
    for (k, v) in map {
        // println!("{}: {}", k, v);
        ret *= k.pow(v);
    };
    ret
}


fn pe_006(n: u64) -> u64 {
    let mut sum_of_nums = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum_of_nums += i;
        sum_of_squares += i.pow(2);
    };
    sum_of_nums.pow(2) - sum_of_squares
}


fn pe_007(n: u64) -> u64 {
    let mut z = 1;
    let mut _c = 0;
    while _c <= n {
        if is_pn(z) { 
            _c += 1; 
            if _c == n { break };
        };
        z += 1;
    };
    z
}


fn is_pn(n: u64) -> bool{
    let mut ret = true;
    if n <= 1 {
        ret = false;
    } else if n == 2 {
        ret = true;
    } else if n % 2 == 0 {
        ret = false;
    } else {
        let n_ = n as f64;
        let lim = n_.sqrt() as u64;
        
        for i in (2..=lim+1).filter(|&x| x % 2 != 0) {
            if n % i == 0 {
                ret = false;
                break
            };
        };
    };
    // println!("{}: {}", n, ret);
    ret
}


use std::cmp::max;

fn pe_008 (num: &str, len: usize) -> u64 {
    let num = num.to_string().chars().collect::<String>();
    let mut i = 0;    
    let mut m_product = 0;
    while i <= num.len()-len {
        let mut product = 1;
        for n in num[i..i+len].to_string().chars() { 
            if n == '0' { break };
            let n: u64 = n.to_string().parse().unwrap();
            product *= n;
        };
        m_product = max(product, m_product);
        i += 1;
    };
    m_product
}


fn pe_009(num: u64) -> u64 {
    let mut ans = 0;
    for a in 1..=num {        
        for b in a+1..=num-a {
            let c = num - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                ans = a*b*c;
                break
            };
        };
    };
    ans
}


fn pe_010(num: u64) -> u64 {
    let mut i = 2;
    let mut sum = 0;
    while i < num {
        if is_pn(i) {sum += i}
        i += 1
    };
    sum
}