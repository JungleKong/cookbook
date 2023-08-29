// use inline_python::python;
// // 需要切换到nightly版本

// #[cfg(test)]
// mod test_ffi {
//     use super::*;

//     #[test]
//     fn test1() {
//         python! {
//             import sys
//             print("Hello from {}!".format(sys.version))
//         }
//     }

//     #[test]
//     fn test2() {
//         let x = 1;
//         let y = 2;
//         let z: i32 = python! {
//             'x + 'y
//         };
//         assert_eq!(z, 3);
//     }
// }