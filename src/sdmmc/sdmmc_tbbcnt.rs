#[doc = "Register `SDMMC_TBBCNT` reader"]
pub type R = crate::R<SdmmcTbbcntSpec>;
#[doc = "Field `TRANS_FIFO_BYTE_COUNT` reader - Number of bytes transferred between Host/DMA memory and BIU\n\nFIFO.\n\nIn 32-bit or 64-bit AMBA data-bus-width modes, register should\n\nbe accessed in full to avoid read-coherency problems. In 16-bit\n\nAMBA data-bus-width mode, internal 16-bit coherency register is\n\nimplemented. User should first read lower 16 bits and then higher\n\n16 bits. When reading lower 16 bits, higher 16 bits of counter are\n\nstored in temporary register. When higher 16 bits are read, data\n\nfrom temporary register is supplied.\n\nBoth TCBCNT and TBBCNT share same coherency register."]
pub type TransFifoByteCountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU\n\nFIFO.\n\nIn 32-bit or 64-bit AMBA data-bus-width modes, register should\n\nbe accessed in full to avoid read-coherency problems. In 16-bit\n\nAMBA data-bus-width mode, internal 16-bit coherency register is\n\nimplemented. User should first read lower 16 bits and then higher\n\n16 bits. When reading lower 16 bits, higher 16 bits of counter are\n\nstored in temporary register. When higher 16 bits are read, data\n\nfrom temporary register is supplied.\n\nBoth TCBCNT and TBBCNT share same coherency register."]
    #[inline(always)]
    pub fn trans_fifo_byte_count(&self) -> TransFifoByteCountR {
        TransFifoByteCountR::new(self.bits)
    }
}
#[doc = "Transferred host/DMA to/from BIU-FIFO byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_tbbcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcTbbcntSpec;
impl crate::RegisterSpec for SdmmcTbbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_tbbcnt::R`](R) reader structure"]
impl crate::Readable for SdmmcTbbcntSpec {}
#[doc = "`reset()` method sets SDMMC_TBBCNT to value 0"]
impl crate::Resettable for SdmmcTbbcntSpec {
    const RESET_VALUE: u32 = 0;
}
