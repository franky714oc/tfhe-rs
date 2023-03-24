use serde::Serialize;
use std::fs;
use std::path::PathBuf;
#[cfg(feature = "boolean")]
use tfhe::boolean::parameters::BooleanParameters;
use tfhe::core_crypto::prelude::*;
#[cfg(feature = "shortint")]
use tfhe::shortint::Parameters;

#[derive(Clone, Serialize)]
pub struct CryptoParametersRecord {
    lwe_dimension: LweDimension,
    glwe_dimension: GlweDimension,
    polynomial_size: PolynomialSize,
    lwe_modular_std_dev: StandardDev,
    glwe_modular_std_dev: StandardDev,
    pbs_base_log: DecompositionBaseLog,
    pbs_level: DecompositionLevelCount,
    ks_base_log: DecompositionBaseLog,
    ks_level: DecompositionLevelCount,
    pfks_level: Option<DecompositionLevelCount>,
    pfks_base_log: Option<DecompositionBaseLog>,
    pfks_modular_std_dev: Option<StandardDev>,
    cbs_level: Option<DecompositionLevelCount>,
    cbs_base_log: Option<DecompositionBaseLog>,
    message_modulus: Option<usize>,
    carry_modulus: Option<usize>,
}

#[cfg(feature = "boolean")]
impl From<BooleanParameters> for CryptoParametersRecord {
    fn from(params: BooleanParameters) -> Self {
        CryptoParametersRecord {
            lwe_dimension: params.lwe_dimension,
            glwe_dimension: params.glwe_dimension,
            polynomial_size: params.polynomial_size,
            lwe_modular_std_dev: params.lwe_modular_std_dev,
            glwe_modular_std_dev: params.glwe_modular_std_dev,
            pbs_base_log: params.pbs_base_log,
            pbs_level: params.pbs_level,
            ks_base_log: params.ks_base_log,
            ks_level: params.ks_level,
            pfks_level: None,
            pfks_base_log: None,
            pfks_modular_std_dev: None,
            cbs_level: None,
            cbs_base_log: None,
            message_modulus: None,
            carry_modulus: None,
        }
    }
}

#[cfg(feature = "shortint")]
impl From<Parameters> for CryptoParametersRecord {
    fn from(params: Parameters) -> Self {
        CryptoParametersRecord {
            lwe_dimension: params.lwe_dimension,
            glwe_dimension: params.glwe_dimension,
            polynomial_size: params.polynomial_size,
            lwe_modular_std_dev: params.lwe_modular_std_dev,
            glwe_modular_std_dev: params.glwe_modular_std_dev,
            pbs_base_log: params.pbs_base_log,
            pbs_level: params.pbs_level,
            ks_base_log: params.ks_base_log,
            ks_level: params.ks_level,
            pfks_level: Some(params.pfks_level),
            pfks_base_log: Some(params.pfks_base_log),
            pfks_modular_std_dev: Some(params.pfks_modular_std_dev),
            cbs_level: Some(params.cbs_level),
            cbs_base_log: Some(params.cbs_base_log),
            message_modulus: Some(params.message_modulus.0),
            carry_modulus: Some(params.carry_modulus.0),
        }
    }
}

#[derive(Serialize)]
enum PolynomialMultiplication {
    Fft,
    // Ntt,
}

#[derive(Serialize)]
enum IntegerRepresentation {
    Radix,
    // Crt,
    // Hybrid,
}

#[derive(Serialize)]
enum ExecutionType {
    Sequential,
    Parallel,
}

#[derive(Serialize)]
enum KeySetType {
    Single,
    // Multi,
}

#[derive(Serialize)]
enum OperandType {
    CipherText,
    PlainText,
}

#[derive(Serialize)]
struct BenchmarkParametersRecord {
    display_name: String,
    crypto_parameters_alias: String,
    crypto_parameters: CryptoParametersRecord,
    message_modulus: Option<usize>,
    carry_modulus: Option<usize>,
    ciphertext_modulus: usize,
    polynomial_multiplication: PolynomialMultiplication,
    precision: u32,
    error_probability: f64,
    integer_representation: IntegerRepresentation,
    decomposition_basis: u32,
    pbs_algorithm: Option<String>,
    execution_type: ExecutionType,
    key_set_type: KeySetType,
    operand_type: OperandType,
}

/// Writes benchmarks parameters to disk in JSON format.
pub fn write_to_json<T: Into<CryptoParametersRecord>>(
    bench_id: &str,
    params: T,
    params_alias: impl Into<String>,
    display_name: impl Into<String>,
) {
    let params = params.into();

    let execution_type = match bench_id.contains("parallelized") {
        true => ExecutionType::Parallel,
        false => ExecutionType::Sequential,
    };
    let operand_type = match bench_id.contains("scalar") {
        true => OperandType::PlainText,
        false => OperandType::CipherText,
    };

    let record = BenchmarkParametersRecord {
        display_name: display_name.into(),
        crypto_parameters_alias: params_alias.into(),
        crypto_parameters: params.to_owned(),
        message_modulus: params.message_modulus,
        carry_modulus: params.carry_modulus,
        ciphertext_modulus: 64,
        polynomial_multiplication: PolynomialMultiplication::Fft,
        precision: (params.message_modulus.unwrap_or(2) as u32).ilog2(),
        error_probability: 2f64.powf(-41.0),
        integer_representation: IntegerRepresentation::Radix,
        decomposition_basis: (params.message_modulus.unwrap_or(2) as u32).ilog2(),
        pbs_algorithm: None, // To be added in future version
        execution_type,
        key_set_type: KeySetType::Single,
        operand_type,
    };

    let mut params_directory = ["benchmarks_parameters", bench_id]
        .iter()
        .collect::<PathBuf>();
    fs::create_dir_all(&params_directory).unwrap();
    params_directory.push("parameters.json");

    fs::write(params_directory, serde_json::to_string(&record).unwrap()).unwrap();
}

// Empty main to please clippy.
#[allow(dead_code)]
pub fn main() {}
