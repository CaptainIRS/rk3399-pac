#[doc = "Register `PCIE_DMA_INTERRUPT_DISABLE` reader"]
pub type R = crate::R<PcieDmaInterruptDisableSpec>;
#[doc = "Register `PCIE_DMA_INTERRUPT_DISABLE` writer"]
pub type W = crate::W<PcieDmaInterruptDisableSpec>;
#[doc = "Field `ch0_done_dis` reader - Channel 0 Done Disable Interrupt \\[ch0_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch0DoneDisR = crate::BitReader;
#[doc = "Field `ch0_done_dis` writer - Channel 0 Done Disable Interrupt \\[ch0_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch0DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch1_done_dis` reader - Channel 1 Done Disable Interrupt \\[ch1_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch1DoneDisR = crate::BitReader;
#[doc = "Field `ch1_done_dis` writer - Channel 1 Done Disable Interrupt \\[ch1_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch1DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch2_done_dis` reader - Channel 2 Done Disable Interrupt \\[ch2_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch2DoneDisR = crate::BitReader;
#[doc = "Field `ch2_done_dis` writer - Channel 2 Done Disable Interrupt \\[ch2_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch2DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch3_done_dis` reader - Channel 3 Done Disable Interrupt \\[ch3_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch3DoneDisR = crate::BitReader;
#[doc = "Field `ch3_done_dis` writer - Channel 3 Done Disable Interrupt \\[ch3_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch3DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch4_done_dis` reader - Channel 4 Done Disable Interrupt \\[ch4_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch4DoneDisR = crate::BitReader;
#[doc = "Field `ch4_done_dis` writer - Channel 4 Done Disable Interrupt \\[ch4_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch4DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch5_done_dis` reader - Channel 5 Done Disable Interrupt \\[ch5_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch5DoneDisR = crate::BitReader;
#[doc = "Field `ch5_done_dis` writer - Channel 5 Done Disable Interrupt \\[ch5_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch5DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch6_done_dis` reader - Channel 6 Done Disable Interrupt \\[ch6_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch6DoneDisR = crate::BitReader;
#[doc = "Field `ch6_done_dis` writer - Channel 6 Done Disable Interrupt \\[ch6_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch6DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch7_done_dis` reader - Channel 7 Done Disable Interrupt \\[ch7_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch7DoneDisR = crate::BitReader;
#[doc = "Field `ch7_done_dis` writer - Channel 7 Done Disable Interrupt \\[ch7_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
pub type Ch7DoneDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch0_error_dis` reader - Channel 0 Error Disable Interrupt \\[ch0_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch0ErrorDisR = crate::BitReader;
#[doc = "Field `ch0_error_dis` writer - Channel 0 Error Disable Interrupt \\[ch0_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch0ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch1_error_dis` reader - Channel 1 Error Disable Interrupt \\[ch1_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch1ErrorDisR = crate::BitReader;
#[doc = "Field `ch1_error_dis` writer - Channel 1 Error Disable Interrupt \\[ch1_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch1ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch2_error_dis` reader - Channel 2 Error Disable Interrupt \\[ch2_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch2ErrorDisR = crate::BitReader;
#[doc = "Field `ch2_error_dis` writer - Channel 2 Error Disable Interrupt \\[ch2_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch2ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch3_error_dis` reader - Channel 3 Error Disable Interrupt \\[ch3_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch3ErrorDisR = crate::BitReader;
#[doc = "Field `ch3_error_dis` writer - Channel 3 Error Disable Interrupt \\[ch3_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch3ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch4_error_dis` reader - Channel 4 Error Disable Interrupt \\[ch4_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch4ErrorDisR = crate::BitReader;
#[doc = "Field `ch4_error_dis` writer - Channel 4 Error Disable Interrupt \\[ch4_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch4ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch5_error_dis` reader - Channel 5 Error Disable Interrupt \\[ch5_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch5ErrorDisR = crate::BitReader;
#[doc = "Field `ch5_error_dis` writer - Channel 5 Error Disable Interrupt \\[ch5_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch5ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch6_error_dis` reader - Channel 6 Error Disable Interrupt \\[ch6_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch6ErrorDisR = crate::BitReader;
#[doc = "Field `ch6_error_dis` writer - Channel 6 Error Disable Interrupt \\[ch6_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch6ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ch7_error_dis` reader - Channel 7 Error Disable Interrupt \\[ch7_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch7ErrorDisR = crate::BitReader;
#[doc = "Field `ch7_error_dis` writer - Channel 7 Error Disable Interrupt \\[ch7_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
pub type Ch7ErrorDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Done Disable Interrupt \\[ch0_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch0_done_dis(&self) -> Ch0DoneDisR {
        Ch0DoneDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Done Disable Interrupt \\[ch1_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch1_done_dis(&self) -> Ch1DoneDisR {
        Ch1DoneDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Done Disable Interrupt \\[ch2_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch2_done_dis(&self) -> Ch2DoneDisR {
        Ch2DoneDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Done Disable Interrupt \\[ch3_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch3_done_dis(&self) -> Ch3DoneDisR {
        Ch3DoneDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Done Disable Interrupt \\[ch4_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch4_done_dis(&self) -> Ch4DoneDisR {
        Ch4DoneDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Done Disable Interrupt \\[ch5_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch5_done_dis(&self) -> Ch5DoneDisR {
        Ch5DoneDisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Done Disable Interrupt \\[ch6_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch6_done_dis(&self) -> Ch6DoneDisR {
        Ch6DoneDisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Done Disable Interrupt \\[ch7_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    pub fn ch7_done_dis(&self) -> Ch7DoneDisR {
        Ch7DoneDisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Error Disable Interrupt \\[ch0_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch0_error_dis(&self) -> Ch0ErrorDisR {
        Ch0ErrorDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Error Disable Interrupt \\[ch1_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch1_error_dis(&self) -> Ch1ErrorDisR {
        Ch1ErrorDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Error Disable Interrupt \\[ch2_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch2_error_dis(&self) -> Ch2ErrorDisR {
        Ch2ErrorDisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Error Disable Interrupt \\[ch3_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch3_error_dis(&self) -> Ch3ErrorDisR {
        Ch3ErrorDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Error Disable Interrupt \\[ch4_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch4_error_dis(&self) -> Ch4ErrorDisR {
        Ch4ErrorDisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Error Disable Interrupt \\[ch5_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch5_error_dis(&self) -> Ch5ErrorDisR {
        Ch5ErrorDisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Error Disable Interrupt \\[ch6_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch6_error_dis(&self) -> Ch6ErrorDisR {
        Ch6ErrorDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Error Disable Interrupt \\[ch7_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    pub fn ch7_error_dis(&self) -> Ch7ErrorDisR {
        Ch7ErrorDisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Done Disable Interrupt \\[ch0_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_done_dis(&mut self) -> Ch0DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch0DoneDisW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Done Disable Interrupt \\[ch1_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_done_dis(&mut self) -> Ch1DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch1DoneDisW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Done Disable Interrupt \\[ch2_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_done_dis(&mut self) -> Ch2DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch2DoneDisW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Done Disable Interrupt \\[ch3_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_done_dis(&mut self) -> Ch3DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch3DoneDisW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Done Disable Interrupt \\[ch4_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_done_dis(&mut self) -> Ch4DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch4DoneDisW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Done Disable Interrupt \\[ch5_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_done_dis(&mut self) -> Ch5DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch5DoneDisW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Done Disable Interrupt \\[ch6_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_done_dis(&mut self) -> Ch6DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch6DoneDisW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Done Disable Interrupt \\[ch7_done_dis\\]
Assert to 1 to disable done interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_done_dis(&mut self) -> Ch7DoneDisW<PcieDmaInterruptDisableSpec> {
        Ch7DoneDisW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 0 Error Disable Interrupt \\[ch0_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_error_dis(&mut self) -> Ch0ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch0ErrorDisW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Error Disable Interrupt \\[ch1_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_error_dis(&mut self) -> Ch1ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch1ErrorDisW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Error Disable Interrupt \\[ch2_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_error_dis(&mut self) -> Ch2ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch2ErrorDisW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 Error Disable Interrupt \\[ch3_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_error_dis(&mut self) -> Ch3ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch3ErrorDisW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Error Disable Interrupt \\[ch4_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_error_dis(&mut self) -> Ch4ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch4ErrorDisW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 5 Error Disable Interrupt \\[ch5_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_error_dis(&mut self) -> Ch5ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch5ErrorDisW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 6 Error Disable Interrupt \\[ch6_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_error_dis(&mut self) -> Ch6ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch6ErrorDisW::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 7 Error Disable Interrupt \\[ch7_error_dis\\]
Assert to 1 to disable error interrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_error_dis(&mut self) -> Ch7ErrorDisW<PcieDmaInterruptDisableSpec> {
        Ch7ErrorDisW::new(self, 15)
    }
}
#[doc = "PCIe DMA Interrupt Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt_disable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt_disable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaInterruptDisableSpec;
impl crate::RegisterSpec for PcieDmaInterruptDisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_interrupt_disable::R`](R) reader structure"]
impl crate::Readable for PcieDmaInterruptDisableSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_interrupt_disable::W`](W) writer structure"]
impl crate::Writable for PcieDmaInterruptDisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets PCIE_DMA_INTERRUPT_DISABLE to value 0xffff"]
impl crate::Resettable for PcieDmaInterruptDisableSpec {
    const RESET_VALUE: u32 = 0xffff;
}
