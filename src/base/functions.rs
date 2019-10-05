use std::collections::HashMap;

pub fn xor(x: &Vec<u8>, y: &Vec<u8>) -> Vec<u8> {
    use std::cmp::max;
    let size = max(x.len(), y.len());
    let mut i = 0;
    let mut n = vec![];
    while i < size {
        n.push(x[i % x.len()] ^ y[i % y.len()]);
        i += 1;
    }
    n
}

pub fn hamming(x: &Vec<u8>, y: &Vec<u8>) -> u32 {
    xor(x,y).iter().map(|&x| x.count_ones()).sum()
}

// Standard deviation from letter frequency in the English language
pub fn english_std_dev(x: &Vec<u8>) -> f64 {
    // Create a table to look up letter frequency in the English language (most common letters)
    // Source: https://www.data-compression.com/english.html
    let mut letter_frequency:HashMap<char,f64> = HashMap::with_capacity(27);
    letter_frequency.insert('e',10.41442);
    letter_frequency.insert('t', 7.29357);
    letter_frequency.insert('a', 6.51738);
    letter_frequency.insert('o', 5.96302);
    letter_frequency.insert('i', 5.58094);
    letter_frequency.insert('n', 5.64513);
    letter_frequency.insert(' ',19.18182);
/*
    letter_frequency.insert('s', 5.15760);
    letter_frequency.insert('h', 4.92888);
    letter_frequency.insert('r', 4.97563);
    letter_frequency.insert('d', 3.49835);
    letter_frequency.insert('l', 3.31490);
    letter_frequency.insert('u', 2.25134);
    letter_frequency.insert('c', 2.17339);

    letter_frequency.insert('b', 1.24248);
    letter_frequency.insert('f', 1.97881);
    letter_frequency.insert('g', 1.58610);
    letter_frequency.insert('j', 0.09033);
    letter_frequency.insert('k', 0.50529);
    letter_frequency.insert('m', 2.02124);
    letter_frequency.insert('p', 1.37645);
    letter_frequency.insert('q', 0.08606);
    letter_frequency.insert('v', 0.82903);
    letter_frequency.insert('w', 1.71272);
    letter_frequency.insert('x', 0.13692);
    letter_frequency.insert('y', 1.45984);
    letter_frequency.insert('z', 0.07836);
*/
    let mut std_dev = 0.0;
    for (letter, frequency) in &letter_frequency {

        // Letter frequency in the decrypted bytes
        let self_frequency = 100.0 * x.iter().filter(|&&a| a == *letter as u8).count() as f64 / x.len() as f64;
        std_dev += (*frequency - self_frequency).powi(2) / letter_frequency.len() as f64;

    };

    std_dev
}

// Count the occurrence of each byte. X: byte, Y: number of occurrences
fn full_bar_plot(x: &Vec<u8>) -> Vec<usize> {

    // Allocate vector entry for all 256 bytes
    let mut bytes_counted: Vec<usize> = vec![0; 256];

    // Count the occurences
    for &b in x.iter() {
        bytes_counted[b as usize] += 1;
    }

    bytes_counted
}

// Count the occurrence of each byte and put them in descending order.
pub fn occurrence_map(x: &Vec<u8>) -> Vec<(usize, usize)> {

    // Get the occurrence of each byte
    let plot = full_bar_plot(&x);

    // Put them in a Vec<(x,y)> where X is the byte, Y is  the occurrence. Cut the ones where there's no occurrence.
    let mut plot_map = plot.iter().enumerate().map(|(index,&item)| (index,item)).filter(|(_, item)| *item != 0).collect::<Vec<(usize,usize)>>();

    // Sort it by the occurrence
    plot_map.sort_by(|&(_,x), &(_,y) | y.cmp(&x));

    plot_map
}
