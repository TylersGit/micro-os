const MULTIBOOT_MAGIC: u32 = 0xE85250D6;
const MULTIBOOT_ARCH: u32 = 0;
const MULTIBOOT_HEADER_LEN: u32 = 24; // hardcoded for now, needs a more elegant way to set it. 
const MULTIBOOT_CHECKSUM: u32 = !(MULTIBOOT_MAGIC + MULTIBOOT_ARCH + MULTIBOOT_HEADER_LEN) + 1; // 0 = magic + arch + header_len + checksum

struct MultibootHeader // According to the multiboot2 specs.
{
    magic: u32,
    arch: u32,
    header_len: u32,
    checksum: u32
}

struct MultibootTag
{
    tag_type: u16, // type is a used keyword
    flags: u16,
    size: u32
}

// The end tag is defined according to the spec.
const END_TAG: MultibootTag = MultibootTag {
    tag_type: 0,
    flags: 0,
    size: 8
};

const MULTIBOOT_TAGS: [MultibootTag; 1] = [END_TAG];


#[unsafe(no_mangle)]
#[unsafe(link_section = ".multiboot")] 
#[used] // Without this flag, the header is optimized out since it is not used anywhere. 
// static MULTIBOOT_HEADER: [u32; 3] = [MULTIBOOT_MAGIC, MULTIBOOT_FLAGS, MULTIBOOT_CHECKSUM];
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MULTIBOOT_MAGIC,
    arch: MULTIBOOT_ARCH,
    header_len: MULTIBOOT_HEADER_LEN,
    checksum: MULTIBOOT_CHECKSUM
};