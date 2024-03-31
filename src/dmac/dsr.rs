#[doc = "Register `DSR` reader"]
pub type R = crate::R<DsrSpec>;
#[doc = "Field `DSR_BITS_3` reader - The operating state of the DMA manager:\n\nb0000 = Stopped\n\nb0001 = Executing\n\nb0010 = Cache miss\n\nb0011 = Updating PC\n\nb0100 = Waiting for event\n\nb0101-b1110 = reserved\n\nb1111 = Faulting."]
pub type DsrBits3R = crate::FieldReader;
#[doc = "Field `DSR_BITS_2` reader - When the DMA manager thread executes a DMAWFE instruction, it\n\nwaits for the following event to occur:\n\nb00000 = event\\[0\\]\n\nb00001 = event\\[1\\]\n\nb00010 = event\\[2\\]\n\n...\n\nb11111 = event\\[31\\]."]
pub type DsrBits2R = crate::FieldReader;
#[doc = "Field `DSR_BITS_1` reader - Provides the security status of the DMA manager thread:\n\n0 = DMA manager operates in the Secure state\n\n1 = DMA manager operates in the Non-secure state."]
pub type DsrBits1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The operating state of the DMA manager:\n\nb0000 = Stopped\n\nb0001 = Executing\n\nb0010 = Cache miss\n\nb0011 = Updating PC\n\nb0100 = Waiting for event\n\nb0101-b1110 = reserved\n\nb1111 = Faulting."]
    #[inline(always)]
    pub fn dsr_bits_3(&self) -> DsrBits3R {
        DsrBits3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - When the DMA manager thread executes a DMAWFE instruction, it\n\nwaits for the following event to occur:\n\nb00000 = event\\[0\\]\n\nb00001 = event\\[1\\]\n\nb00010 = event\\[2\\]\n\n...\n\nb11111 = event\\[31\\]."]
    #[inline(always)]
    pub fn dsr_bits_2(&self) -> DsrBits2R {
        DsrBits2R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Provides the security status of the DMA manager thread:\n\n0 = DMA manager operates in the Secure state\n\n1 = DMA manager operates in the Non-secure state."]
    #[inline(always)]
    pub fn dsr_bits_1(&self) -> DsrBits1R {
        DsrBits1R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DMA Manager Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrSpec;
impl crate::RegisterSpec for DsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr::R`](R) reader structure"]
impl crate::Readable for DsrSpec {}
#[doc = "`reset()` method sets DSR to value 0"]
impl crate::Resettable for DsrSpec {
    const RESET_VALUE: u32 = 0;
}
