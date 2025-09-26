use std::time::{Duration, Instant};

use ark_bn254::Bn254;
use ark_ff::UniformRand;
use criterion::{criterion_group, criterion_main, Criterion};
use jf_plonk::{nightfall::FFTPlonk, proof_system::UniversalSNARK, transcript::StandardTranscript};

use jf_relation::{Arithmetization, Circuit, PlonkCircuit};
use nf_curves::ed_on_bn254::Fq as Fr254;

use jf_primitives::circuit::poseidon::PoseidonHashGadget;
use jf_primitives::pcs::prelude::UnivariateKzgPCS;
fn bench_poseidon_hash_gadget_criterion(
    c: &mut Criterion,
    circuit: &mut PlonkCircuit<Fr254>,
    i: usize,
) {
    let mut rng = jf_utils::test_rng();
    let mut inputs: Vec<Fr254> = vec![Fr254::from(1)];
    for _ in 1..i {
        let rand = Fr254::rand(&mut rng);
        inputs.push(rand);
    }

    let start = Instant::now();
    let inputs_vars = inputs
        .iter()
        .map(|&x| circuit.create_variable(x).unwrap())
        .collect::<Vec<_>>();
    circuit.poseidon_hash(&inputs_vars).ok();
    // Constraint count
    println!(
        "Poseidon : {} constraints before padding",
        circuit.num_gates()
    );

    circuit.finalize_for_arithmetization().unwrap();
    println!("Poseidon witness time:{} ms", start.elapsed().as_millis());

    // Constraint count
    println!(
        "Poseidon : {} constraints after padding",
        circuit.num_gates()
    );

    // Srs generation time
    c.bench_function("Poseidon Srs generation time:", |b| {
        b.iter(|| {
            let srs_size = circuit.srs_size().unwrap();
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(srs_size, &mut rng)
                .unwrap();
        })
    });

    let srs_size = circuit.srs_size().unwrap();
    let srs = FFTPlonk::<UnivariateKzgPCS<Bn254>>::universal_setup_for_testing(srs_size, &mut rng)
        .unwrap();

    let (pk, vk) = FFTPlonk::<UnivariateKzgPCS<Bn254>>::preprocess(&srs, None, circuit).unwrap();

    //Proving time
    c.bench_function("Poseidon Proving time:", |b| {
        b.iter(|| {
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::prove::<_, _, StandardTranscript>(
                &mut rng, circuit, &pk, None,
            )
            .unwrap();
        })
    });
    let proof = FFTPlonk::<UnivariateKzgPCS<Bn254>>::prove::<_, _, StandardTranscript>(
        &mut rng, circuit, &pk, None,
    )
    .unwrap();
    // Verification time
    c.bench_function("Poseidon Verification time:", |b| {
        b.iter(|| {
            FFTPlonk::<UnivariateKzgPCS<Bn254>>::verify::<StandardTranscript>(
                &vk,
                &[],
                &proof,
                None,
            )
            .unwrap();
        })
    });
}

fn bench_poseidon_hash_gadget_criterion_main(c: &mut Criterion) {
    for i in 1..7 {
        println!("The number of inputs:{}", i + 1);
        let mut circuit_turbo: PlonkCircuit<Fr254> = PlonkCircuit::new_turbo_plonk();
        bench_poseidon_hash_gadget_criterion(c, &mut circuit_turbo, i);
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(2)).warm_up_time(Duration::from_secs(1));
    targets = bench_poseidon_hash_gadget_criterion_main
}
criterion_main!(benches);
