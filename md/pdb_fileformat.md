# Microsoft PDB (Program Database)

https://llvm.org/docs/PDB/index.html
https://github.com/microsoft/microsoft-pdb
https://en.wikipedia.org/wiki/Program_database

## Format

The PDB is organized in fixed-size pages, typically 1K, 2K, or 4K, numbered consecutively starting at 0.

The PDB is a single file which is logically composed of several sub-files, called streams.

Streams can be removed, added, or replaced without rewriting any other streams, and the changes to the metadata which describes the streams is minimized as well.

Streams are numbered consecutively starting with 0.

There is also a root stream, unnumbered, which contains some of the metadata. The root stream describes all of the PDB streams starting with stream 0. Its contents vary with the PDB format version.

Each stream in the PDB occupies several pages, which aren't necessarily consecutively numbered. The stream is identified by a number and has a length. The stream content is the concatenation of its pages, truncated to the stream's length.

# Investigating Source Code

https://github.com/microsoft/microsoft-pdb

PDB\msf\msf.cpp - line 933 ff

```
union MSF_HDR { // page 0
    struct {
        char szMagic[0x2C];
        CB  cbPg;       // page size
        PN  pnFpm;      // page no. of valid FPM
        PN  pnMac;      // current no. of pages
        SI_PERSIST
            siSt;       // stream table stream info
        PN  mpspnpn[cpnMaxForCb(StrmTbl::cbMaxSerialization)];
    };
    PG pg;
};

union BIGMSF_HDR {  // page 0 (and more if necessary)
    struct {
        char    szMagic[0x1e];  // version string
        CB  cbPg;               // page size
        UPN pnFpm;              // page no. of valid FPM
        UPN pnMac;              // current no. of pages
        SI_PERSIST
            siSt;               // stream table stream info
        PN32 mpspnpnSt[cpnMaxForCb(cpnMaxForCb(StrmTbl::cbBigMSFMaxSer) * sizeof(PN32))];
    };
    PG  pg;
};
```

The LLVM documentation (https://llvm.org/docs/PDB/MsfFile.html) defines these structures as the SuperBlock.

```
The Superblock

At file offset 0 in an MSF file is the MSF SuperBlock, which is laid out as follows:

struct SuperBlock {
  char FileMagic[sizeof(Magic)];
  ulittle32_t BlockSize;
  ulittle32_t FreeBlockMapBlock;
  ulittle32_t NumBlocks;
  ulittle32_t NumDirectoryBytes;
  ulittle32_t Unknown;
  ulittle32_t BlockMapAddr;
};

FileMagic - Must be equal to "Microsoft C/C++ MSF 7.00\\r\\n" followed by the bytes 1A 44 53 00 00 00.

BlockSize - The block size of the internal file system. Valid values are 512, 1024, 2048, and 4096 bytes. Certain aspects of the MSF file layout vary depending on the block sizes. For the purposes of LLVM, we handle only block sizes of 4KiB, and all further discussion assumes a block size of 4KiB.

FreeBlockMapBlock - The index of a block within the file, at which begins a bitfield representing the set of all blocks within the file which are “free” (i.e. the data within that block is not used). See The Free Block Map for more information. Important: FreeBlockMapBlock can only be 1 or 2!

NumBlocks - The total number of blocks in the file. NumBlocks * BlockSize should equal the size of the file on disk.

NumDirectoryBytes - The size of the stream directory, in bytes. The stream directory contains information about each stream’s size and the set of blocks that it occupies. It will be described in more detail later.

BlockMapAddr - The index of a block within the MSF file. At this block is an array of ulittle32_t’s listing the blocks that the stream directory resides on. For large MSF files, the stream directory (which describes the block layout of each stream) may not fit entirely on a single block. As a result, this extra layer of indirection is introduced, whereby this block contains the list of blocks that the stream directory occupies, and the stream directory itself can be stitched together accordingly. The number of ulittle32_t’s in this array is given by ceil(NumDirectoryBytes / BlockSize).
```

PDB\msf\msf.cpp - line 961 ff

```
static const char szHdrMagic[0x2c] =    "Microsoft C/C++ program database 2.00\r\n\x1a\x4a\x47";
static const char szBigHdrMagic[0x1e] = "Microsoft C/C++ MSF 7.00\r\n\x1a\x44\x53";
```

There is a MSF class for Multi Stream File.
Then there is a MSF_HB class for Multi Stream File Home Brewed.