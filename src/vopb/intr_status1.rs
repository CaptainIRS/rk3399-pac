#[doc = "Register `INTR_STATUS1` reader"]
pub type R = crate::R<IntrStatus1Spec>;
#[doc = "Register `INTR_STATUS1` writer"]
pub type W = crate::W<IntrStatus1Spec>;
#[doc = "Field `INT_STATUS_FBCD0` reader - interrupt status"]
pub type IntStatusFbcd0R = crate::BitReader;
#[doc = "Field `INT_STATUS_FBCD1` reader - interrupt status"]
pub type IntStatusFbcd1R = crate::BitReader;
#[doc = "Field `INT_STATUS_FBCD2` reader - interrupt status"]
pub type IntStatusFbcd2R = crate::BitReader;
#[doc = "Field `INT_STATUS_FBCD2` writer - interrupt status"]
pub type IntStatusFbcd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_FBCD3` reader - interrupt status"]
pub type IntStatusFbcd3R = crate::BitReader;
#[doc = "Field `INT_STATUS_FBCD3` writer - interrupt status"]
pub type IntStatusFbcd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD0_HREG_DEC_RESP` reader - interrupt status"]
pub type IntStatusAfbcd0HregDecRespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD0_HREG_DEC_RESP` writer - interrupt status"]
pub type IntStatusAfbcd0HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD0_HREG_AXI_RRESP` reader - interrupt status"]
pub type IntStatusAfbcd0HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD0_HREG_AXI_RRESP` writer - interrupt status"]
pub type IntStatusAfbcd0HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD1_HREG_DEC_RESP` reader - interrupt status"]
pub type IntStatusAfbcd1HregDecRespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD1_HREG_DEC_RESP` writer - interrupt status"]
pub type IntStatusAfbcd1HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD1_HREG_AXI_RRESP` reader - interrupt status"]
pub type IntStatusAfbcd1HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD1_HREG_AXI_RRESP` writer - interrupt status"]
pub type IntStatusAfbcd1HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD2_HREG_DEC_RESP` reader - interrupt status"]
pub type IntStatusAfbcd2HregDecRespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD2_HREG_DEC_RESP` writer - interrupt status"]
pub type IntStatusAfbcd2HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD2_HREG_AXI_RRESP` reader - interrupt status"]
pub type IntStatusAfbcd2HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD2_HREG_AXI_RRESP` writer - interrupt status"]
pub type IntStatusAfbcd2HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD3_HREG_DEC_RESP` reader - interrupt status"]
pub type IntStatusAfbcd3HregDecRespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD3_HREG_DEC_RESP` writer - interrupt status"]
pub type IntStatusAfbcd3HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_AFBCD4_HREG_DEC_RESP` reader - interrupt status"]
pub type IntStatusAfbcd4HregDecRespR = crate::BitReader;
#[doc = "Field `INT_STATUS_AFBCD4_HREG_DEC_RESP` writer - interrupt status"]
pub type IntStatusAfbcd4HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_WB_YRGB_FIFO_FULL` reader - int_status_wb_yrgb_fifo_full"]
pub type IntStatusWbYrgbFifoFullR = crate::BitReader;
#[doc = "Field `INT_STATUS_WB_YRGB_FIFO_FULL` writer - int_status_wb_yrgb_fifo_full"]
pub type IntStatusWbYrgbFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_WB_UV_FIFO_FULL` reader - int_status_wb_uv_fifo_full"]
pub type IntStatusWbUvFifoFullR = crate::BitReader;
#[doc = "Field `INT_STATUS_WB_UV_FIFO_FULL` writer - int_status_wb_uv_fifo_full"]
pub type IntStatusWbUvFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_WB_DMA_FINISH` reader - int_status_wb_dma_finish"]
pub type IntStatusWbDmaFinishR = crate::BitReader;
#[doc = "Field `INT_STATUS_WB_DMA_FINISH` writer - int_status_wb_dma_finish"]
pub type IntStatusWbDmaFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_VFP` reader - int_status_vfp"]
pub type IntStatusVfpR = crate::BitReader;
#[doc = "Field `INT_STATUS_VFP` writer - int_status_vfp"]
pub type IntStatusVfpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt status"]
    #[inline(always)]
    pub fn int_status_fbcd0(&self) -> IntStatusFbcd0R {
        IntStatusFbcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt status"]
    #[inline(always)]
    pub fn int_status_fbcd1(&self) -> IntStatusFbcd1R {
        IntStatusFbcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt status"]
    #[inline(always)]
    pub fn int_status_fbcd2(&self) -> IntStatusFbcd2R {
        IntStatusFbcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt status"]
    #[inline(always)]
    pub fn int_status_fbcd3(&self) -> IntStatusFbcd3R {
        IntStatusFbcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd0_hreg_dec_resp(&self) -> IntStatusAfbcd0HregDecRespR {
        IntStatusAfbcd0HregDecRespR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd0_hreg_axi_rresp(&self) -> IntStatusAfbcd0HregAxiRrespR {
        IntStatusAfbcd0HregAxiRrespR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd1_hreg_dec_resp(&self) -> IntStatusAfbcd1HregDecRespR {
        IntStatusAfbcd1HregDecRespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd1_hreg_axi_rresp(&self) -> IntStatusAfbcd1HregAxiRrespR {
        IntStatusAfbcd1HregAxiRrespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd2_hreg_dec_resp(&self) -> IntStatusAfbcd2HregDecRespR {
        IntStatusAfbcd2HregDecRespR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd2_hreg_axi_rresp(&self) -> IntStatusAfbcd2HregAxiRrespR {
        IntStatusAfbcd2HregAxiRrespR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd3_hreg_dec_resp(&self) -> IntStatusAfbcd3HregDecRespR {
        IntStatusAfbcd3HregDecRespR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - interrupt status"]
    #[inline(always)]
    pub fn int_status_afbcd4_hreg_dec_resp(&self) -> IntStatusAfbcd4HregDecRespR {
        IntStatusAfbcd4HregDecRespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - int_status_wb_yrgb_fifo_full"]
    #[inline(always)]
    pub fn int_status_wb_yrgb_fifo_full(&self) -> IntStatusWbYrgbFifoFullR {
        IntStatusWbYrgbFifoFullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - int_status_wb_uv_fifo_full"]
    #[inline(always)]
    pub fn int_status_wb_uv_fifo_full(&self) -> IntStatusWbUvFifoFullR {
        IntStatusWbUvFifoFullR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - int_status_wb_dma_finish"]
    #[inline(always)]
    pub fn int_status_wb_dma_finish(&self) -> IntStatusWbDmaFinishR {
        IntStatusWbDmaFinishR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - int_status_vfp"]
    #[inline(always)]
    pub fn int_status_vfp(&self) -> IntStatusVfpR {
        IntStatusVfpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_fbcd2(&mut self) -> IntStatusFbcd2W<IntrStatus1Spec> {
        IntStatusFbcd2W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_fbcd3(&mut self) -> IntStatusFbcd3W<IntrStatus1Spec> {
        IntStatusFbcd3W::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd0_hreg_dec_resp(
        &mut self,
    ) -> IntStatusAfbcd0HregDecRespW<IntrStatus1Spec> {
        IntStatusAfbcd0HregDecRespW::new(self, 4)
    }
    #[doc = "Bit 5 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd0_hreg_axi_rresp(
        &mut self,
    ) -> IntStatusAfbcd0HregAxiRrespW<IntrStatus1Spec> {
        IntStatusAfbcd0HregAxiRrespW::new(self, 5)
    }
    #[doc = "Bit 6 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd1_hreg_dec_resp(
        &mut self,
    ) -> IntStatusAfbcd1HregDecRespW<IntrStatus1Spec> {
        IntStatusAfbcd1HregDecRespW::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd1_hreg_axi_rresp(
        &mut self,
    ) -> IntStatusAfbcd1HregAxiRrespW<IntrStatus1Spec> {
        IntStatusAfbcd1HregAxiRrespW::new(self, 7)
    }
    #[doc = "Bit 8 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd2_hreg_dec_resp(
        &mut self,
    ) -> IntStatusAfbcd2HregDecRespW<IntrStatus1Spec> {
        IntStatusAfbcd2HregDecRespW::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd2_hreg_axi_rresp(
        &mut self,
    ) -> IntStatusAfbcd2HregAxiRrespW<IntrStatus1Spec> {
        IntStatusAfbcd2HregAxiRrespW::new(self, 9)
    }
    #[doc = "Bit 10 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd3_hreg_dec_resp(
        &mut self,
    ) -> IntStatusAfbcd3HregDecRespW<IntrStatus1Spec> {
        IntStatusAfbcd3HregDecRespW::new(self, 10)
    }
    #[doc = "Bit 11 - interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_afbcd4_hreg_dec_resp(
        &mut self,
    ) -> IntStatusAfbcd4HregDecRespW<IntrStatus1Spec> {
        IntStatusAfbcd4HregDecRespW::new(self, 11)
    }
    #[doc = "Bit 12 - int_status_wb_yrgb_fifo_full"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_wb_yrgb_fifo_full(&mut self) -> IntStatusWbYrgbFifoFullW<IntrStatus1Spec> {
        IntStatusWbYrgbFifoFullW::new(self, 12)
    }
    #[doc = "Bit 13 - int_status_wb_uv_fifo_full"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_wb_uv_fifo_full(&mut self) -> IntStatusWbUvFifoFullW<IntrStatus1Spec> {
        IntStatusWbUvFifoFullW::new(self, 13)
    }
    #[doc = "Bit 14 - int_status_wb_dma_finish"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_wb_dma_finish(&mut self) -> IntStatusWbDmaFinishW<IntrStatus1Spec> {
        IntStatusWbDmaFinishW::new(self, 14)
    }
    #[doc = "Bit 15 - int_status_vfp"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_vfp(&mut self) -> IntStatusVfpW<IntrStatus1Spec> {
        IntStatusVfpW::new(self, 15)
    }
}
#[doc = "interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatus1Spec;
impl crate::RegisterSpec for IntrStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status1::R`](R) reader structure"]
impl crate::Readable for IntrStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_status1::W`](W) writer structure"]
impl crate::Writable for IntrStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_STATUS1 to value 0"]
impl crate::Resettable for IntrStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
