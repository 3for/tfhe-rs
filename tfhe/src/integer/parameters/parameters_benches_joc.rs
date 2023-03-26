pub use crate::core_crypto::commons::dispersion::{DispersionParameter, StandardDev};
pub use crate::core_crypto::commons::parameters::{
    DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension, PolynomialSize,
};
use crate::shortint::parameters::{CarryModulus, MessageModulus};
use crate::shortint::Parameters;

pub const ID_1_RADIX_16_BITS_16_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(615),
    glwe_dimension: GlweDimension(4),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.00009380341682666086),
    glwe_modular_std_dev: StandardDev( 0.0000000000000003162026630747649),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(2),
};
pub const ID_2_RADIX_16_BITS_8_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(702),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.000018916438292526045),
    glwe_modular_std_dev: StandardDev( 0.0000000000000003162026630747649),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(4),
    ks_level: DecompositionLevelCount(7),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(4),
};
pub const ID_3_CRT_16_BITS_5_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(872),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(4096),
    lwe_modular_std_dev: StandardDev(0.0000008244869530752798),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(22),
    pbs_level: DecompositionLevelCount(1),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(4),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(4),
};
pub const ID_4_RADIX_32_BITS_32_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(667),
    glwe_dimension: GlweDimension(6),
    polynomial_size: PolynomialSize(256),
    lwe_modular_std_dev: StandardDev(0.00003604103581022737),
    glwe_modular_std_dev: StandardDev(0.000000000003953518398797519),
    pbs_base_log: DecompositionBaseLog(18),
    pbs_level: DecompositionLevelCount(1),
    ks_level: DecompositionLevelCount(3),
    ks_base_log: DecompositionBaseLog(4),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(2),
};
pub const ID_5_RADIX_32_BITS_16_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(784),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.000004174399189990001),
    glwe_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    pbs_base_log: DecompositionBaseLog(23),
    pbs_level: DecompositionLevelCount(1),
    ks_level: DecompositionLevelCount(3),
    ks_base_log: DecompositionBaseLog(4),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(4),
};
pub const ID_6_RADIX_32_BITS_8_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(983),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(16384),
    lwe_modular_std_dev: StandardDev(0.00000010595830454427828),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(4),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(16),
};

pub const ID_6_CRT_32_BITS_6_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(983),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(16384),
    lwe_modular_std_dev: StandardDev(0.00000010595830454427828),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(4),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(4),
};

pub const ID_7_RADIX_16_BITS_16_BLOCKS_WOPBS: Parameters = Parameters {
    lwe_dimension: LweDimension(549),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0003177104139262535),
    glwe_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(17),
    pfks_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    cbs_level: DecompositionLevelCount(1),
    cbs_base_log: DecompositionBaseLog(13),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(2),
};
pub const ID_8_RADIX_16_BITS_8_BLOCKS_WOPBS: Parameters = Parameters {
    lwe_dimension: LweDimension(534),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0004192214045106218),
    glwe_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(17),
    pfks_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    cbs_level: DecompositionLevelCount(2),
    cbs_base_log: DecompositionBaseLog(9),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(4),
};
pub const ID_9_CRT_16_BITS_5_BLOCKS_WOPBS: Parameters = Parameters {
    lwe_dimension: LweDimension(538),
    glwe_dimension: GlweDimension(4),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00038844554870845634),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(4),
    pbs_level: DecompositionLevelCount(11),
    ks_level: DecompositionLevelCount(10),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(20),
    pfks_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(7),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(4),
};
pub const ID_10_NATIF_CRT_16_BITS_5_BLOCKS_WOPBS: Parameters = Parameters {
    lwe_dimension: LweDimension(696),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00002113509320237618),
    glwe_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(4),
    ks_level: DecompositionLevelCount(7),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(17),
    pfks_modular_std_dev: StandardDev(0.0000000000000003162026630747649),
    cbs_level: DecompositionLevelCount(3),
    cbs_base_log: DecompositionBaseLog(7),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};
pub const ID_11_NATIF_CRT_32_BITS_6_BLOCKS_WOPBS: Parameters = Parameters {
    lwe_dimension: LweDimension(791),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(4096),
    lwe_modular_std_dev: StandardDev(0.000003659302213002263),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(3),
    pbs_level: DecompositionLevelCount(14),
    ks_level: DecompositionLevelCount(16),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(20),
    pfks_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(5),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(1),
};

    pub const ID_12_HYBRID_CRT_32_bits: Parameters = Parameters {
    lwe_dimension: LweDimension(838),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(4096),
    lwe_modular_std_dev: StandardDev(0.0000015398206356719045),
    glwe_modular_std_dev: StandardDev(0.0000000000000000002168404344971009),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(3),
    pfks_level: DecompositionLevelCount(0),
    pfks_base_log: DecompositionBaseLog(0),
    pfks_modular_std_dev: StandardDev(0.0),
    cbs_level: DecompositionLevelCount(0),
    cbs_base_log: DecompositionBaseLog(0),
    message_modulus: MessageModulus(8),
    carry_modulus: CarryModulus(8),
    };


pub const TEST_WOPBS: Parameters = Parameters {
    lwe_dimension: LweDimension(10),
    glwe_dimension: GlweDimension(1),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0000000000000000000004168323308734758),
    glwe_modular_std_dev: StandardDev(0.00000000000000000000000000000000000004905643852600863),
    pbs_base_log: DecompositionBaseLog(7),
    pbs_level: DecompositionLevelCount(6),
    ks_base_log: DecompositionBaseLog(1),
    ks_level: DecompositionLevelCount(14),
    pfks_level: DecompositionLevelCount(6),
    pfks_base_log: DecompositionBaseLog(7),
    pfks_modular_std_dev: StandardDev(0.000000000000000000000000000000000000004905643852600863),
    cbs_level: DecompositionLevelCount(7),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};