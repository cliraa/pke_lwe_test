#[cfg(test)]
mod tests {

    use lambdaworks_math::field::element::FieldElement;
    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField; 

    const ORDER: u64 = 97; // Defining modulo
    type F = U64PrimeField<ORDER>;
    type FE = FieldElement<F>;

    fn fun (a: u64) -> u64 {

        let sample = vec! [(18), (5), (8), (13), (11)];

        println!("Sampling: {:?}", sample);

        let mut u = FE::zero();
        let mut v = FE::zero();

        let s = FE::new(5);
        let q: u64 = 97;
        let message: u64 = a;

        let mut b: Vec<FE> = Vec::new();

        let a = vec! [
            FE::new(47), FE::new(27), 
            FE::new(72), FE::new(89), 
            FE::new(19), FE::new(64), 
            FE::new(26), FE::new(79), 
            FE::new(68), FE::new(83), 
            FE::new(1), FE::new(76), 
            FE::new(25), FE::new(65), 
            FE::new(55), FE::new(82), 
            FE::new(85), FE::new(14), 
            FE::new(92), FE::new(5)];

        let e = vec! [
            FE::new(4), FE::new(3), 
            FE::new(4), FE::new(4), 
            FE::new(2), FE::new(4), 
            FE::new(2), FE::new(3), 
            FE::new(4), FE::new(4), 
            FE::new(2), FE::new(3), 
            FE::new(2), FE::new(4), 
            FE::new(2), FE::new(2), 
            FE::new(4), FE::new(1), 
            FE::new(2), FE::new(2)];

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
            let ver: u64 = 1;
            return ver;
        } else {
            let ver: u64 = 0;
            return ver;
        }        
    }

    #[test]
    fn test_message_1() {
        let che = fun(1);
        assert_eq!(che, 1);
    }

    #[test]
    fn test_message_0() {
        let che = fun(0);
        assert_eq!(che, 0);
    }
}
