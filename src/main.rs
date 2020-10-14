use monarch::Monarch;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()>{
    let res = run()?;
    Ok(res)
}


fn run() -> Result<()> {
    let opt = Monarch::from_args();
    // println!("{:?}", opt);

    match opt {
        Monarch::Encode(enc) => {
            let output = enc.run()?;
            output.save(enc.outfile.unwrap_or(enc.image))?;
            println!("Successfully encoded the message!")
        },
        Monarch::Decode(dec) => {
            let output = dec.run()?;
            println!("{}", output);
        }
    }

    Ok(())
}
