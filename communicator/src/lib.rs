// 프로젝트 폴더 구존
// 파일, 폴더별로 네임스페이싱 됨
// 해당 폴더 네임스페이스 루트격: mod.rs


// 현재 구조
// client
//      ::connect
// network
//      ::client
//          ::connect
//      ::server
//          ::connect
pub mod client;

pub mod network;
