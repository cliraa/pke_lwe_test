use lambdaworks_math::field::element::FieldElement;
fn main () {

    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField; 

    const ORDER: u64 = 97; // Defining modulo
    type F = U64PrimeField<ORDER>;
    type FE = FieldElement<F>;

    let sample = vec! [(18), (5), (8), (13), (11)];

    println!("Sampling: {:?}", sample);

    let mut u = FE::zero();
    let mut v = FE::zero();

    let s = FE::new(5);
    let q: u64 = 97;
    let message: u64 = 0;

    let mut b: Vec<FE> = Vec::new();

    let a = vec! [
        FE::new(80), FE::new(86), 
        FE::new(19), FE::new(62), 
        FE::new(2), FE::new(83), 
        FE::new(25), FE::new(47), 
        FE::new(20), FE::new(58), 
        FE::new(45), FE::new(15), 
        FE::new(30), FE::new(68), 
        FE::new(4), FE::new(13), 
        FE::new(8), FE::new(6), 
        FE::new(42), FE::new(92)];

    let e = vec! [
        FE::new(3), FE::new(3), 
        FE::new(4), FE::new(1), 
        FE::new(3), FE::new(3), 
        FE::new(4), FE::new(4), 
        FE::new(1), FE::new(4), 
        FE::new(3), FE::new(3), 
        FE::new(2), FE::new(2), 
        FE::new(3), FE::new(2), 
        FE::new(4), FE::new(4), 
        FE::new(1), FE::new(3)];

    for x in 0..a.len() {
        b.push(a[x] * s + e[x]);
    }
    
    println!("b: {:?}", b);

    for &x in &sample {
        u += a[x];
        v += b[x];        
    }

    v = (v.representative() + (q / 2) * message).into();

        println!("u: {:?}", u);
        println!("v: {:?}", v);

    let res = v - s * u;
    println!("res: {:?}", res);

    let comp = FE::new(49);

    let resi = res.representative();
    let compi = comp.representative();

    if resi >= (compi) {
        println!("Message is a 1");
    } else {
        println!("Message is a 0");
    }

}
