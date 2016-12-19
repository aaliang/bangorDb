pub mod RowFormat {

    use std::{mem, slice};
    use std::slice::Iter;
    use std::iter::Chain;
    use std::io::{Read, Write, Error};

    use tokio_core::io;
    //use futures::Future;
    use futures::future::Future;

    //pub fn to_bytes<'a> (data: &'a [u8]) -> Chain<Iter<'a, u8>, Iter<'a, u8>> {
        //let len = data.len();
        //let len_ptr: *const usize = &len;
        //let cast_bytes: *const u8 =  len_ptr as _;
        //let sz = unsafe {
            //slice::from_raw_parts(
                //cast_bytes,
                //mem::size_of::<usize>()
                //)
        //};
        //sz.iter().chain(data)
    //}

    //pub fn write_row<'a, W> (data: &'a [u8], writer: W) where W: Write {
        //let b = to_bytes(data);
        //writer.write(b);
    //}


    // if we name the type (write all), we can drop the Box and save the runtime allocation
    // that seems unnecessary at this stage.
    pub fn write_row<'a, W> (data: &'a [u8], mut writer: W) -> Box<Future<Item=(), Error=Error>> where W: Write + 'static {
        let len = data.len();
        if len > u32::max_value() as usize {
            panic!("data too large");
        }
        let len_ptr: *const u32 = &(len as u32);
        let cast_bytes: *const u8 =  len_ptr as _;
        let sz = unsafe {
            slice::from_raw_parts(cast_bytes,
                mem::size_of::<u32>())
        };

        // kind of unnecessary to allocate
        let x = sz.iter().chain(data).map(|b|*b).collect::<Vec<u8>>();


        Box::new(
            io::write_all(writer, x).map(|_| ())
            )

    }


    fn as_u32_be(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 24) |
        ((array[1] as u32) << 16) |
        ((array[2] as u32) <<  8) |
        ((array[3] as u32) <<  0)
    }


    // wtf... this is a shitshow
    pub fn read_row<'a, R>(r: R, sz: &'static mut [u8; 4]) -> Box<Future<Item=Vec<u8>, Error=Error>> where R: Read + Send + 'static {
        let future = io::read_exact(r, sz)
            .and_then(|(_r, _sz)| {
                let sz_u32 = as_u32_be(_sz);
                let mut buf = Vec::with_capacity(sz_u32 as usize);
                io::read_exact(_r, buf)
            })
            .map(|(a, b)|b);
        future.boxed()
    }
}

pub struct WriteEv {
    
}
