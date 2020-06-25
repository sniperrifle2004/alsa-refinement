use alsa::{Direction, ValueOr, Output};
use alsa::pcm::{PCM, HwParams, Format, Access};

fn main() {
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    dump_params(&hwp);
    try_refine(&hwp, |hwp| hwp.set_access(Access::RWInterleaved)).unwrap();
    try_refine(&hwp, |hwp| hwp.set_format(Format::s16())).unwrap();
    try_refine(&hwp, |hwp| hwp.set_channels(2)).unwrap();
    try_refine(&hwp, |hwp| hwp.set_rate(44_100, ValueOr::Nearest)).unwrap();
    try_refine(&hwp, |hwp| hwp.set_period_size_near(1024, ValueOr::Nearest)).unwrap();
    try_refine(&hwp, |hwp| hwp.set_buffer_size_near(4096)).unwrap();
    println!("Refinement succesful")
}

fn dump_params(params: &HwParams) {
    println!("---------------");
    let mut o = Output::buffer_open().unwrap();
    params.dump(&mut o).unwrap();
    print!("{}", o);
}

fn try_refine<R, F>
    (params: &HwParams, f: F) -> Result<(), String>
    where R: std::fmt::Debug, F: FnOnce(&HwParams) -> alsa::Result<R> {
    let r = f(params);
    dump_params(params);
    match r {
        Ok(_) => Ok(()),
        e => {
            println!("{:?}", e);
            Err("Refinement failed".to_string())
        }
    }
}
