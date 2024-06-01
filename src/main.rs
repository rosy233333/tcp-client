use std::time::{Duration, Instant};
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::{thread, time};

mod stats;
use rand::rngs::SmallRng;
use rand::*;
use stats::*;

// const LISTENER: &str = "0.0.0.0:5556";
const DEST: &str = "127.0.0.1:5555";
const FRAME_SIZE : usize = 2048;

fn main() {
    // test_response_time(10).expect("test response time failed");
    #[cfg(feature = "output")]
    test_throughput(10, 2, 1).expect("test throughput failed");
    #[cfg(not(feature = "output"))]
    test_throughput(40, 20, 10).expect("test throughput failed");
}


// fn test_response_time(n: usize) -> io::Result<()> {
//     let mut results: Vec<u128> = Vec::new();
//     let mut buf = [0; FRAME_SIZE];
//     for i in 0 .. n {
//         let start_time = Instant::now();
//         let mut w_stream = TcpStream::connect(DEST)?;
//         w_stream.write_all(&DATA)?;
//         let mut total_size = w_stream.read(&mut buf)?;
//         while total_size < FRAME_SIZE {
//             let size = w_stream.read(&mut buf)?;
//             if size == 0 {
//                 break;
//             }
//             total_size += size;
//         }
//         let end_time = Instant::now();
//         let response_time = (end_time - start_time).as_micros();
//         results.push(response_time);

//         #[cfg(feature = "output")]
//         {
//             println!("connection {} response time: {} μs", i, response_time);
//         }
//     }

//     println!("resonse time test result stats:");
//     println!("mean: {} μs, variance: {} μs", mean(&results).unwrap(), variance(&results).unwrap());

//     Ok(())
// }

fn test_throughput(connection_num: usize, request_per_connection: usize, test_num: usize) -> io::Result<()> {
    use std::collections::VecDeque;

    let mut streams: VecDeque<TcpStream> = VecDeque::new();
    let mut results: Vec<f64> = Vec::new();

    assert!(FRAME_SIZE % 8 == 0);
    let mut rng = SmallRng::seed_from_u64(0xdead_beef);
    let vec: Vec<u64> = (0..(connection_num * FRAME_SIZE / 8))
        .map(|_| rng.next_u32() as u64)
        .collect::<Vec<_>>();
    let mut byte_vec: Vec<u8> = Vec::new();
    vec.iter().for_each(|value| {
        byte_vec.append(&mut Vec::from(value.to_le_bytes()));
    });

    println!("creating connections...");

    for i in 0 .. connection_num {
        let w_stream = TcpStream::connect(DEST)?;
        #[cfg(feature = "output")]
        {
            let this_time = Instant::now();
            println!("connection {} connected", i);
        }
        streams.push_back(w_stream);
    }

    println!("testing...");

    for _ in 0 .. test_num {
        let start_time = Instant::now();

        for _ in 0 .. request_per_connection {
            for i in 0 .. connection_num {
                let w_data = &byte_vec[(i * FRAME_SIZE) .. ((i + 1) * FRAME_SIZE)];
                // println!("{:?}", w_data);
                streams[i].write_all(w_data)?;
                #[cfg(feature = "output")]
                {
                    let this_time = Instant::now();
                    println!("connection {} request sended, total time: {} μs", i, (this_time - start_time).as_micros());
                }
            }
        }

    
        let mut buf: [u8; 8] = [0; 8];
        let mut total_time: u128 = 0;
        for i in 0 .. connection_num {
            let mut total_size = streams[i].read(&mut buf)?;
            #[cfg(feature = "output")]
            {
                println!("connection {} response received, size = {}", i, total_size);
            }
            while total_size < 8 * request_per_connection {
                let size = streams[i].read(&mut buf)?;
                #[cfg(feature = "output")]
                {
                    println!("connection {} response received, size = {}", i, size);
                }
                if size == 0 {
                    break;
                }
                total_size += size;
            }
            #[cfg(feature = "output")]
            {
                let this_time = Instant::now();
                println!("connection {} closed, total time: {} μs", i, (this_time - start_time).as_micros());
            }
            if i == connection_num - 1 {
                let finish_time = Instant::now();
                total_time = (finish_time - start_time).as_micros();
            }
        }
    
        let average_time: f64 = total_time as f64 / connection_num as f64;
        let average_througput: f64 = 1000000 as f64 * connection_num as f64 * request_per_connection as f64 / total_time as f64;
        println!("total time: {} μs, average time: {} μs, average thoughput: {} requests/s", total_time, average_time, average_througput);
        results.push(average_througput);    
    }

    println!("throutput test result stats:");
    println!("mean: {} requests/s, variance: {} requests/s", mean(&results).unwrap(), variance(&results).unwrap());
    Ok(())
}