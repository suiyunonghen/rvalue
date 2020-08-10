use std::{io,fs,fmt,result};
use std::io::Read;
use std::fs::read;
use crate::dxvalue::DxValue;

pub enum errJsonReson{
    JET_NoObjBack,           //缺少}
    JET_NoArrBack,           //缺少]
    JET_NoKeyStart,
    JET_NoStrStart,
    JET_NoStrEnd,
    JET_NoKVSplit,
    JET_NoValueSplit,
    JET_Invalidate,
    JET_UnParse
}

impl fmt::Debug for errJsonReson{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            errJsonReson::JET_UnParse=>f.write_str("JET_UnParse"),
            errJsonReson::JET_Invalidate=>f.write_str("JET_Invalidate"),
            errJsonReson::JET_NoArrBack=>f.write_str("JET_NoArrBack"),
            errJsonReson::JET_NoKeyStart=>f.write_str("JET_NoKeyStart"),
            errJsonReson::JET_NoKVSplit=>f.write_str("JET_NoKVSplit"),
            errJsonReson::JET_NoObjBack=>f.write_str("JET_NoObjBack"),
            errJsonReson::JET_NoStrEnd=>f.write_str("JET_NoObjBack"),
            errJsonReson::JET_NoStrStart=>f.write_str("JET_NoObjBack"),
            errJsonReson::JET_NoValueSplit=>f.write_str("JET_NoValueSplit"),
        }
    }
}


pub struct errorJson{
    errPos: isize,
    reson: errJsonReson
}

impl fmt::Debug for errorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("errPos",&self.errPos)
            .field("reson",&self.reson)
            .finish()
    }
}

pub type Result = std::result::Result<DxValue, errorJson>;

pub fn loadFromFile(file: &str)->Result{
    let f = fs::File::open(file);
    match f {
        io::Result::Ok(mut fhandle)=>{
            let mut buf: Vec<u8> = Vec::new();
            if let Ok(_) = fhandle.read_to_end(&mut buf){
                return parseJson(buf)
            }
        },
        io::Result::Err(e)=>{
            return result::Result::Err(errorJson{errPos:0,reson: errJsonReson::JET_UnParse});
        }
    }
    result::Result::Err(errorJson{errPos:0,reson: errJsonReson::JET_UnParse})
}

fn skipWB<'a>(b: &'a[u8])->(&'a [u8],usize){
    if b.len() == 0 || b[0] > 0x20{
        return (b,0);
    }
    if b.len() == 0 || b[0] != 0x20 && b[0] != 0x0A && b[0] != 0x09 && b[0] != 0x0D {
        return (b,0);
    }
    for i in 1..b.len()-1{
        if b[i] != 0x20 && b[i] != 0x0A && b[i] != 0x09 && b[i] != 0x0D {
            return (&b[i..],i-1);
        }
    }
    (b,0)
}

pub fn parseJson(buf: Vec<u8>)->Result{
    //先判定一下是否有BOM头
    let mut realdata = &buf[..];
    if buf.len() > 2 && buf[0] == 0xEF && buf[1] == 0xBB && buf[2] == 0xBF{ //BOM
        realdata = &buf[3..];
    }
    let (buf,skiplen) = skipWB(&buf);

    Result::Err(errorJson{errPos:0,reson: errJsonReson::JET_Invalidate})
}