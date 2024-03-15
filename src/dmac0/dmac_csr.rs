#[doc = "Register `DMAC_CSR%s` reader"]
pub type R = crate::R<DmacCsrSpec>;
#[doc = "Field `DMAC_CSR_BITS_7` reader - The channel status encoding is: b0000 = Stopped b0001 = Executing b0010 = Cache miss b0011 = Updating PC b0100 = Waiting for event b0101 = At barrier b0110 = reserved b0111 = Waiting for peripheral b1000 = Killing b1001 = Completing b1010-b1101 = reserved b1110 = Faulting completing b1111 = Faulting"]
pub type DmacCsrBits7R = crate::FieldReader;
#[doc = "Field `DMAC_CSR_BITS_6` reader - If the DMA channel is in the Waiting for event state or the Waiting for peripheral state then these bits indicate the event or peripheral number that the channel is waiting for: b00000 = DMA channel is waiting for event, or peripheral, 0 b00001 = DMA channel is waiting for event, or peripheral, 1 b00010 = DMA channel is waiting for event, or peripheral, 2 ... b11111 = DMA channel is waiting for event, or peripheral, 31"]
pub type DmacCsrBits6R = crate::FieldReader;
#[doc = "Field `DMAC_CSR_BITS_4` reader - When the DMA channel thread executes DMAWFP this bit indicates if the burst or single operand were set: 0 = DMAWFP executed with the single operand set 1 = DMAWFP executed with the burst operand set."]
pub type DmacCsrBits4R = crate::BitReader;
#[doc = "Field `DMAC_CSR_BITS_3` reader - When the DMA channel thread executes DMAWFP this bit indicates if the periph operand was set: 0 = DMAWFP executed with the periph operand not set 1 = DMAWFP executed with the periph operand set"]
pub type DmacCsrBits3R = crate::BitReader;
#[doc = "Field `DMAC_CSR_BITS_1` reader - The channel non-secure bit provides the security of the DMA channel: 0 = DMA channel operates in the Secure state 1 = DMA channel operates in the Non-secure state"]
pub type DmacCsrBits1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The channel status encoding is: b0000 = Stopped b0001 = Executing b0010 = Cache miss b0011 = Updating PC b0100 = Waiting for event b0101 = At barrier b0110 = reserved b0111 = Waiting for peripheral b1000 = Killing b1001 = Completing b1010-b1101 = reserved b1110 = Faulting completing b1111 = Faulting"]
    #[inline(always)]
    pub fn dmac_csr_bits_7(&self) -> DmacCsrBits7R {
        DmacCsrBits7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - If the DMA channel is in the Waiting for event state or the Waiting for peripheral state then these bits indicate the event or peripheral number that the channel is waiting for: b00000 = DMA channel is waiting for event, or peripheral, 0 b00001 = DMA channel is waiting for event, or peripheral, 1 b00010 = DMA channel is waiting for event, or peripheral, 2 ... b11111 = DMA channel is waiting for event, or peripheral, 31"]
    #[inline(always)]
    pub fn dmac_csr_bits_6(&self) -> DmacCsrBits6R {
        DmacCsrBits6R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - When the DMA channel thread executes DMAWFP this bit indicates if the burst or single operand were set: 0 = DMAWFP executed with the single operand set 1 = DMAWFP executed with the burst operand set."]
    #[inline(always)]
    pub fn dmac_csr_bits_4(&self) -> DmacCsrBits4R {
        DmacCsrBits4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When the DMA channel thread executes DMAWFP this bit indicates if the periph operand was set: 0 = DMAWFP executed with the periph operand not set 1 = DMAWFP executed with the periph operand set"]
    #[inline(always)]
    pub fn dmac_csr_bits_3(&self) -> DmacCsrBits3R {
        DmacCsrBits3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - The channel non-secure bit provides the security of the DMA channel: 0 = DMA channel operates in the Secure state 1 = DMA channel operates in the Non-secure state"]
    #[inline(always)]
    pub fn dmac_csr_bits_1(&self) -> DmacCsrBits1R {
        DmacCsrBits1R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Channel Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCsrSpec;
impl crate::RegisterSpec for DmacCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_csr::R`](R) reader structure"]
impl crate::Readable for DmacCsrSpec {}
#[doc = "`reset()` method sets DMAC_CSR%s to value 0"]
impl crate::Resettable for DmacCsrSpec {
    const RESET_VALUE: u32 = 0;
}
