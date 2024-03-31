#[doc = "Register `CRDn` reader"]
pub type R = crate::R<CrdnSpec>;
#[doc = "Field `CRDn_BITS_9` reader - The data bus width of the AXI interface:\n\nb000 = reserved\n\nb001 = reserved\n\nb010 = 32-bit\n\nb011 = 64-bit\n\nb100 = 128-bit\n\nb101-b111 = reserved."]
pub type CrdnBits9R = crate::FieldReader;
#[doc = "Field `CRDn_BITS_7` reader - Write issuing capability that programs the number of outstanding\n\nwrite transactions:\n\nb000 = 1\n\nb001 = 2\n\n...\n\nb111 = 8"]
pub type CrdnBits7R = crate::FieldReader;
#[doc = "Field `CRDn_BITS_5` reader - The depth of the write queue:\n\nb0000 = 1 line\n\nb0001 = 2 lines\n\n...\n\nb1111 = 16 lines."]
pub type CrdnBits5R = crate::FieldReader;
#[doc = "Field `CRDn_BITS_4` reader - Read issuing capability that programs the number of outstanding\n\nread transactions:\n\nb000 = 1\n\nb001 = 2\n\n...\n\nb111 = 8"]
pub type CrdnBits4R = crate::FieldReader;
#[doc = "Field `CRDn_BITS_2` reader - The depth of the read queue:\n\nb0000 = 1 line\n\nb0001 = 2 lines\n\n...\n\nb1111 = 16 lines."]
pub type CrdnBits2R = crate::FieldReader;
#[doc = "Field `CRDn_BITS_1` reader - The number of lines that the data buffer contains:\n\nb000000000 = 1 line\n\nb000000001 = 2 lines\n\n...\n\nb111111111 = 1024 lines"]
pub type CrdnBits1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - The data bus width of the AXI interface:\n\nb000 = reserved\n\nb001 = reserved\n\nb010 = 32-bit\n\nb011 = 64-bit\n\nb100 = 128-bit\n\nb101-b111 = reserved."]
    #[inline(always)]
    pub fn crdn_bits_9(&self) -> CrdnBits9R {
        CrdnBits9R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write issuing capability that programs the number of outstanding\n\nwrite transactions:\n\nb000 = 1\n\nb001 = 2\n\n...\n\nb111 = 8"]
    #[inline(always)]
    pub fn crdn_bits_7(&self) -> CrdnBits7R {
        CrdnBits7R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - The depth of the write queue:\n\nb0000 = 1 line\n\nb0001 = 2 lines\n\n...\n\nb1111 = 16 lines."]
    #[inline(always)]
    pub fn crdn_bits_5(&self) -> CrdnBits5R {
        CrdnBits5R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Read issuing capability that programs the number of outstanding\n\nread transactions:\n\nb000 = 1\n\nb001 = 2\n\n...\n\nb111 = 8"]
    #[inline(always)]
    pub fn crdn_bits_4(&self) -> CrdnBits4R {
        CrdnBits4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - The depth of the read queue:\n\nb0000 = 1 line\n\nb0001 = 2 lines\n\n...\n\nb1111 = 16 lines."]
    #[inline(always)]
    pub fn crdn_bits_2(&self) -> CrdnBits2R {
        CrdnBits2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:29 - The number of lines that the data buffer contains:\n\nb000000000 = 1 line\n\nb000000001 = 2 lines\n\n...\n\nb111111111 = 1024 lines"]
    #[inline(always)]
    pub fn crdn_bits_1(&self) -> CrdnBits1R {
        CrdnBits1R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crdn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrdnSpec;
impl crate::RegisterSpec for CrdnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crdn::R`](R) reader structure"]
impl crate::Readable for CrdnSpec {}
#[doc = "`reset()` method sets CRDn to value 0"]
impl crate::Resettable for CrdnSpec {
    const RESET_VALUE: u32 = 0;
}
