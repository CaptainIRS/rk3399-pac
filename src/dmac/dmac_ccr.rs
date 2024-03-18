#[doc = "Register `DMAC_CCR%s` reader"]
pub type R = crate::R<DmacCcrSpec>;
#[doc = "Field `DMAC_CCR_BITS_10` reader - Programs the burst type that the DMAC performs when it reads the source data: 0 = Fixed-address burst. The DMAC signals ARBURST\\[0\\]
LOW. 1 = Incrementing-address burst. The DMAC signals ARBURST\\[0\\]
HIGH"]
pub type DmacCcrBits10R = crate::BitReader;
#[doc = "Field `DMAC_CCR_BITS_9` reader - For each beat within a burst, it programs the number of bytes that the DMAC reads from the source: b000 = reads 1 byte per beat b001 = reads 2 bytes per beat b010 = reads 4 bytes per beat b011 = reads 8 bytes per beat b100 = reads 16 bytes per beat b101-b111 = reserved. The total number of bytes that the DMAC reads into the MFIFO when it executes a DMALD instruction is the product of src_burst_len and src_burst_size"]
pub type DmacCcrBits9R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_8` reader - For each burst, these bits program the number of data transfers that the DMAC performs when it reads the source data: b0000 = 1 data transfer b0001 = 2 data transfers b0010 = 3 data transfers ... b1111 = 16 data transfers. The total number of bytes that the DMAC reads into the MFIFO when it executes a DMALD instruction is the product of src_burst_len and src_burst_size"]
pub type DmacCcrBits8R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_7` reader - Programs the state of ARPROT\\[2:0\\]a when the DMAC reads the source data. Bit \\[10\\]
0 = ARPROT\\[2\\]
is LOW 1 = ARPROT\\[2\\]
is HIGH. Bit \\[9\\]
0 = ARPROT\\[1\\]
is LOW 1 = ARPROT\\[1\\]
is HIGH. Bit \\[8\\]
0 = ARPROT\\[0\\]
is LOW 1 = ARPROT\\[0\\]
is HIGH."]
pub type DmacCcrBits7R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_6` reader - Set the bits to control the state of ARCACHE\\[2:0\\]a when the DMAC reads the source data. Bit \\[13\\]
0 = ARCACHE\\[2\\]
is LOW 1 = ARCACHE\\[2\\]
is HIGH. Bit \\[12\\]
0 = ARCACHE\\[1\\]
is LOW 1 = ARCACHE\\[1\\]
is HIGH. Bit \\[11\\]
0 = ARCACHE\\[0\\]
is LOW 1 = ARCACHE\\[0\\]
is HIGH."]
pub type DmacCcrBits6R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_5` reader - Programs the burst type that the DMAC performs when it writes the destination data: 0 = Fixed-address burst. The DMAC signals AWBURST\\[0\\]
LOW. 1 = Incrementing-address burst. The DMAC signals AWBURST\\[0\\]
HIGH."]
pub type DmacCcrBits5R = crate::BitReader;
#[doc = "Field `DMAC_CCR_BITS_4` reader - For each beat within a burst, it programs the number of bytes that the DMAC writes to the destination: b000 = writes 1 byte per beat b001 = writes 2 bytes per beat b010 = writes 4 bytes per beat b011 = writes 8 bytes per beat b100 = writes 16 bytes per beat b101-b111 = reserved. The total number of bytes that the DMAC writes out of the MFIFO when it executes a DMAST instruction is the product of dst_burst_len and dst_burst_size."]
pub type DmacCcrBits4R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_3` reader - For each burst, these bits program the number of data transfers that the DMAC performs when it writes the destination data: b0000 = 1 data transfer b0001 = 2 data transfers b0010 = 3 data transfers ... b1111 = 16 data transfers. The total number of bytes that the DMAC writes out of the MFIFO when it executes a DMAST instruction is the product of dst_burst_len and dst_burst_size"]
pub type DmacCcrBits3R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_2` reader - Programs the state of AWPROT\\[2:0\\]a when the DMAC writes the destination data. Bit \\[24\\]
0 = AWPROT\\[2\\]
is LOW 1 = AWPROT\\[2\\]
is HIGH. Bit \\[23\\]
0 = AWPROT\\[1\\]
is LOW 1 = AWPROT\\[1\\]
is HIGH. Bit \\[22\\]
0 = AWPROT\\[0\\]
is LOW 1 = AWPROT\\[0\\]
is HIGH"]
pub type DmacCcrBits2R = crate::FieldReader;
#[doc = "Field `DMAC_CCR_BITS_1` reader - Programs the state of AWCACHE\\[3,1:0\\]a when the DMAC writes the destination data. Bit \\[27\\]
0 = AWCACHE\\[3\\]
is LOW 1 = AWCACHE\\[3\\]
is HIGH. Bit \\[26\\]
0 = AWCACHE\\[1\\]
is LOW 1 = AWCACHE\\[1\\]
is HIGH. Bit \\[25\\]
0 = AWCACHE\\[0\\]
is LOW 1 = AWCACHE\\[0\\]
is HIGH"]
pub type DmacCcrBits1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Programs the burst type that the DMAC performs when it reads the source data: 0 = Fixed-address burst. The DMAC signals ARBURST\\[0\\]
LOW. 1 = Incrementing-address burst. The DMAC signals ARBURST\\[0\\]
HIGH"]
    #[inline(always)]
    pub fn dmac_ccr_bits_10(&self) -> DmacCcrBits10R {
        DmacCcrBits10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - For each beat within a burst, it programs the number of bytes that the DMAC reads from the source: b000 = reads 1 byte per beat b001 = reads 2 bytes per beat b010 = reads 4 bytes per beat b011 = reads 8 bytes per beat b100 = reads 16 bytes per beat b101-b111 = reserved. The total number of bytes that the DMAC reads into the MFIFO when it executes a DMALD instruction is the product of src_burst_len and src_burst_size"]
    #[inline(always)]
    pub fn dmac_ccr_bits_9(&self) -> DmacCcrBits9R {
        DmacCcrBits9R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - For each burst, these bits program the number of data transfers that the DMAC performs when it reads the source data: b0000 = 1 data transfer b0001 = 2 data transfers b0010 = 3 data transfers ... b1111 = 16 data transfers. The total number of bytes that the DMAC reads into the MFIFO when it executes a DMALD instruction is the product of src_burst_len and src_burst_size"]
    #[inline(always)]
    pub fn dmac_ccr_bits_8(&self) -> DmacCcrBits8R {
        DmacCcrBits8R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Programs the state of ARPROT\\[2:0\\]a when the DMAC reads the source data. Bit \\[10\\]
0 = ARPROT\\[2\\]
is LOW 1 = ARPROT\\[2\\]
is HIGH. Bit \\[9\\]
0 = ARPROT\\[1\\]
is LOW 1 = ARPROT\\[1\\]
is HIGH. Bit \\[8\\]
0 = ARPROT\\[0\\]
is LOW 1 = ARPROT\\[0\\]
is HIGH."]
    #[inline(always)]
    pub fn dmac_ccr_bits_7(&self) -> DmacCcrBits7R {
        DmacCcrBits7R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Set the bits to control the state of ARCACHE\\[2:0\\]a when the DMAC reads the source data. Bit \\[13\\]
0 = ARCACHE\\[2\\]
is LOW 1 = ARCACHE\\[2\\]
is HIGH. Bit \\[12\\]
0 = ARCACHE\\[1\\]
is LOW 1 = ARCACHE\\[1\\]
is HIGH. Bit \\[11\\]
0 = ARCACHE\\[0\\]
is LOW 1 = ARCACHE\\[0\\]
is HIGH."]
    #[inline(always)]
    pub fn dmac_ccr_bits_6(&self) -> DmacCcrBits6R {
        DmacCcrBits6R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Programs the burst type that the DMAC performs when it writes the destination data: 0 = Fixed-address burst. The DMAC signals AWBURST\\[0\\]
LOW. 1 = Incrementing-address burst. The DMAC signals AWBURST\\[0\\]
HIGH."]
    #[inline(always)]
    pub fn dmac_ccr_bits_5(&self) -> DmacCcrBits5R {
        DmacCcrBits5R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - For each beat within a burst, it programs the number of bytes that the DMAC writes to the destination: b000 = writes 1 byte per beat b001 = writes 2 bytes per beat b010 = writes 4 bytes per beat b011 = writes 8 bytes per beat b100 = writes 16 bytes per beat b101-b111 = reserved. The total number of bytes that the DMAC writes out of the MFIFO when it executes a DMAST instruction is the product of dst_burst_len and dst_burst_size."]
    #[inline(always)]
    pub fn dmac_ccr_bits_4(&self) -> DmacCcrBits4R {
        DmacCcrBits4R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:21 - For each burst, these bits program the number of data transfers that the DMAC performs when it writes the destination data: b0000 = 1 data transfer b0001 = 2 data transfers b0010 = 3 data transfers ... b1111 = 16 data transfers. The total number of bytes that the DMAC writes out of the MFIFO when it executes a DMAST instruction is the product of dst_burst_len and dst_burst_size"]
    #[inline(always)]
    pub fn dmac_ccr_bits_3(&self) -> DmacCcrBits3R {
        DmacCcrBits3R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Programs the state of AWPROT\\[2:0\\]a when the DMAC writes the destination data. Bit \\[24\\]
0 = AWPROT\\[2\\]
is LOW 1 = AWPROT\\[2\\]
is HIGH. Bit \\[23\\]
0 = AWPROT\\[1\\]
is LOW 1 = AWPROT\\[1\\]
is HIGH. Bit \\[22\\]
0 = AWPROT\\[0\\]
is LOW 1 = AWPROT\\[0\\]
is HIGH"]
    #[inline(always)]
    pub fn dmac_ccr_bits_2(&self) -> DmacCcrBits2R {
        DmacCcrBits2R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Programs the state of AWCACHE\\[3,1:0\\]a when the DMAC writes the destination data. Bit \\[27\\]
0 = AWCACHE\\[3\\]
is LOW 1 = AWCACHE\\[3\\]
is HIGH. Bit \\[26\\]
0 = AWCACHE\\[1\\]
is LOW 1 = AWCACHE\\[1\\]
is HIGH. Bit \\[25\\]
0 = AWCACHE\\[0\\]
is LOW 1 = AWCACHE\\[0\\]
is HIGH"]
    #[inline(always)]
    pub fn dmac_ccr_bits_1(&self) -> DmacCcrBits1R {
        DmacCcrBits1R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[doc = "Channel Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCcrSpec;
impl crate::RegisterSpec for DmacCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ccr::R`](R) reader structure"]
impl crate::Readable for DmacCcrSpec {}
#[doc = "`reset()` method sets DMAC_CCR%s to value 0"]
impl crate::Resettable for DmacCcrSpec {
    const RESET_VALUE: u32 = 0;
}
