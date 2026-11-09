use super::*;
use std::vec;

fn fixture() -> std::vec::Vec<u8> {
    let mut image = vec![0; 0xc0000];
    image[..2].copy_from_slice(b"MZ");
    image[60..64].copy_from_slice(&0x148u32.to_le_bytes());
    let header = &mut image[0x148..0x148 + 84];
    header[..4].copy_from_slice(b"PE\0\0");
    header[4..6].copy_from_slice(&0x14cu16.to_le_bytes());
    header[6..8].copy_from_slice(&4u16.to_le_bytes());
    header[8..12].copy_from_slice(&0x429c_4f34u32.to_le_bytes());
    header[24..26].copy_from_slice(&0x10bu16.to_le_bytes());
    header[80..84].copy_from_slice(&0x02fc_c000u32.to_le_bytes());
    for &(rva, bytes) in SIGNATURES {
        image[rva as usize..rva as usize + bytes.len()].copy_from_slice(bytes);
    }
    image
}

fn identifies(image: &[u8]) -> bool {
    is_jp(|rva, bytes| {
        bytes.copy_from_slice(image.get(rva as usize..rva as usize + bytes.len())?);
        Some(())
    })
}

#[test]
fn requires_jp_metadata_and_both_gameplay_signatures() {
    let image = fixture();
    assert!(identifies(&image));
    // Each independent discriminator must actually reject a mismatch.
    for offset in [
        0, 0x148, 0x14c, 0x14e, 0x150, 0x160, 0x198, 0x53cf0, 0xbd800,
    ] {
        let mut other = image.clone();
        other[offset] ^= 1;
        assert!(!identifies(&other), "accepted mismatch at {offset:x}");
    }
    let mut patch2 = image;
    patch2[0x150..0x154].copy_from_slice(&0x43c5_009au32.to_le_bytes());
    assert!(!identifies(&patch2));
}

fn launcher_patches(image: &mut std::vec::Vec<u8>) {
    image.resize(0x34e700, 0);
    for (rva, bytes) in [
        (0x1cd00, &[0xb8, 1, 0, 0, 0, 0xc3][..]),
        (0x5ecc4, &[0x90, 0x90]),
        (0x5eda9, &[0x90, 0x90]),
        (0x5edbe, &[0x70, 0xe6, 0x74, 0]),
        (0x34e670, b"replacement debug label"),
        (0x270, &0x108690u32.to_le_bytes()), // .rdata VirtualSize
        (0x11eb0b, &2560u32.to_le_bytes()),
        (0x11eb16, &1440u32.to_le_bytes()),
    ] {
        image[rva..rva + bytes.len()].copy_from_slice(bytes);
    }
}

#[test]
fn accepts_launcher_debug_and_resolution_patches() {
    let mut image = fixture();
    launcher_patches(&mut image);
    assert!(identifies(&image));
}

#[test]
fn unreadable_truncated_or_unbounded_header_is_rejected() {
    assert!(!is_jp(|_, _| None));
    assert!(!identifies(&fixture()[..100]));
    let mut image = fixture();
    image[60..64].copy_from_slice(&u32::MAX.to_le_bytes());
    let mut reads = 0;
    assert!(!is_jp(|rva, bytes| {
        reads += 1;
        bytes.copy_from_slice(&image[rva as usize..rva as usize + bytes.len()]);
        Some(())
    }));
    assert_eq!(reads, 1);
}

// .text's RVA equals its raw file offset in both supplied exes. No game assets
// are checked in; run this locally to verify the guard against the real inputs.
#[test]
#[ignore = "requires LSW1_JP_EXE and LSW1_PATCH2_EXE local file paths"]
fn real_build_identity() {
    let mut jp = std::fs::read(std::env::var("LSW1_JP_EXE").unwrap()).unwrap();
    let patch2 = std::fs::read(std::env::var("LSW1_PATCH2_EXE").unwrap()).unwrap();
    assert!(identifies(&jp));
    assert!(!identifies(&patch2));
    launcher_patches(&mut jp);
    assert!(identifies(&jp));
}
