// let intervals: Vec<f64> = vec![
//     1_f64,
//     65537_f64 / 65536_f64,
//     4375_f64 / 4374_f64,
//     2401_f64 / 2400_f64,
//     (2_f64).powf(1_f64 / 1200_f64),
//     (2_f64).powf(1_f64 / 1000_f64),
//     32805_f64 / 32768_f64,
//     3_f64 / (2_f64 / ((2_f64).powf(7_f64 / 12_f64))),
//     (10_f64).powf(1_f64 / 1000_f64),
//     225_f64 / 224_f64,
//     15625_f64 / 15552_f64,
//     2109375_f64 / 2097152_f64,
//     160_f64 / 159_f64,
//     145_f64 / 144_f64,
//     (2_f64).powf(1_f64 / 96_f64),
//     1728_f64 / 1715_f64,
//     129_f64 / 128_f64,
//     126_f64 / 125_f64,
//     121_f64 / 120_f64,
//     (2_f64).powf(1_f64 / 72_f64),
//     96_f64 / 95_f64,
//     2048_f64 / 2025_f64,
//     81_f64 / 80_f64,
//     (2_f64).powf(1_f64 / 53_f64),
//     531441_f64 / 524288_f64,
//     (2_f64).powf(1_f64 / 48_f64),
//     65_f64 / 64_f64,
//     64_f64 / 63_f64,
//     (2_f64).powf(1_f64 / 41_f64),
//     56_f64 / 55_f64,
//     (2_f64).powf(1_f64 / 36_f64),
//     51_f64 / 50_f64,
//     50_f64 / 49_f64,
//     49_f64 / 48_f64,
//     46_f64 / 45_f64,
//     (2_f64).powf(1_f64 / 31_f64),
//     45_f64 / 44_f64,
//     (2_f64).powf(1_f64 / 30_f64),
//     128_f64 / 125_f64,
//     42_f64 / 41_f64,
//     41_f64 / 40_f64,
//     40_f64 / 39_f64,
//     39_f64 / 38_f64,
//     38_f64 / 37_f64,
//     37_f64 / 36_f64,
//     36_f64 / 35_f64,
//     246_f64 / 239_f64,
//     (2_f64).powf(1_f64 / 24_f64),
//     35_f64 / 34_f64,
//     59049_f64 / 57344_f64,
//     34_f64 / 33_f64,
//     33_f64 / 32_f64,
//     32_f64 / 31_f64,
//     529_f64 / 512_f64,
//     31_f64 / 30_f64,
//     30_f64 / 29_f64,
//     29_f64 / 28_f64,
//     28_f64 / 27_f64,
//     (3_f64 / 2_f64).powf(1_f64 / 11_f64),
//     27_f64 / 26_f64,
//     133_f64 / 128_f64,
//     (2_f64).powf(1_f64 / 18_f64),
//     26_f64 / 25_f64,
//     25_f64 / 24_f64,
//     24_f64 / 23_f64,
//     (2_f64).powf(1_f64 / 16_f64),
//     23_f64 / 22_f64,
//     (3_f64 / 2_f64).powf(1_f64 / 9_f64),
//     67_f64 / 64_f64,
//     22_f64 / 21_f64,
//     21_f64 / 20_f64,
//     20_f64 / 19_f64,
//     256_f64 / 243_f64,
//     135_f64 / 128_f64,
//     19_f64 / 18_f64,
//     128_f64 / 121_f64,
//     18_f64 / 17_f64,
//     (2_f64).powf(1_f64 / 12_f64),
//     17_f64 / 16_f64,
//     5_f64.powf(1_f64 / 25_f64),
//     16_f64 / 15_f64,
//     2187_f64 / 2048_f64,
//     (18_f64 / 5_f64).powf(1_f64 / 19_f64),
//     15_f64 / 14_f64,
//     (2_f64).powf(5_f64 / 48_f64),
//     14_f64 / 13_f64,
//     69_f64 / 64_f64,
//     27_f64 / 25_f64,
//     (2_f64).powf(1_f64 / 9_f64),
//     13_f64 / 12_f64,
//     (2_f64).powf(3_f64 / 24_f64),
//     12_f64 / 11_f64,
//     35_f64 / 32_f64,
//     800_f64 / 729_f64,
//     11_f64 / 10_f64,
//     (2_f64).powf(1_f64 / 7_f64),
//     (2_f64).powf(7_f64 / 48_f64),
//     71_f64 / 64_f64,
//     65536_f64 / 59049_f64,
//     10_f64 / 9_f64,
//     (2_f64).powf(2_f64 / 12_f64),
//     9_f64 / 8_f64,
//     145_f64 / 128_f64,
//     256_f64 / 225_f64,
//     (2_f64).powf(3_f64 / 16_f64),
//     73_f64 / 64_f64,
//     8_f64 / 7_f64,
//     (2_f64).powf(1_f64 / 5_f64),
//     15_f64 / 13_f64,
//     (2_f64).powf(5_f64 / 24_f64),
//     37_f64 / 32_f64,
//     125_f64 / 108_f64,
//     64_f64 / 55_f64,
//     7_f64 / 6_f64,
//     299_f64 / 256_f64,
//     75_f64 / 64_f64,
//     (2_f64).powf(11_f64 / 48_f64),
//     13_f64 / 11_f64,
//     32_f64 / 27_f64,
//     19_f64 / 16_f64,
//     (2_f64).powf(3_f64 / 12_f64),
//     25_f64 / 21_f64,
//     6_f64 / (5_f64/(81_f64 / 80_f64).powf(1_f64 / 4_f64)),
//     (3_f64 / 2_f64).powf(4_f64 / 9_f64),
//     6_f64 / 5_f64,
//     19683_f64 / 16384_f64,
//     77_f64 / 64_f64,
//     (2_f64).powf(13_f64 / 48_f64),
//     17_f64 / 14_f64,
//     243_f64 / 200_f64,
//     39_f64 / 32_f64,
//     (2_f64).powf(2_f64 / 7_f64),
//     128_f64 / 105_f64,
//     11_f64 / 9_f64,
//     (2_f64).powf(7_f64 / 24_f64),
//     27_f64 / 22_f64,
//     16_f64 / 13_f64,
//     79_f64 / 64_f64,
//     100_f64 / 81_f64,
//     (2_f64).powf(5_f64 / 16_f64),
//     8192_f64 / 6561_f64,
//     5_f64 / 4_f64,
//     161_f64 / 128_f64,
//     (2_f64).powf(4_f64 / 12_f64),
//     323_f64 / 256_f64,
//     81_f64 / 64_f64,
//     14_f64 / 11_f64,
//     (2_f64).powf(17_f64 / 48_f64),
//     32_f64 / 25_f64,
//     41_f64 / 32_f64,
//     9_f64 / 7_f64,
//     128_f64 / 99_f64,
//     (2_f64).powf(9_f64 / 24_f64),
//     83_f64 / 64_f64,
//     13_f64 / 10_f64,
//     125_f64 / 96_f64,
//     64_f64 / 49_f64,
//     21_f64 / 16_f64,
//     (2_f64).powf(19_f64 / 48_f64),
//     675_f64 / 512_f64,
//     (2_f64).powf(2_f64 / 5_f64),
//     85_f64 / 64_f64,
//     4_f64 / 3_f64,
//     (2_f64).powf(5_f64 / 12_f64),
//     171_f64 / 128_f64,
//     (3_f64 / 2_f64).powf(8_f64 / 11_f64),
//     43_f64 / 32_f64,
//     (2_f64).powf(3_f64 / 7_f64),
//     27_f64 / 20_f64,
//     177147_f64 / 131072_f64,
//     (2_f64).powf(7_f64 / 16_f64),
//     87_f64 / 64_f64,
//     15_f64 / 11_f64,
//     (2_f64).powf(11_f64 / 24_f64),
//     11_f64 / 8_f64,
//     18_f64 / 13_f64,
//     25_f64 / 18_f64,
//     89_f64 / 64_f64,
//     (2_f64).powf(23_f64 / 48_f64),
//     7_f64 / 5_f64,
//     1024_f64 / 729_f64,
//     45_f64 / 32_f64,
//     361_f64 / 256_f64,
//     (2_f64).powf(1_f64 / 2_f64),
//     91_f64 / 64_f64,
//     64_f64 / 45_f64,
//     729_f64 / 512_f64,
//     10_f64 / 7_f64,
//     (2_f64).powf(25_f64 / 48_f64),
//     23_f64 / 16_f64,
//     36_f64 / 25_f64,
//     93_f64 / 64_f64,
//     16_f64 / 11_f64,
//     (2_f64).powf(13_f64 / 24_f64),
//     47_f64 / 32_f64,
//     (2_f64).powf(9_f64 / 16_f64),
//     262144_f64 / 177147_f64,
//     40_f64 / 27_f64,
//     95_f64 / 64_f64,
//     12167_f64 / 8192_f64,
//     (2_f64).powf(4_f64 / 7_f64),
//     3_f64 / (2_f64/(81_f64 / 80_f64).powf(1_f64 / 2_f64)),
//     3_f64 / (2_f64/(81_f64 / 80_f64).powf(1_f64 / 3_f64)),
//     3_f64 / (2_f64/(81_f64 / 80_f64).powf(2_f64 / 7_f64)),
//     3_f64 / (2_f64/(81_f64 / 80_f64).powf(1_f64 / 4_f64)),
//     3_f64 / (2_f64/(81_f64 / 80_f64).powf(1_f64 / 5_f64)),
//     3_f64 / (2_f64/(81_f64 / 80_f64).powf(1_f64 / 6_f64)),
//     (2_f64).powf(7_f64 / 12_f64),
//     (2_f64).powf(31_f64 / 53_f64),
//     3_f64 / 2_f64,
//     (2_f64).powf(24_f64 / 41_f64),
//     (2_f64).powf(17_f64 / 29_f64),
//     97_f64 / 64_f64,
//     (2_f64).powf(3_f64 / 5_f64),
//     1024_f64 / 675_f64,
//     (2_f64).powf(29_f64 / 48_f64),
//     32_f64 / 21_f64,
//     391_f64 / 256_f64,
//     49_f64 / 32_f64,
//     192_f64 / 125_f64,
//     (2_f64).powf(15_f64 / 24_f64),
//     99_f64 / 64_f64,
//     14_f64 / 9_f64,
//     25_f64 / 16_f64,
//     (2_f64).powf(31_f64 / 48_f64),
//     std::f64::consts::PI / 2_f64,
//     11_f64 / 7_f64,
//     101_f64 / 64_f64,
//     128_f64 / 81_f64,
//     203_f64 / 128_f64,
//     (2_f64).powf(8_f64 / 12_f64),
//     51_f64 / 32_f64,
//     8_f64 / 5_f64,
//     6561_f64 / 4096_f64,
//     103_f64 / 64_f64,
//     (2_f64).powf(11_f64 / 16_f64),
//     207_f64 / 128_f64,
//     ((5_f64).powf(1_f64 / 2_f64) + 1_f64) / 2_f64,
//     81_f64 / 50_f64,
//     13_f64 / 8_f64,
//     209_f64 / 128_f64,
//     (2_f64).powf(17_f64 / 24_f64),
//     18_f64 / 11_f64,
//     105_f64 / 64_f64,
//     (2_f64).powf(5_f64 / 7_f64),
//     400_f64 / 243_f64,
//     53_f64 / 32_f64,
//     (2_f64).powf(35_f64 / 48_f64),
//     128_f64 / 77_f64,
//     32768_f64 / 19683_f64,
//     5_f64 / 3_f64,
//     107_f64 / 64_f64,
//     6859_f64 / 4096_f64,
//     (2_f64).powf(9_f64 / 12_f64),
//     32_f64 / 19_f64,
//     27_f64 / 16_f64,
//     109_f64 / 64_f64,
//     (2_f64).powf(37_f64 / 48_f64),
//     128_f64 / 75_f64,
//     437_f64 / 256_f64,
//     12_f64 / 7_f64,
//     55_f64 / 32_f64,
//     (2_f64).powf(19_f64 / 24_f64),
//     111_f64 / 64_f64,
//     125_f64 / 72_f64,
//     (3_f64 / 2_f64).powf(15_f64 / 11_f64),
//     (2_f64).powf(4_f64 / 5_f64),
//     7_f64 / 4_f64,
//     (2_f64).powf(13_f64 / 16_f64),
//     225_f64 / 128_f64,
//     113_f64 / 64_f64,
//     16_f64 / 9_f64,
//     57_f64 / 32_f64,
//     (2_f64).powf(10_f64 / 12_f64),
//     115_f64 / 64_f64,
//     9_f64 / 5_f64,
//     59049_f64 / 32768_f64,
//     (2_f64).powf(41_f64 / 48_f64),
//     (2_f64).powf(6_f64 / 7_f64),
//     29_f64 / 16_f64,
//     20_f64 / 11_f64,
//     729_f64 / 400_f64,
//     117_f64 / 64_f64,
//     64_f64 / 35_f64,
//     11_f64 / 6_f64,
//     (2_f64).powf(21_f64 / 24_f64),
//     59_f64 / 32_f64,
//     50_f64 / 27_f64,
//     13_f64 / 7_f64,
//     119_f64 / 64_f64,
//     (2_f64).powf(43_f64 / 48_f64),
//     4096_f64 / 2187_f64,
//     15_f64 / 8_f64,
//     32_f64 / 17_f64,
//     (2_f64).powf(11_f64 / 12_f64),
//     121_f64 / 64_f64,
//     256_f64 / 135_f64,
//     243_f64 / 128_f64,
//     61_f64 / 32_f64,
//     (2_f64).powf(15_f64 / 16_f64),
//     48_f64 / 25_f64,
//     123_f64 / 64_f64,
//     27_f64 / 14_f64,
//     247_f64 / 128_f64,
//     31_f64 / 16_f64,
//     64_f64 / 33_f64,
//     (2_f64).powf(23_f64 / 24_f64),
//     35_f64 / 18_f64,
//     125_f64 / 64_f64,
//     63_f64 / 32_f64,
//     (2_f64).powf(47_f64 / 48_f64),
//     160_f64 / 81_f64,
//     253_f64 / 128_f64,
//     127_f64 / 64_f64,
//     2_f64
// ];