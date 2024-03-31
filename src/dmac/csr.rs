#[doc = "Register `CSR%s` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `CSR_BITS_7` reader - The channel status encoding is:\n\nb0000 = Stopped\n\nb0001 = Executing\n\nb0010 = Cache miss\n\nb0011 = Updating PC\n\nb0100 = Waiting for event\n\nb0101 = At barrier\n\nb0110 = reserved\n\nb0111 = Waiting for peripheral\n\nb1000 = Killing\n\nb1001 = Completing\n\nb1010-b1101 = reserved\n\nb1110 = Faulting completing\n\nb1111 = Faulting"]
pub type CsrBits7R = crate::FieldReader;
#[doc = "Field `CSR_BITS_6` reader - If the DMA channel is in the Waiting for event state or the Waiting\n\nfor peripheral state then these bits\n\nindicate the event or peripheral number that the channel is waiting\n\nfor:\n\nb00000 = DMA channel is waiting for event, or peripheral, 0\n\nb00001 = DMA channel is waiting for event, or peripheral, 1\n\nb00010 = DMA channel is waiting for event, or peripheral, 2\n\n...\n\nb11111 = DMA channel is waiting for event, or peripheral, 31"]
pub type CsrBits6R = crate::FieldReader;
#[doc = "Field `CSR_BITS_4` reader - When the DMA channel thread executes DMAWFP this bit indicates\n\nif the burst or single operand were set:\n\n0 = DMAWFP executed with the single operand set\n\n1 = DMAWFP executed with the burst operand set."]
pub type CsrBits4R = crate::BitReader;
#[doc = "Field `CSR_BITS_3` reader - When the DMA channel thread executes DMAWFP this bit indicates\n\nif the periph operand was set:\n\n0 = DMAWFP executed with the periph operand not set\n\n1 = DMAWFP executed with the periph operand set"]
pub type CsrBits3R = crate::BitReader;
#[doc = "Field `CSR_BITS_1` reader - The channel non-secure bit provides the security of the DMA\n\nchannel:\n\n0 = DMA channel operates in the Secure state\n\n1 = DMA channel operates in the Non-secure state"]
pub type CsrBits1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The channel status encoding is:\n\nb0000 = Stopped\n\nb0001 = Executing\n\nb0010 = Cache miss\n\nb0011 = Updating PC\n\nb0100 = Waiting for event\n\nb0101 = At barrier\n\nb0110 = reserved\n\nb0111 = Waiting for peripheral\n\nb1000 = Killing\n\nb1001 = Completing\n\nb1010-b1101 = reserved\n\nb1110 = Faulting completing\n\nb1111 = Faulting"]
    #[inline(always)]
    pub fn csr_bits_7(&self) -> CsrBits7R {
        CsrBits7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - If the DMA channel is in the Waiting for event state or the Waiting\n\nfor peripheral state then these bits\n\nindicate the event or peripheral number that the channel is waiting\n\nfor:\n\nb00000 = DMA channel is waiting for event, or peripheral, 0\n\nb00001 = DMA channel is waiting for event, or peripheral, 1\n\nb00010 = DMA channel is waiting for event, or peripheral, 2\n\n...\n\nb11111 = DMA channel is waiting for event, or peripheral, 31"]
    #[inline(always)]
    pub fn csr_bits_6(&self) -> CsrBits6R {
        CsrBits6R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - When the DMA channel thread executes DMAWFP this bit indicates\n\nif the burst or single operand were set:\n\n0 = DMAWFP executed with the single operand set\n\n1 = DMAWFP executed with the burst operand set."]
    #[inline(always)]
    pub fn csr_bits_4(&self) -> CsrBits4R {
        CsrBits4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When the DMA channel thread executes DMAWFP this bit indicates\n\nif the periph operand was set:\n\n0 = DMAWFP executed with the periph operand not set\n\n1 = DMAWFP executed with the periph operand set"]
    #[inline(always)]
    pub fn csr_bits_3(&self) -> CsrBits3R {
        CsrBits3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - The channel non-secure bit provides the security of the DMA\n\nchannel:\n\n0 = DMA channel operates in the Secure state\n\n1 = DMA channel operates in the Non-secure state"]
    #[inline(always)]
    pub fn csr_bits_1(&self) -> CsrBits1R {
        CsrBits1R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Channel Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`reset()` method sets CSR%s to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
