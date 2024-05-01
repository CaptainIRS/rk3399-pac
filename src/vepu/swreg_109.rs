#[doc = "Register `SWREG_109` reader"]
pub type R = crate::R<Swreg109Spec>;
#[doc = "Register `SWREG_109` writer"]
pub type W = crate::W<Swreg109Spec>;
#[doc = "Field `ENC_IRQ` reader - enc interrupt"]
pub type EncIrqR = crate::BitReader;
#[doc = "Field `ENC_IRQ` writer - enc interrupt"]
pub type EncIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_FRAME_RDY` reader - one frame encoder sucess flag"]
pub type IrqFrameRdyR = crate::BitReader;
#[doc = "Field `IRQ_FRAME_RDY` writer - one frame encoder sucess flag"]
pub type IrqFrameRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_SLICE_READY` reader - slice ready flag"]
pub type IrqSliceReadyR = crate::BitReader;
#[doc = "Field `IRQ_SLICE_READY` writer - slice ready flag"]
pub type IrqSliceReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_INT` reader - Field0000 Abstract\n\nField0000 Description"]
pub type FuseIntR = crate::BitReader;
#[doc = "Field `FUSE_INT` writer - Field0000 Abstract\n\nField0000 Description"]
pub type FuseIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_BUS_ERROR` reader - bus error irq"]
pub type IrqBusErrorR = crate::BitReader;
#[doc = "Field `IRQ_BUS_ERROR` writer - bus error irq"]
pub type IrqBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_BUFFER_FULL` reader - buffer full flag"]
pub type IrqBufferFullR = crate::BitReader;
#[doc = "Field `IRQ_BUFFER_FULL` writer - buffer full flag"]
pub type IrqBufferFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_TIMEOUT` reader - HW wait timeout flag"]
pub type IrqTimeoutR = crate::BitReader;
#[doc = "Field `IRQ_TIMEOUT` writer - HW wait timeout flag"]
pub type IrqTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_DIS` reader - irq disable"]
pub type IrqDisR = crate::BitReader;
#[doc = "Field `IRQ_DIS` writer - irq disable"]
pub type IrqDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_CLR` reader - irq clear"]
pub type IrqClrR = crate::BitReader;
#[doc = "Field `IRQ_CLR` writer - irq clear"]
pub type IrqClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_TIMEOUT_EN` reader - enable interrupt for timeout"]
pub type IntTimeoutEnR = crate::BitReader;
#[doc = "Field `INT_TIMEOUT_EN` writer - enable interrupt for timeout"]
pub type IntTimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GATING_EN` reader - clock gating enable flag\n\ndefault clk_gating_en =1'b1"]
pub type ClkGatingEnR = crate::BitReader;
#[doc = "Field `CLK_GATING_EN` writer - clock gating enable flag\n\ndefault clk_gating_en =1'b1"]
pub type ClkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLICE_RDYINT_EN` reader - enable slice ready interruupt\n\nenable slice ready interruupt"]
pub type SliceRdyintEnR = crate::BitReader;
#[doc = "Field `SLICE_RDYINT_EN` writer - enable slice ready interruupt\n\nenable slice ready interruupt"]
pub type SliceRdyintEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROCON_WRITE_DIS` reader - write reconstructed image disable flag"]
pub type RoconWriteDisR = crate::BitReader;
#[doc = "Field `ROCON_WRITE_DIS` writer - write reconstructed image disable flag"]
pub type RoconWriteDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MV_SAD_WREN` reader - the each MB MV and SAD be writed to mv_wr_st_adr enable"]
pub type MvSadWrenR = crate::BitReader;
#[doc = "Field `MV_SAD_WREN` writer - the each MB MV and SAD be writed to mv_wr_st_adr enable"]
pub type MvSadWrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_NON` reader - Field0000 Abstract\n\nField0000 Description"]
pub type IntNonR = crate::BitReader;
#[doc = "Field `INT_NON` writer - Field0000 Abstract\n\nField0000 Description"]
pub type IntNonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enc interrupt"]
    #[inline(always)]
    pub fn enc_irq(&self) -> EncIrqR {
        EncIrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - one frame encoder sucess flag"]
    #[inline(always)]
    pub fn irq_frame_rdy(&self) -> IrqFrameRdyR {
        IrqFrameRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - slice ready flag"]
    #[inline(always)]
    pub fn irq_slice_ready(&self) -> IrqSliceReadyR {
        IrqSliceReadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    pub fn fuse_int(&self) -> FuseIntR {
        FuseIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - bus error irq"]
    #[inline(always)]
    pub fn irq_bus_error(&self) -> IrqBusErrorR {
        IrqBusErrorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - buffer full flag"]
    #[inline(always)]
    pub fn irq_buffer_full(&self) -> IrqBufferFullR {
        IrqBufferFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HW wait timeout flag"]
    #[inline(always)]
    pub fn irq_timeout(&self) -> IrqTimeoutR {
        IrqTimeoutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - irq disable"]
    #[inline(always)]
    pub fn irq_dis(&self) -> IrqDisR {
        IrqDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - irq clear"]
    #[inline(always)]
    pub fn irq_clr(&self) -> IrqClrR {
        IrqClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable interrupt for timeout"]
    #[inline(always)]
    pub fn int_timeout_en(&self) -> IntTimeoutEnR {
        IntTimeoutEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - clock gating enable flag\n\ndefault clk_gating_en =1'b1"]
    #[inline(always)]
    pub fn clk_gating_en(&self) -> ClkGatingEnR {
        ClkGatingEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - enable slice ready interruupt\n\nenable slice ready interruupt"]
    #[inline(always)]
    pub fn slice_rdyint_en(&self) -> SliceRdyintEnR {
        SliceRdyintEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - write reconstructed image disable flag"]
    #[inline(always)]
    pub fn rocon_write_dis(&self) -> RoconWriteDisR {
        RoconWriteDisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - the each MB MV and SAD be writed to mv_wr_st_adr enable"]
    #[inline(always)]
    pub fn mv_sad_wren(&self) -> MvSadWrenR {
        MvSadWrenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    pub fn int_non(&self) -> IntNonR {
        IntNonR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enc interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enc_irq(&mut self) -> EncIrqW<Swreg109Spec> {
        EncIrqW::new(self, 0)
    }
    #[doc = "Bit 1 - one frame encoder sucess flag"]
    #[inline(always)]
    #[must_use]
    pub fn irq_frame_rdy(&mut self) -> IrqFrameRdyW<Swreg109Spec> {
        IrqFrameRdyW::new(self, 1)
    }
    #[doc = "Bit 2 - slice ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn irq_slice_ready(&mut self) -> IrqSliceReadyW<Swreg109Spec> {
        IrqSliceReadyW::new(self, 2)
    }
    #[doc = "Bit 3 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_int(&mut self) -> FuseIntW<Swreg109Spec> {
        FuseIntW::new(self, 3)
    }
    #[doc = "Bit 4 - bus error irq"]
    #[inline(always)]
    #[must_use]
    pub fn irq_bus_error(&mut self) -> IrqBusErrorW<Swreg109Spec> {
        IrqBusErrorW::new(self, 4)
    }
    #[doc = "Bit 5 - buffer full flag"]
    #[inline(always)]
    #[must_use]
    pub fn irq_buffer_full(&mut self) -> IrqBufferFullW<Swreg109Spec> {
        IrqBufferFullW::new(self, 5)
    }
    #[doc = "Bit 6 - HW wait timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn irq_timeout(&mut self) -> IrqTimeoutW<Swreg109Spec> {
        IrqTimeoutW::new(self, 6)
    }
    #[doc = "Bit 8 - irq disable"]
    #[inline(always)]
    #[must_use]
    pub fn irq_dis(&mut self) -> IrqDisW<Swreg109Spec> {
        IrqDisW::new(self, 8)
    }
    #[doc = "Bit 9 - irq clear"]
    #[inline(always)]
    #[must_use]
    pub fn irq_clr(&mut self) -> IrqClrW<Swreg109Spec> {
        IrqClrW::new(self, 9)
    }
    #[doc = "Bit 10 - enable interrupt for timeout"]
    #[inline(always)]
    #[must_use]
    pub fn int_timeout_en(&mut self) -> IntTimeoutEnW<Swreg109Spec> {
        IntTimeoutEnW::new(self, 10)
    }
    #[doc = "Bit 12 - clock gating enable flag\n\ndefault clk_gating_en =1'b1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gating_en(&mut self) -> ClkGatingEnW<Swreg109Spec> {
        ClkGatingEnW::new(self, 12)
    }
    #[doc = "Bit 16 - enable slice ready interruupt\n\nenable slice ready interruupt"]
    #[inline(always)]
    #[must_use]
    pub fn slice_rdyint_en(&mut self) -> SliceRdyintEnW<Swreg109Spec> {
        SliceRdyintEnW::new(self, 16)
    }
    #[doc = "Bit 20 - write reconstructed image disable flag"]
    #[inline(always)]
    #[must_use]
    pub fn rocon_write_dis(&mut self) -> RoconWriteDisW<Swreg109Spec> {
        RoconWriteDisW::new(self, 20)
    }
    #[doc = "Bit 24 - the each MB MV and SAD be writed to mv_wr_st_adr enable"]
    #[inline(always)]
    #[must_use]
    pub fn mv_sad_wren(&mut self) -> MvSadWrenW<Swreg109Spec> {
        MvSadWrenW::new(self, 24)
    }
    #[doc = "Bit 28 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    #[must_use]
    pub fn int_non(&mut self) -> IntNonW<Swreg109Spec> {
        IntNonW::new(self, 28)
    }
}
#[doc = "encoder status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_109::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_109::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg109Spec;
impl crate::RegisterSpec for Swreg109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_109::R`](R) reader structure"]
impl crate::Readable for Swreg109Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_109::W`](W) writer structure"]
impl crate::Writable for Swreg109Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_109 to value 0x1000"]
impl crate::Resettable for Swreg109Spec {
    const RESET_VALUE: u32 = 0x1000;
}
