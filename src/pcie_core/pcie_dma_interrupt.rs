#[doc = "Register `PCIE_DMA_INTERRUPT` reader"]
pub type R = crate::R<PcieDmaInterruptSpec>;
#[doc = "Register `PCIE_DMA_INTERRUPT` writer"]
pub type W = crate::W<PcieDmaInterruptSpec>;
#[doc = "Field `ch0_done_int` reader - Channel 0 Done Interrupt \\[ch0_done_int\\]\n\nChannel 0 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch0DoneIntR = crate::BitReader;
#[doc = "Field `ch0_done_int` writer - Channel 0 Done Interrupt \\[ch0_done_int\\]\n\nChannel 0 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch0DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch1_done_int` reader - Channel 1 Done Interrupt \\[ch1_done_int\\]\n\nChannel 1 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch1DoneIntR = crate::BitReader;
#[doc = "Field `ch1_done_int` writer - Channel 1 Done Interrupt \\[ch1_done_int\\]\n\nChannel 1 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch1DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch2_done_int` reader - Channel 2 Done Interrupt \\[ch2_done_int\\]\n\nChannel 2 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch2DoneIntR = crate::BitReader;
#[doc = "Field `ch2_done_int` writer - Channel 2 Done Interrupt \\[ch2_done_int\\]\n\nChannel 2 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch2DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch3_done_int` reader - Channel 3 Done Interrupt \\[ch3_done_int\\]\n\nChannel 3 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch3DoneIntR = crate::BitReader;
#[doc = "Field `ch3_done_int` writer - Channel 3 Done Interrupt \\[ch3_done_int\\]\n\nChannel 3 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch3DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch4_done_int` reader - Channel 4 Done Interrupt \\[ch4_done_int\\]\n\nChannel 4 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch4DoneIntR = crate::BitReader;
#[doc = "Field `ch4_done_int` writer - Channel 4 Done Interrupt \\[ch4_done_int\\]\n\nChannel 4 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch4DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch5_done_int` reader - Channel 5 Done Interrupt \\[ch5_done_int\\]\n\nChannel 5 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch5DoneIntR = crate::BitReader;
#[doc = "Field `ch5_done_int` writer - Channel 5 Done Interrupt \\[ch5_done_int\\]\n\nChannel 5 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch5DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch6_done_int` reader - Channel 6 Done Interrupt \\[ch6_done_int\\]\n\nChannel 6 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch6DoneIntR = crate::BitReader;
#[doc = "Field `ch6_done_int` writer - Channel 6 Done Interrupt \\[ch6_done_int\\]\n\nChannel 6 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch6DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch7_done_int` reader - Channel 7 Done Interrupt \\[ch7_done_int\\]\n\nChannel 7 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch7DoneIntR = crate::BitReader;
#[doc = "Field `ch7_done_int` writer - Channel 7 Done Interrupt \\[ch7_done_int\\]\n\nChannel 7 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch7DoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch0_error_int` reader - Channel 0 Error Interrupt \\[ch0_error_int\\]\n\nChannel 0 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch0ErrorIntR = crate::BitReader;
#[doc = "Field `ch0_error_int` writer - Channel 0 Error Interrupt \\[ch0_error_int\\]\n\nChannel 0 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch0ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch1_error_int` reader - Channel 1 Error Interrupt \\[ch1_error_int\\]\n\nChannel 1 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch1ErrorIntR = crate::BitReader;
#[doc = "Field `ch1_error_int` writer - Channel 1 Error Interrupt \\[ch1_error_int\\]\n\nChannel 1 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch1ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch2_error_int` reader - Channel 2 Error Interrupt \\[ch2_error_int\\]\n\nChannel 2 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch2ErrorIntR = crate::BitReader;
#[doc = "Field `ch2_error_int` writer - Channel 2 Error Interrupt \\[ch2_error_int\\]\n\nChannel 2 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch2ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch3_error_int` reader - Channel 3 Error Interrupt \\[ch3_error_int\\]\n\nChannel 3 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch3ErrorIntR = crate::BitReader;
#[doc = "Field `ch3_error_int` writer - Channel 3 Error Interrupt \\[ch3_error_int\\]\n\nChannel 3 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch3ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch4_error_int` reader - Channel 4 Error Interrupt \\[ch4_error_int\\]\n\nChannel 4 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch4ErrorIntR = crate::BitReader;
#[doc = "Field `ch4_error_int` writer - Channel 4 Error Interrupt \\[ch4_error_int\\]\n\nChannel 4 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch4ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch5_error_int` reader - Channel 5 Error Interrupt \\[ch5_error_int\\]\n\nChannel 5 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch5ErrorIntR = crate::BitReader;
#[doc = "Field `ch5_error_int` writer - Channel 5 Error Interrupt \\[ch5_error_int\\]\n\nChannel 5 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch5ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch6_error_int` reader - Channel 6 Error Interrupt \\[ch6_error_int\\]\n\nChannel 6 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch6ErrorIntR = crate::BitReader;
#[doc = "Field `ch6_error_int` writer - Channel 6 Error Interrupt \\[ch6_error_int\\]\n\nChannel 6 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch6ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch7_error_int` reader - Channel 7 Error Interrupt \\[ch7_error_int\\]\n\nChannel 7 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch7ErrorIntR = crate::BitReader;
#[doc = "Field `ch7_error_int` writer - Channel 7 Error Interrupt \\[ch7_error_int\\]\n\nChannel 7 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
pub type Ch7ErrorIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Done Interrupt \\[ch0_done_int\\]\n\nChannel 0 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch0_done_int(&self) -> Ch0DoneIntR {
        Ch0DoneIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Done Interrupt \\[ch1_done_int\\]\n\nChannel 1 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch1_done_int(&self) -> Ch1DoneIntR {
        Ch1DoneIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Done Interrupt \\[ch2_done_int\\]\n\nChannel 2 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch2_done_int(&self) -> Ch2DoneIntR {
        Ch2DoneIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Done Interrupt \\[ch3_done_int\\]\n\nChannel 3 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch3_done_int(&self) -> Ch3DoneIntR {
        Ch3DoneIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Done Interrupt \\[ch4_done_int\\]\n\nChannel 4 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch4_done_int(&self) -> Ch4DoneIntR {
        Ch4DoneIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Done Interrupt \\[ch5_done_int\\]\n\nChannel 5 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch5_done_int(&self) -> Ch5DoneIntR {
        Ch5DoneIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Done Interrupt \\[ch6_done_int\\]\n\nChannel 6 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch6_done_int(&self) -> Ch6DoneIntR {
        Ch6DoneIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Done Interrupt \\[ch7_done_int\\]\n\nChannel 7 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch7_done_int(&self) -> Ch7DoneIntR {
        Ch7DoneIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Error Interrupt \\[ch0_error_int\\]\n\nChannel 0 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch0_error_int(&self) -> Ch0ErrorIntR {
        Ch0ErrorIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Error Interrupt \\[ch1_error_int\\]\n\nChannel 1 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch1_error_int(&self) -> Ch1ErrorIntR {
        Ch1ErrorIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Error Interrupt \\[ch2_error_int\\]\n\nChannel 2 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch2_error_int(&self) -> Ch2ErrorIntR {
        Ch2ErrorIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Error Interrupt \\[ch3_error_int\\]\n\nChannel 3 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch3_error_int(&self) -> Ch3ErrorIntR {
        Ch3ErrorIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Error Interrupt \\[ch4_error_int\\]\n\nChannel 4 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch4_error_int(&self) -> Ch4ErrorIntR {
        Ch4ErrorIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Error Interrupt \\[ch5_error_int\\]\n\nChannel 5 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch5_error_int(&self) -> Ch5ErrorIntR {
        Ch5ErrorIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Error Interrupt \\[ch6_error_int\\]\n\nChannel 6 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch6_error_int(&self) -> Ch6ErrorIntR {
        Ch6ErrorIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Error Interrupt \\[ch7_error_int\\]\n\nChannel 7 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    pub fn ch7_error_int(&self) -> Ch7ErrorIntR {
        Ch7ErrorIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Done Interrupt \\[ch0_done_int\\]\n\nChannel 0 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_done_int(&mut self) -> Ch0DoneIntW<PcieDmaInterruptSpec> {
        Ch0DoneIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Done Interrupt \\[ch1_done_int\\]\n\nChannel 1 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_done_int(&mut self) -> Ch1DoneIntW<PcieDmaInterruptSpec> {
        Ch1DoneIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Done Interrupt \\[ch2_done_int\\]\n\nChannel 2 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_done_int(&mut self) -> Ch2DoneIntW<PcieDmaInterruptSpec> {
        Ch2DoneIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Done Interrupt \\[ch3_done_int\\]\n\nChannel 3 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_done_int(&mut self) -> Ch3DoneIntW<PcieDmaInterruptSpec> {
        Ch3DoneIntW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Done Interrupt \\[ch4_done_int\\]\n\nChannel 4 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_done_int(&mut self) -> Ch4DoneIntW<PcieDmaInterruptSpec> {
        Ch4DoneIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Done Interrupt \\[ch5_done_int\\]\n\nChannel 5 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_done_int(&mut self) -> Ch5DoneIntW<PcieDmaInterruptSpec> {
        Ch5DoneIntW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Done Interrupt \\[ch6_done_int\\]\n\nChannel 6 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_done_int(&mut self) -> Ch6DoneIntW<PcieDmaInterruptSpec> {
        Ch6DoneIntW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Done Interrupt \\[ch7_done_int\\]\n\nChannel 7 Done Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_done_int(&mut self) -> Ch7DoneIntW<PcieDmaInterruptSpec> {
        Ch7DoneIntW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 0 Error Interrupt \\[ch0_error_int\\]\n\nChannel 0 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_error_int(&mut self) -> Ch0ErrorIntW<PcieDmaInterruptSpec> {
        Ch0ErrorIntW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Error Interrupt \\[ch1_error_int\\]\n\nChannel 1 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_error_int(&mut self) -> Ch1ErrorIntW<PcieDmaInterruptSpec> {
        Ch1ErrorIntW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Error Interrupt \\[ch2_error_int\\]\n\nChannel 2 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_error_int(&mut self) -> Ch2ErrorIntW<PcieDmaInterruptSpec> {
        Ch2ErrorIntW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 Error Interrupt \\[ch3_error_int\\]\n\nChannel 3 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_error_int(&mut self) -> Ch3ErrorIntW<PcieDmaInterruptSpec> {
        Ch3ErrorIntW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Error Interrupt \\[ch4_error_int\\]\n\nChannel 4 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_error_int(&mut self) -> Ch4ErrorIntW<PcieDmaInterruptSpec> {
        Ch4ErrorIntW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 5 Error Interrupt \\[ch5_error_int\\]\n\nChannel 5 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_error_int(&mut self) -> Ch5ErrorIntW<PcieDmaInterruptSpec> {
        Ch5ErrorIntW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 6 Error Interrupt \\[ch6_error_int\\]\n\nChannel 6 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_error_int(&mut self) -> Ch6ErrorIntW<PcieDmaInterruptSpec> {
        Ch6ErrorIntW::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 7 Error Interrupt \\[ch7_error_int\\]\n\nChannel 7 Error Interrupt\n\nRegisterInterrupt, Sticky (individual\n\nbits)"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_error_int(&mut self) -> Ch7ErrorIntW<PcieDmaInterruptSpec> {
        Ch7ErrorIntW::new(self, 15)
    }
}
#[doc = "PCIe DMA Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaInterruptSpec;
impl crate::RegisterSpec for PcieDmaInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_interrupt::R`](R) reader structure"]
impl crate::Readable for PcieDmaInterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_interrupt::W`](W) writer structure"]
impl crate::Writable for PcieDmaInterruptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets PCIE_DMA_INTERRUPT to value 0"]
impl crate::Resettable for PcieDmaInterruptSpec {
    const RESET_VALUE: u32 = 0;
}
