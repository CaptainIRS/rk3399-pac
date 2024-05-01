#[doc = "Register `INTR_CLEAR1` reader"]
pub type R = crate::R<IntrClear1Spec>;
#[doc = "Register `INTR_CLEAR1` writer"]
pub type W = crate::W<IntrClear1Spec>;
#[doc = "Field `INT_CLR_FBCD0` reader - interrupt clear (Auto clear)"]
pub type IntClrFbcd0R = crate::BitReader;
#[doc = "Field `INT_CLR_FBCD0` writer - interrupt clear (Auto clear)"]
pub type IntClrFbcd0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_FBCD1` reader - interrupt clear (Auto clear)"]
pub type IntClrFbcd1R = crate::BitReader;
#[doc = "Field `INT_CLR_FBCD1` writer - interrupt clear (Auto clear)"]
pub type IntClrFbcd1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_FBCD2` reader - interrupt clear (Auto clear)"]
pub type IntClrFbcd2R = crate::BitReader;
#[doc = "Field `INT_CLR_FBCD2` writer - interrupt clear (Auto clear)"]
pub type IntClrFbcd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_FBCD3` reader - interrupt clear (Auto clear)"]
pub type IntClrFbcd3R = crate::BitReader;
#[doc = "Field `INT_CLR_FBCD3` writer - interrupt clear (Auto clear)"]
pub type IntClrFbcd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD0_HREG_DEC_RESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd0HregDecRespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD0_HREG_DEC_RESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd0HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD0_HREG_AXI_RRESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd0HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD0_HREG_AXI_RRESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd0HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD1_HREG_DEC_RESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd1HregDecRespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD1_HREG_DEC_RESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd1HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD1_HREG_AXI_RRESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd1HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD1_HREG_AXI_RRESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd1HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD2_HREG_DEC_RESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd2HregDecRespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD2_HREG_DEC_RESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd2HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD2_HREG_AXI_RRESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd2HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD2_HREG_AXI_RRESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd2HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD3_HREG_DEC_RESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd3HregDecRespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD3_HREG_DEC_RESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd3HregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_AFBCD3_HREG_AXI_RRESP` reader - interrupt clear (Auto clear)"]
pub type IntClrAfbcd3HregAxiRrespR = crate::BitReader;
#[doc = "Field `INT_CLR_AFBCD3_HREG_AXI_RRESP` writer - interrupt clear (Auto clear)"]
pub type IntClrAfbcd3HregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_WB_YRGB_FIFO_FULL` reader - int_clr_wb_yrgb_fifo_full"]
pub type IntClrWbYrgbFifoFullR = crate::BitReader;
#[doc = "Field `INT_CLR_WB_YRGB_FIFO_FULL` writer - int_clr_wb_yrgb_fifo_full"]
pub type IntClrWbYrgbFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_WB_UV_FIFO_FULL` reader - int_clr_wb_uv_fifo_full"]
pub type IntClrWbUvFifoFullR = crate::BitReader;
#[doc = "Field `INT_CLR_WB_UV_FIFO_FULL` writer - int_clr_wb_uv_fifo_full"]
pub type IntClrWbUvFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_WB_DMA_FINISH` reader - int_clr_wb_dma_finish"]
pub type IntClrWbDmaFinishR = crate::BitReader;
#[doc = "Field `INT_CLR_WB_DMA_FINISH` writer - int_clr_wb_dma_finish"]
pub type IntClrWbDmaFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_VFP` reader - int_clr_vfp"]
pub type IntClrVfpR = crate::BitReader;
#[doc = "Field `INT_CLR_VFP` writer - int_clr_vfp"]
pub type IntClrVfpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fbcd0(&self) -> IntClrFbcd0R {
        IntClrFbcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fbcd1(&self) -> IntClrFbcd1R {
        IntClrFbcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fbcd2(&self) -> IntClrFbcd2R {
        IntClrFbcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fbcd3(&self) -> IntClrFbcd3R {
        IntClrFbcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd0_hreg_dec_resp(&self) -> IntClrAfbcd0HregDecRespR {
        IntClrAfbcd0HregDecRespR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd0_hreg_axi_rresp(&self) -> IntClrAfbcd0HregAxiRrespR {
        IntClrAfbcd0HregAxiRrespR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd1_hreg_dec_resp(&self) -> IntClrAfbcd1HregDecRespR {
        IntClrAfbcd1HregDecRespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd1_hreg_axi_rresp(&self) -> IntClrAfbcd1HregAxiRrespR {
        IntClrAfbcd1HregAxiRrespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd2_hreg_dec_resp(&self) -> IntClrAfbcd2HregDecRespR {
        IntClrAfbcd2HregDecRespR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd2_hreg_axi_rresp(&self) -> IntClrAfbcd2HregAxiRrespR {
        IntClrAfbcd2HregAxiRrespR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd3_hreg_dec_resp(&self) -> IntClrAfbcd3HregDecRespR {
        IntClrAfbcd3HregDecRespR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_afbcd3_hreg_axi_rresp(&self) -> IntClrAfbcd3HregAxiRrespR {
        IntClrAfbcd3HregAxiRrespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - int_clr_wb_yrgb_fifo_full"]
    #[inline(always)]
    pub fn int_clr_wb_yrgb_fifo_full(&self) -> IntClrWbYrgbFifoFullR {
        IntClrWbYrgbFifoFullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - int_clr_wb_uv_fifo_full"]
    #[inline(always)]
    pub fn int_clr_wb_uv_fifo_full(&self) -> IntClrWbUvFifoFullR {
        IntClrWbUvFifoFullR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - int_clr_wb_dma_finish"]
    #[inline(always)]
    pub fn int_clr_wb_dma_finish(&self) -> IntClrWbDmaFinishR {
        IntClrWbDmaFinishR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - int_clr_vfp"]
    #[inline(always)]
    pub fn int_clr_vfp(&self) -> IntClrVfpR {
        IntClrVfpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fbcd0(&mut self) -> IntClrFbcd0W<IntrClear1Spec> {
        IntClrFbcd0W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fbcd1(&mut self) -> IntClrFbcd1W<IntrClear1Spec> {
        IntClrFbcd1W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fbcd2(&mut self) -> IntClrFbcd2W<IntrClear1Spec> {
        IntClrFbcd2W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fbcd3(&mut self) -> IntClrFbcd3W<IntrClear1Spec> {
        IntClrFbcd3W::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd0_hreg_dec_resp(&mut self) -> IntClrAfbcd0HregDecRespW<IntrClear1Spec> {
        IntClrAfbcd0HregDecRespW::new(self, 4)
    }
    #[doc = "Bit 5 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd0_hreg_axi_rresp(&mut self) -> IntClrAfbcd0HregAxiRrespW<IntrClear1Spec> {
        IntClrAfbcd0HregAxiRrespW::new(self, 5)
    }
    #[doc = "Bit 6 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd1_hreg_dec_resp(&mut self) -> IntClrAfbcd1HregDecRespW<IntrClear1Spec> {
        IntClrAfbcd1HregDecRespW::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd1_hreg_axi_rresp(&mut self) -> IntClrAfbcd1HregAxiRrespW<IntrClear1Spec> {
        IntClrAfbcd1HregAxiRrespW::new(self, 7)
    }
    #[doc = "Bit 8 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd2_hreg_dec_resp(&mut self) -> IntClrAfbcd2HregDecRespW<IntrClear1Spec> {
        IntClrAfbcd2HregDecRespW::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd2_hreg_axi_rresp(&mut self) -> IntClrAfbcd2HregAxiRrespW<IntrClear1Spec> {
        IntClrAfbcd2HregAxiRrespW::new(self, 9)
    }
    #[doc = "Bit 10 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd3_hreg_dec_resp(&mut self) -> IntClrAfbcd3HregDecRespW<IntrClear1Spec> {
        IntClrAfbcd3HregDecRespW::new(self, 10)
    }
    #[doc = "Bit 11 - interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_afbcd3_hreg_axi_rresp(&mut self) -> IntClrAfbcd3HregAxiRrespW<IntrClear1Spec> {
        IntClrAfbcd3HregAxiRrespW::new(self, 11)
    }
    #[doc = "Bit 12 - int_clr_wb_yrgb_fifo_full"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_wb_yrgb_fifo_full(&mut self) -> IntClrWbYrgbFifoFullW<IntrClear1Spec> {
        IntClrWbYrgbFifoFullW::new(self, 12)
    }
    #[doc = "Bit 13 - int_clr_wb_uv_fifo_full"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_wb_uv_fifo_full(&mut self) -> IntClrWbUvFifoFullW<IntrClear1Spec> {
        IntClrWbUvFifoFullW::new(self, 13)
    }
    #[doc = "Bit 14 - int_clr_wb_dma_finish"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_wb_dma_finish(&mut self) -> IntClrWbDmaFinishW<IntrClear1Spec> {
        IntClrWbDmaFinishW::new(self, 14)
    }
    #[doc = "Bit 15 - int_clr_vfp"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_vfp(&mut self) -> IntClrVfpW<IntrClear1Spec> {
        IntClrVfpW::new(self, 15)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_clear1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clear1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrClear1Spec;
impl crate::RegisterSpec for IntrClear1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_clear1::R`](R) reader structure"]
impl crate::Readable for IntrClear1Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_clear1::W`](W) writer structure"]
impl crate::Writable for IntrClear1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INTR_CLEAR1 to value 0"]
impl crate::Resettable for IntrClear1Spec {
    const RESET_VALUE: u32 = 0;
}
