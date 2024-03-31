#[doc = "Register `TCBCNT` reader"]
pub type R = crate::R<TcbcntSpec>;
#[doc = "Field `TRANS_CARD_BYTE_COUNT` reader - Number of bytes transferred by CIU unit to card.\n\nIn 32-bit or 64-bit AMBA data-bus-width modes, register should\n\nbe accessed in full to avoid read-coherency problems. In 16-bit\n\nAMBA data-bus-width mode, internal 16-bit coherency register is\n\nimplemented. User should first read lower 16 bits and then higher\n\n16 bits. When reading lower 16 bits, higher 16 bits of counter are\n\nstored in temporary register. When higher 16 bits are read, data\n\nfrom temporary register is supplied.\n\nBoth TCBCNT and TBBCNT share same coherency register.\n\nWhen AREA_OPTIMIZED parameter is 1, register should be read\n\nonly after data transfer completes; during data transfer, register\n\nreturns 0."]
pub type TransCardByteCountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card.\n\nIn 32-bit or 64-bit AMBA data-bus-width modes, register should\n\nbe accessed in full to avoid read-coherency problems. In 16-bit\n\nAMBA data-bus-width mode, internal 16-bit coherency register is\n\nimplemented. User should first read lower 16 bits and then higher\n\n16 bits. When reading lower 16 bits, higher 16 bits of counter are\n\nstored in temporary register. When higher 16 bits are read, data\n\nfrom temporary register is supplied.\n\nBoth TCBCNT and TBBCNT share same coherency register.\n\nWhen AREA_OPTIMIZED parameter is 1, register should be read\n\nonly after data transfer completes; during data transfer, register\n\nreturns 0."]
    #[inline(always)]
    pub fn trans_card_byte_count(&self) -> TransCardByteCountR {
        TransCardByteCountR::new(self.bits)
    }
}
#[doc = "Transferred CIU card byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcbcntSpec;
impl crate::RegisterSpec for TcbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcbcnt::R`](R) reader structure"]
impl crate::Readable for TcbcntSpec {}
#[doc = "`reset()` method sets TCBCNT to value 0"]
impl crate::Resettable for TcbcntSpec {
    const RESET_VALUE: u32 = 0;
}
