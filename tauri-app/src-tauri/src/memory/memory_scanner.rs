use crate::memory::memory_access::ProcessMemory;
use crate::memory::process_reader::ModuleInfo;

/// Verified VisualBoyAdvance signature used to locate
/// the EWRAM/IWRAM pointer.
const VBA_MEMORY_SIGNATURE: [u8; 8] = [0x81, 0xE6, 0xFF, 0xFF, 0x03, 0x00, 0x8B, 0x15];

/// Offset from the signature to the pointer address.
const POINTER_OFFSET: usize = 8;

/// Resolved memory regions inside VisualBoyAdvance.
#[derive(Debug, Clone)]
pub struct MemoryRegions {
    pub pointer_address: usize,
    pub ewram_address: usize,
    pub iwram_address: usize,
}

/// Scan the executable module for the known VBA signature.
pub fn resolve_memory_regions(
    memory: &ProcessMemory,
    module: &ModuleInfo,
) -> Result<MemoryRegions, String> {
    // Read the entire executable module.
    let module_data = memory.read(module.base_address, module.size)?;

    // Locate the signature.
    let signature_offset = find_pattern(&module_data, &VBA_MEMORY_SIGNATURE).ok_or_else(|| {
        "VBA memory signature was not found in the executable module.".to_string()
    })?;

    // Calculate the address containing the pointer.
    let pointer_location = module
        .base_address
        .checked_add(signature_offset)
        .and_then(|address| address.checked_add(POINTER_OFFSET))
        .ok_or_else(|| "Pointer address calculation overflowed.".to_string())?;

    // Read the pointer address from the target process.
    let pointer_address = read_u32(memory, pointer_location)? as usize;

    if pointer_address == 0 {
        return Err("VBA memory pointer address is null.".to_string());
    }

    // Dereference the pointer to obtain EWRAM.
    let ewram_address = read_u32(memory, pointer_address)? as usize;

    // IWRAM is stored immediately after the EWRAM pointer.
    let iwram_pointer_location = pointer_address
        .checked_add(4)
        .ok_or_else(|| "IWRAM pointer address overflowed.".to_string())?;

    let iwram_address = read_u32(memory, iwram_pointer_location)? as usize;

    if ewram_address == 0 || iwram_address == 0 {
        return Err("VBA returned a null EWRAM or IWRAM address.".to_string());
    }

    Ok(MemoryRegions {
        pointer_address,
        ewram_address,
        iwram_address,
    })
}

/// Find the first occurrence of a byte pattern.
fn find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > data.len() {
        return None;
    }

    data.windows(pattern.len())
        .position(|window| window == pattern)
}

/// Read a little-endian 32-bit value from another process.
fn read_u32(memory: &ProcessMemory, address: usize) -> Result<u32, String> {
    let bytes = memory.read(address, 4)?;

    let array: [u8; 4] = bytes
        .try_into()
        .map_err(|_| "Expected exactly 4 bytes.".to_string())?;

    Ok(u32::from_le_bytes(array))
}
