#[doc = "Register `DMAC_CRDn` reader"]
pub type R = crate::R<DmacCrdnSpec>;
#[doc = "Field `DMAC_CRDn_BITS_9` reader - The data bus width of the AXI interface: b000 = reserved b001 = reserved b010 = 32-bit b011 = 64-bit b100 = 128-bit b101-b111 = reserved."]
pub type DmacCrdnBits9R = crate::FieldReader;
#[doc = "Field `DMAC_CRDn_BITS_7` reader - Write issuing capability that programs the number of outstanding write transactions: b000 = 1 b001 = 2 ... b111 = 8"]
pub type DmacCrdnBits7R = crate::FieldReader;
#[doc = "Field `DMAC_CRDn_BITS_5` reader - The depth of the write queue: b0000 = 1 line b0001 = 2 lines ... b1111 = 16 lines."]
pub type DmacCrdnBits5R = crate::FieldReader;
#[doc = "Field `DMAC_CRDn_BITS_4` reader - Read issuing capability that programs the number of outstanding read transactions: b000 = 1 b001 = 2 ... b111 = 8"]
pub type DmacCrdnBits4R = crate::FieldReader;
#[doc = "Field `DMAC_CRDn_BITS_2` reader - The depth of the read queue: b0000 = 1 line b0001 = 2 lines ... b1111 = 16 lines."]
pub type DmacCrdnBits2R = crate::FieldReader;
#[doc = "Field `DMAC_CRDn_BITS_1` reader - The number of lines that the data buffer contains: b000000000 = 1 line b000000001 = 2 lines ... b111111111 = 1024 lines"]
pub type DmacCrdnBits1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - The data bus width of the AXI interface: b000 = reserved b001 = reserved b010 = 32-bit b011 = 64-bit b100 = 128-bit b101-b111 = reserved."]
    #[inline(always)]
    pub fn dmac_crdn_bits_9(&self) -> DmacCrdnBits9R {
        DmacCrdnBits9R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write issuing capability that programs the number of outstanding write transactions: b000 = 1 b001 = 2 ... b111 = 8"]
    #[inline(always)]
    pub fn dmac_crdn_bits_7(&self) -> DmacCrdnBits7R {
        DmacCrdnBits7R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - The depth of the write queue: b0000 = 1 line b0001 = 2 lines ... b1111 = 16 lines."]
    #[inline(always)]
    pub fn dmac_crdn_bits_5(&self) -> DmacCrdnBits5R {
        DmacCrdnBits5R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Read issuing capability that programs the number of outstanding read transactions: b000 = 1 b001 = 2 ... b111 = 8"]
    #[inline(always)]
    pub fn dmac_crdn_bits_4(&self) -> DmacCrdnBits4R {
        DmacCrdnBits4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - The depth of the read queue: b0000 = 1 line b0001 = 2 lines ... b1111 = 16 lines."]
    #[inline(always)]
    pub fn dmac_crdn_bits_2(&self) -> DmacCrdnBits2R {
        DmacCrdnBits2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:29 - The number of lines that the data buffer contains: b000000000 = 1 line b000000001 = 2 lines ... b111111111 = 1024 lines"]
    #[inline(always)]
    pub fn dmac_crdn_bits_1(&self) -> DmacCrdnBits1R {
        DmacCrdnBits1R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_crdn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCrdnSpec;
impl crate::RegisterSpec for DmacCrdnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_crdn::R`](R) reader structure"]
impl crate::Readable for DmacCrdnSpec {}
#[doc = "`reset()` method sets DMAC_CRDn to value 0"]
impl crate::Resettable for DmacCrdnSpec {
    const RESET_VALUE: u32 = 0;
}
