#[doc = "Register `INTR_RAW_STATUS1` reader"]
pub type R = crate::R<IntrRawStatus1Spec>;
#[doc = "Register `INTR_RAW_STATUS1` writer"]
pub type W = crate::W<IntrRawStatus1Spec>;
#[doc = "Field `INT_RAW_STATUS_FBCD0` reader - interrupt raw status"]
pub type IntRawStatusFbcd0R = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_FBCD1` reader - interrupt raw status"]
pub type IntRawStatusFbcd1R = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_FBCD1` writer - interrupt raw status"]
pub type IntRawStatusFbcd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_FBCD2` reader - interrupt raw status"]
pub type IntRawStatusFbcd2R = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_FBCD2` writer - interrupt raw status"]
pub type IntRawStatusFbcd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_FBCD3` reader - interrupt raw status"]
pub type IntRawStatusFbcd3R = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_FBCD3` writer - interrupt raw status"]
pub type IntRawStatusFbcd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD0_HREG_DEC_RESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd0HregDecRespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD0_HREG_DEC_RESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd0HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD0_HREG_AXI_RRESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd0HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD0_HREG_AXI_RRESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd0HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD1_HREG_DEC_RESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd1HregDecRespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD1_HREG_DEC_RESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd1HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD1_HREG_AXI_RRESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd1HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD1_HREG_AXI_RRESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd1HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD2_HREG_DEC_RESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd2HregDecRespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD2_HREG_DEC_RESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd2HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD2_HREG_AXI_RRESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd2HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD2_HREG_AXI_RRESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd2HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD3_HREG_DEC_RESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd3HregDecRespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD3_HREG_DEC_RESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd3HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_AFBCD3_HREG_AXI_RRESP` reader - interrupt raw status"]
pub type IntRawStatusAfbcd3HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_AFBCD3_HREG_AXI_RRESP` writer - interrupt raw status"]
pub type IntRawStatusAfbcd3HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_WB_YRGB_FIFO_FULL` reader - int_raw_status_wb_yrgb_fifo_full"]
pub type IntRawStatusWbYrgbFifoFullR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WB_YRGB_FIFO_FULL` writer - int_raw_status_wb_yrgb_fifo_full"]
pub type IntRawStatusWbYrgbFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_WB_UV_FIFO_FULL` reader - int_raw_status_wb_uv_fifo_full"]
pub type IntRawStatusWbUvFifoFullR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WB_UV_FIFO_FULL` writer - int_raw_status_wb_uv_fifo_full"]
pub type IntRawStatusWbUvFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_WB_DMA_FINISH` reader - int_raw_status_wb_dma_finish"]
pub type IntRawStatusWbDmaFinishR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WB_DMA_FINISH` writer - int_raw_status_wb_dma_finish"]
pub type IntRawStatusWbDmaFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RAW_STATUS_VFP` reader - int_raw_status_vfp"]
pub type IntRawStatusVfpR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_VFP` writer - int_raw_status_vfp"]
pub type IntRawStatusVfpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_fbcd0(&self) -> IntRawStatusFbcd0R {
        IntRawStatusFbcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_fbcd1(&self) -> IntRawStatusFbcd1R {
        IntRawStatusFbcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_fbcd2(&self) -> IntRawStatusFbcd2R {
        IntRawStatusFbcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_fbcd3(&self) -> IntRawStatusFbcd3R {
        IntRawStatusFbcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd0_hreg_dec_resp(&self) -> IntRawStatusAfbcd0HregDecRespR {
        IntRawStatusAfbcd0HregDecRespR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd0_hreg_axi_rresp(&self) -> IntRawStatusAfbcd0HregAxiRrespR {
        IntRawStatusAfbcd0HregAxiRrespR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd1_hreg_dec_resp(&self) -> IntRawStatusAfbcd1HregDecRespR {
        IntRawStatusAfbcd1HregDecRespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd1_hreg_axi_rresp(&self) -> IntRawStatusAfbcd1HregAxiRrespR {
        IntRawStatusAfbcd1HregAxiRrespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd2_hreg_dec_resp(&self) -> IntRawStatusAfbcd2HregDecRespR {
        IntRawStatusAfbcd2HregDecRespR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd2_hreg_axi_rresp(&self) -> IntRawStatusAfbcd2HregAxiRrespR {
        IntRawStatusAfbcd2HregAxiRrespR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd3_hreg_dec_resp(&self) -> IntRawStatusAfbcd3HregDecRespR {
        IntRawStatusAfbcd3HregDecRespR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_afbcd3_hreg_axi_rresp(&self) -> IntRawStatusAfbcd3HregAxiRrespR {
        IntRawStatusAfbcd3HregAxiRrespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - int_raw_status_wb_yrgb_fifo_full"]
    #[inline(always)]
    pub fn int_raw_status_wb_yrgb_fifo_full(&self) -> IntRawStatusWbYrgbFifoFullR {
        IntRawStatusWbYrgbFifoFullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - int_raw_status_wb_uv_fifo_full"]
    #[inline(always)]
    pub fn int_raw_status_wb_uv_fifo_full(&self) -> IntRawStatusWbUvFifoFullR {
        IntRawStatusWbUvFifoFullR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - int_raw_status_wb_dma_finish"]
    #[inline(always)]
    pub fn int_raw_status_wb_dma_finish(&self) -> IntRawStatusWbDmaFinishR {
        IntRawStatusWbDmaFinishR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - int_raw_status_vfp"]
    #[inline(always)]
    pub fn int_raw_status_vfp(&self) -> IntRawStatusVfpR {
        IntRawStatusVfpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_fbcd1(&mut self) -> IntRawStatusFbcd1W<IntrRawStatus1Spec> {
        IntRawStatusFbcd1W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_fbcd2(&mut self) -> IntRawStatusFbcd2W<IntrRawStatus1Spec> {
        IntRawStatusFbcd2W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_fbcd3(&mut self) -> IntRawStatusFbcd3W<IntrRawStatus1Spec> {
        IntRawStatusFbcd3W::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd0_hreg_dec_resp(
        &mut self,
    ) -> IntRawStatusAfbcd0HregDecRespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd0HregDecRespW::new(self, 4)
    }
    #[doc = "Bit 5 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd0_hreg_axi_rresp(
        &mut self,
    ) -> IntRawStatusAfbcd0HregAxiRrespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd0HregAxiRrespW::new(self, 5)
    }
    #[doc = "Bit 6 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd1_hreg_dec_resp(
        &mut self,
    ) -> IntRawStatusAfbcd1HregDecRespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd1HregDecRespW::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd1_hreg_axi_rresp(
        &mut self,
    ) -> IntRawStatusAfbcd1HregAxiRrespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd1HregAxiRrespW::new(self, 7)
    }
    #[doc = "Bit 8 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd2_hreg_dec_resp(
        &mut self,
    ) -> IntRawStatusAfbcd2HregDecRespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd2HregDecRespW::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd2_hreg_axi_rresp(
        &mut self,
    ) -> IntRawStatusAfbcd2HregAxiRrespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd2HregAxiRrespW::new(self, 9)
    }
    #[doc = "Bit 10 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd3_hreg_dec_resp(
        &mut self,
    ) -> IntRawStatusAfbcd3HregDecRespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd3HregDecRespW::new(self, 10)
    }
    #[doc = "Bit 11 - interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_afbcd3_hreg_axi_rresp(
        &mut self,
    ) -> IntRawStatusAfbcd3HregAxiRrespW<IntrRawStatus1Spec> {
        IntRawStatusAfbcd3HregAxiRrespW::new(self, 11)
    }
    #[doc = "Bit 12 - int_raw_status_wb_yrgb_fifo_full"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_wb_yrgb_fifo_full(
        &mut self,
    ) -> IntRawStatusWbYrgbFifoFullW<IntrRawStatus1Spec> {
        IntRawStatusWbYrgbFifoFullW::new(self, 12)
    }
    #[doc = "Bit 13 - int_raw_status_wb_uv_fifo_full"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_wb_uv_fifo_full(
        &mut self,
    ) -> IntRawStatusWbUvFifoFullW<IntrRawStatus1Spec> {
        IntRawStatusWbUvFifoFullW::new(self, 13)
    }
    #[doc = "Bit 14 - int_raw_status_wb_dma_finish"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_wb_dma_finish(&mut self) -> IntRawStatusWbDmaFinishW<IntrRawStatus1Spec> {
        IntRawStatusWbDmaFinishW::new(self, 14)
    }
    #[doc = "Bit 15 - int_raw_status_vfp"]
    #[inline(always)]
    #[must_use]
    pub fn int_raw_status_vfp(&mut self) -> IntRawStatusVfpW<IntrRawStatus1Spec> {
        IntRawStatusVfpW::new(self, 15)
    }
}
#[doc = "raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_raw_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRawStatus1Spec;
impl crate::RegisterSpec for IntrRawStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw_status1::R`](R) reader structure"]
impl crate::Readable for IntrRawStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_raw_status1::W`](W) writer structure"]
impl crate::Writable for IntrRawStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_RAW_STATUS1 to value 0"]
impl crate::Resettable for IntrRawStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
