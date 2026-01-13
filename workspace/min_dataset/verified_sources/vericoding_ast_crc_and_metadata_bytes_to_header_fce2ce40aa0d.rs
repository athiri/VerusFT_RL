use vstd::prelude::*;
use vstd :: set :: * ;
use vstd :: arithmetic :: div_mod :: * ;
use vstd :: bytes :: * ;
use vstd :: seq :: * ;
use vstd :: slice :: * ;
use vstd :: arithmetic :: mul :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub const header_log_size_offset : u64 = 24 ;
pub const header_tail_offset : u64 = 16 ;
pub const header_head_offset : u64 = 8 ;
pub const header_size : u64 = 32 ;
# [verifier :: ext_equal] pub struct PersistentHeader { pub crc : u64 , pub metadata : PersistentHeaderMetadata , }
# [verifier :: ext_equal] pub struct PersistentHeaderMetadata { pub head : u64 , pub tail : u64 , pub log_size : u64 , }
pub open spec (checked) fn spec_bytes_to_metadata (header_seq : Seq < u8 >) -> PersistentHeaderMetadata recommends header_seq . len () == 3 * 8 { let head = spec_u64_from_le_bytes (header_seq . subrange (header_head_offset - 8 , header_head_offset - 8 + 8)) ; let tail = spec_u64_from_le_bytes (header_seq . subrange (header_tail_offset - 8 , header_tail_offset - 8 + 8)) ; let log_size = spec_u64_from_le_bytes (header_seq . subrange (header_log_size_offset - 8 , header_log_size_offset - 8 + 8)) ; PersistentHeaderMetadata { head , tail , log_size } }
exec fn crc_and_metadata_bytes_to_header (crc_bytes : & [u8] , header_bytes : & [u8]) -> (out : PersistentHeader) requires crc_bytes @ . len () == 8 , header_bytes @ . len () == header_size - 8 ensures out . crc == spec_u64_from_le_bytes (crc_bytes @) , out . metadata == spec_bytes_to_metadata (header_bytes @) { let head_bytes = slice_subrange (header_bytes , (header_head_offset - 8) as usize , (header_head_offset + 8 - 8) as usize) ; let tail_bytes = slice_subrange (header_bytes , (header_tail_offset - 8) as usize , (header_tail_offset + 8 - 8) as usize) ; let log_size_bytes = slice_subrange (header_bytes , (header_log_size_offset - 8) as usize , (header_log_size_offset + 8 - 8) as usize) ; PersistentHeader { crc : u64_from_le_bytes (crc_bytes) , metadata : PersistentHeaderMetadata { head : u64_from_le_bytes (head_bytes) , tail : u64_from_le_bytes (tail_bytes) , log_size : u64_from_le_bytes (log_size_bytes) } } }

} // verus!