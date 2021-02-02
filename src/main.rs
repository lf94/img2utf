use std::{
  fs::File,
  env,
};

/*
Need to construct a mask and translation table.
Match from "heaviest" to "lightest".

[1,1,1]
[1,1,1] → 🮋
[1,1,1]

[1,1,1]
[0,0,0] → 🬂
[0,0,0]

etc...

This implementation may not use 3x3 mask or those exact characters, but
something more suitable.
*/

const linetext: [char; 42] = [
  '█',
  '🭪',
  '🭨',
  '🭖',
  '🭡',
  '🭐',
  '🭅',
  '🭒',
  '🭝',
  '🭌',
  '🭁',

  
  '🬎',
  '🬹',
  '▋',
  '🮉',
  
  '🮇',
  '🮂',
  '▍',
  '▂',

  '🭲',
  '🭹',
  '╋',

  '◢',
  '◥',
  '◢',
  '◥',
  
  '🬏',
  '🬾',
  '🭙',
  '🬀',
  '🬁',
  '🭤',
  '🭉',
  '🬞',
  
  '╺',
  '╹',
  '╸',
  '╻',
  
  '🭢',
  '🭗',
  '🬼',
  '🭇',
  
];

const masks: [[[u8; 3]; 3]; 42] = [
  [
    [0,0,0],
    [0,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [0,0,1],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,0,0],
    [1,0,0],
  ],
  [
    [0,0,0],
    [0,0,1],
    [0,0,1],
  ],
  [
    [0,0,1],
    [0,0,1],
    [0,0,0],
  ],
  [
    [1,0,0],
    [1,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [0,0,0],
    [1,0,0],
  ],
  [
    [0,0,0],
    [0,0,0],
    [0,0,1],
  ],
  [
    [0,0,1],
    [0,0,0],
    [0,0,0],
  ],
  [
    [1,0,0],
    [0,0,0],
    [0,0,0],
  ],

  [
    [0,0,0],
    [0,0,0],
    [1,1,1],
  ],
  [
    [1,1,1],
    [0,0,0],
    [0,0,0],
  ],
  [
    [0,0,1],
    [0,0,1],
    [0,0,1],
  ],
  [
    [1,0,0],
    [1,0,0],
    [1,0,0],
  ],

  [
    [1,1,0],
    [1,1,0],
    [1,1,0],
  ],
  [
    [0,0,0],
    [1,1,1],
    [1,1,1],
  ],
  [
    [0,1,1],
    [0,1,1],
    [0,1,1],
  ],
  [
    [1,1,1],
    [1,1,1],
    [0,0,0],
  ],
  
  [
    [1,1,1],
    [0,0,0],
    [1,1,1],
  ],
  [
    [1,0,1],
    [1,0,1],
    [1,0,1],
  ],
  [
    [1,0,1],
    [0,0,0],
    [1,0,1],
  ],

  
  [
    [1,0,0],
    [0,0,0],
    [0,0,1],
  ],
  [
    [0,0,1],
    [0,0,0],
    [1,0,0],
  ],
  [
    [1,1,0],
    [1,0,0],
    [0,0,0],
  ],
  [
    [0,0,0],
    [1,0,0],
    [1,1,0],
  ],
  
  [
    [1,1,1],
    [1,1,1],
    [0,0,1],
  ],
  [
    [1,1,1],
    [0,1,1],
    [0,1,1],
  ],
  [
    [0,1,1],
    [0,1,1],
    [1,1,1],
  ],
  [
    [0,0,1],
    [1,1,1],
    [1,1,1],
  ],
  [
    [1,0,0],
    [1,1,1],
    [1,1,1],
  ],
  [
    [1,1,0],
    [1,1,0],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,1,0],
    [1,1,0],
  ],
  [
    [1,1,1],
    [1,1,1],
    [1,0,0],
  ],

  
  [
    [1,1,1],
    [1,0,0],
    [1,1,1],
  ],
  [
    [1,0,1],
    [1,0,1],
    [1,1,1],
  ],
  [
    [1,1,1],
    [0,0,1],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,0,1],
    [1,0,1],
  ],
  
  [
    [1,1,0],
    [1,1,1],
    [1,1,1],
  ],
  [
    [0,1,1],
    [1,1,1],
    [1,1,1],
  ],
  [
    [1,1,1],
    [1,1,1],
    [0,1,1],
  ],
  [
    [1,1,1],
    [1,1,1],
    [1,1,0],
  ],
];

/*
  Turns an array of RGBA (8 bits per channel) bytes into an array of 0s and 1s.
*/
fn pixels_to_bitplane(buf: &[u8], width: u32, height: u32) -> Vec<Vec<u8>> {
  let mut nbuf: Vec<Vec<u8>> = vec![];
  
  for y in 0..height {
    let mut line = vec![];
    for x in 0..width {
      let idx = ((y * (width * 4)) + (x * 4)) as usize;
      if idx >= buf.len() { break; }
      
      line.push(buf[idx] / 255);
    }
    nbuf.push(line);
  }

  nbuf
}

fn dot_matrix_print(buf: Vec<Vec<u8>>) {
  for line in buf {
    for dot in line {
      print!("{}", if dot == 0 {
        " "
      } else {
        "0"
      });
    }
    println!("");
  }
}

fn bitplane_to_linetext(buf: Vec<Vec<u8>>, width: u32, height: u32) {
  let mut shifted = 0;
  let mut x = 0;
  let mut y = 0;
  let mut printed_match = false;

  while true {
    for (idx, matrix) in masks.iter().enumerate() {
      if buf[y + 0][x] == matrix[0][0] && buf[y + 0][x + 1] == matrix[0][1] && buf[y + 0][x + 2] == matrix[0][2]
      && buf[y + 1][x] == matrix[1][0] && buf[y + 1][x + 1] == matrix[1][1] && buf[y + 1][x + 2] == matrix[1][2]
      && buf[y + 2][x] == matrix[2][0] && buf[y + 2][x + 1] == matrix[2][1] && buf[y + 2][x + 2] == matrix[2][2] {
        print!("{}", linetext[idx]);
        printed_match = true;
        break;
      }
    }

    // Track how many pixels we have moved to the right
    if !printed_match {
      x += 1;
      shifted += 1;
    } else {
      printed_match = false;
      x += (3 - shifted);
      shifted = 0;
    }

    //
    // We have moved enough to cover a whole mask space
    // so print a missing pattern character.
    // Basically for debugging. Change the " " to be a "?" or whatever you want.
    // Set to " " as the default for missing patterns.
    // 
    if shifted >= 3 {
      print!(" ");
      //println!("{}:{}", x, y);
      shifted = 0;
    }

    // Know when to print a newline
    if x >= (width as usize) - 3 {
      x = 0;
      y += 3;
      println!("");
    }
    if y >= (height as usize) - 3 {
      break;
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let decoder = png::Decoder::new(File::open(&args[1]).unwrap());
  let (info, mut reader) = decoder.read_info().unwrap();
  let mut buf = vec![0; info.buffer_size()];
  reader.next_frame(&mut buf).unwrap();
  let bitplane = pixels_to_bitplane(&buf, info.width, info.height);
  bitplane_to_linetext(bitplane, info.width, info.height);
}
