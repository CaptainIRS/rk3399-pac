#[doc = "Register `GMAC_OVERFLOW_CNT` reader"]
pub type R = crate::R<GmacOverflowCntSpec>;
#[doc = "Field `FRAME_MISS_NUMBER_2` reader - Indicates the number of frames missed by the controller due to\n\nthe Host Receive Buffer being unavailable. This counter is\n\nincremented each time the DMA discards an incoming frame. The\n\ncounter is cleared when this register is read with mci_be_i\\[0\\]
at\n\n1'b1.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type FrameMissNumber2R = crate::FieldReader<u16>;
#[doc = "Field `MISS_FRAME_OVERFLOW_BIT` reader - Overflow bit for Missed Frame Counter\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MissFrameOverflowBitR = crate::BitReader;
#[doc = "Field `FRAME_MISS_NUMBER` reader - Indicates the number of frames missed by the application\n\nThis counter is incremented each time the MTL asserts the\n\nsideband signal mtl_rxoverflow_o. The counter is cleared when\n\nthis register is read with mci_be_i\\[2\\]
at 1'b1.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type FrameMissNumberR = crate::FieldReader<u16>;
#[doc = "Field `FIFO_OVERFLOW_BIT` reader - Overflow bit for FIFO Overflow Counter\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type FifoOverflowBitR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Indicates the number of frames missed by the controller due to\n\nthe Host Receive Buffer being unavailable. This counter is\n\nincremented each time the DMA discards an incoming frame. The\n\ncounter is cleared when this register is read with mci_be_i\\[0\\]
at\n\n1'b1."]
    #[inline(always)]
    pub fn frame_miss_number_2(&self) -> FrameMissNumber2R {
        FrameMissNumber2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn miss_frame_overflow_bit(&self) -> MissFrameOverflowBitR {
        MissFrameOverflowBitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Indicates the number of frames missed by the application\n\nThis counter is incremented each time the MTL asserts the\n\nsideband signal mtl_rxoverflow_o. The counter is cleared when\n\nthis register is read with mci_be_i\\[2\\]
at 1'b1."]
    #[inline(always)]
    pub fn frame_miss_number(&self) -> FrameMissNumberR {
        FrameMissNumberR::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn fifo_overflow_bit(&self) -> FifoOverflowBitR {
        FifoOverflowBitR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_overflow_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacOverflowCntSpec;
impl crate::RegisterSpec for GmacOverflowCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_overflow_cnt::R`](R) reader structure"]
impl crate::Readable for GmacOverflowCntSpec {}
#[doc = "`reset()` method sets GMAC_OVERFLOW_CNT to value 0"]
impl crate::Resettable for GmacOverflowCntSpec {
    const RESET_VALUE: u32 = 0;
}
