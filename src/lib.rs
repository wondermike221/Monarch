use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, anyhow};
use image::{ImageBuffer, Rgba};

#[derive(Debug, StructOpt)]
pub enum Monarch {
    /// Encode a secret message
    Encode(Encode),
    /// Decode a secret message
    Decode(Decode)
}

#[derive(Debug, StructOpt)]
pub struct Encode {
    /// Image to be modified
    #[structopt(parse(from_os_str))]
    pub image: PathBuf,

    /// Message to hide
    // short and long flags (-m, --message) will be deduced from the field's name
    #[structopt(short, long)]
    pub message: String,

    /// Output file (overwrites original if omitted)
    #[structopt(parse(from_os_str), short)]
    pub outfile: Option<PathBuf>
}

#[derive(Debug, StructOpt)]
pub struct Decode {
    /// Image to read from
    #[structopt(parse(from_os_str))]
    pub image: PathBuf,
}

impl Encode {
    pub fn run(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let img_as_rgba = image::open(&self.image)?.to_rgba();
        let (width, height) = img_as_rgba.dimensions();
        let bytes = width * height;
        let msg_as_bytes = self.message.as_bytes();

        if msg_as_bytes.len() > bytes as usize {
            return Err(anyhow!("Message is too large for image size"));
        }

        let mut output = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

        for (x, y, px) in img_as_rgba.enumerate_pixels() {
            let mut tmp_px = px.clone();

            let input_index = x + (y * width);
            if input_index < msg_as_bytes.len() as u32 {
                tmp_px.0[3] = msg_as_bytes[input_index as usize];
            }

            output.put_pixel(x, y, tmp_px.clone());
        }
        Ok(output)
    }
}

impl Decode {
    pub fn run(&self) -> Result<String> {
        let img_as_rgba = image::open(&self.image)?.to_rgba();

        let mut output: Vec<u8> = Vec::new();

        for(_, _, px) in img_as_rgba.enumerate_pixels() {
            output.push(px.0[3]);
        }

        // if alpha is empty filter it out
        let output = output.into_iter()
                                    .filter(|b| {
                                        *b != 0xff_u8
                                    })
                                    .collect::<Vec<u8>>();

        // convert back to string
        let output = String::from_utf8(output)?;

        Ok(output)
    }
}

