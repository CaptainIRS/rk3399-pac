#[doc = "Register `PCIE_DMA_INTERRUPT_ENABLE` reader"]
pub type R = crate::R<PcieDmaInterruptEnableSpec>;
#[doc = "Register `PCIE_DMA_INTERRUPT_ENABLE` writer"]
pub type W = crate::W<PcieDmaInterruptEnableSpec>;
#[doc = "Field `ch0_done_ena` reader - Channel 0 Done Enable Interrupt \\[ch0_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch0DoneEnaR = crate::BitReader;
#[doc = "Field `ch0_done_ena` writer - Channel 0 Done Enable Interrupt \\[ch0_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch0DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch1_done_ena` reader - Channel 1 Done Enable Interrupt \\[ch1_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch1DoneEnaR = crate::BitReader;
#[doc = "Field `ch1_done_ena` writer - Channel 1 Done Enable Interrupt \\[ch1_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch1DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch2_done_ena` reader - Channel 2 Done Enable Interrupt \\[ch2_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch2DoneEnaR = crate::BitReader;
#[doc = "Field `ch2_done_ena` writer - Channel 2 Done Enable Interrupt \\[ch2_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch2DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch3_done_ena` reader - Channel 3 Done Enable Interrupt \\[ch3_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch3DoneEnaR = crate::BitReader;
#[doc = "Field `ch3_done_ena` writer - Channel 3 Done Enable Interrupt \\[ch3_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch3DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch4_done_ena` reader - Channel 4 Done Enable Interrupt \\[ch4_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch4DoneEnaR = crate::BitReader;
#[doc = "Field `ch4_done_ena` writer - Channel 4 Done Enable Interrupt \\[ch4_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch4DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch5_done_ena` reader - Channel 5 Done Enable Interrupt \\[ch5_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch5DoneEnaR = crate::BitReader;
#[doc = "Field `ch5_done_ena` writer - Channel 5 Done Enable Interrupt \\[ch5_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch5DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch6_done_ena` reader - Channel 6 Done Enable Interrupt \\[ch6_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch6DoneEnaR = crate::BitReader;
#[doc = "Field `ch6_done_ena` writer - Channel 6 Done Enable Interrupt \\[ch6_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch6DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch7_done_ena` reader - Channel 7 Done Enable Interrupt \\[ch7_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch7DoneEnaR = crate::BitReader;
#[doc = "Field `ch7_done_ena` writer - Channel 7 Done Enable Interrupt \\[ch7_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
pub type Ch7DoneEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch0_error_ena` reader - Channel 0 Error Enable Interrupt \\[ch0_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch0ErrorEnaR = crate::BitReader;
#[doc = "Field `ch0_error_ena` writer - Channel 0 Error Enable Interrupt \\[ch0_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch0ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch1_error_ena` reader - Channel 1 Error Enable Interrupt \\[ch1_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch1ErrorEnaR = crate::BitReader;
#[doc = "Field `ch1_error_ena` writer - Channel 1 Error Enable Interrupt \\[ch1_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch1ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch2_error_ena` reader - Channel 2 Error Enable Interrupt \\[ch2_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch2ErrorEnaR = crate::BitReader;
#[doc = "Field `ch2_error_ena` writer - Channel 2 Error Enable Interrupt \\[ch2_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch2ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch3_error_ena` reader - Channel 3 Error Enable Interrupt \\[ch3_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch3ErrorEnaR = crate::BitReader;
#[doc = "Field `ch3_error_ena` writer - Channel 3 Error Enable Interrupt \\[ch3_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch3ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch4_error_ena` reader - Channel 4 Error Enable Interrupt \\[ch4_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch4ErrorEnaR = crate::BitReader;
#[doc = "Field `ch4_error_ena` writer - Channel 4 Error Enable Interrupt \\[ch4_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch4ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch5_error_ena` reader - Channel 5 Error Enable Interrupt \\[ch5_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch5ErrorEnaR = crate::BitReader;
#[doc = "Field `ch5_error_ena` writer - Channel 5 Error Enable Interrupt \\[ch5_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch5ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch6_error_ena` reader - Channel 6 Error Enable Interrupt \\[ch6_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch6ErrorEnaR = crate::BitReader;
#[doc = "Field `ch6_error_ena` writer - Channel 6 Error Enable Interrupt \\[ch6_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch6ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ch7_error_ena` reader - Channel 7 Error Enable Interrupt \\[ch7_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch7ErrorEnaR = crate::BitReader;
#[doc = "Field `ch7_error_ena` writer - Channel 7 Error Enable Interrupt \\[ch7_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
pub type Ch7ErrorEnaW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Done Enable Interrupt \\[ch0_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch0_done_ena(&self) -> Ch0DoneEnaR {
        Ch0DoneEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Done Enable Interrupt \\[ch1_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch1_done_ena(&self) -> Ch1DoneEnaR {
        Ch1DoneEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Done Enable Interrupt \\[ch2_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch2_done_ena(&self) -> Ch2DoneEnaR {
        Ch2DoneEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Done Enable Interrupt \\[ch3_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch3_done_ena(&self) -> Ch3DoneEnaR {
        Ch3DoneEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Done Enable Interrupt \\[ch4_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch4_done_ena(&self) -> Ch4DoneEnaR {
        Ch4DoneEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Done Enable Interrupt \\[ch5_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch5_done_ena(&self) -> Ch5DoneEnaR {
        Ch5DoneEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Done Enable Interrupt \\[ch6_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch6_done_ena(&self) -> Ch6DoneEnaR {
        Ch6DoneEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Done Enable Interrupt \\[ch7_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch7_done_ena(&self) -> Ch7DoneEnaR {
        Ch7DoneEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Error Enable Interrupt \\[ch0_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch0_error_ena(&self) -> Ch0ErrorEnaR {
        Ch0ErrorEnaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Error Enable Interrupt \\[ch1_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch1_error_ena(&self) -> Ch1ErrorEnaR {
        Ch1ErrorEnaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Error Enable Interrupt \\[ch2_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch2_error_ena(&self) -> Ch2ErrorEnaR {
        Ch2ErrorEnaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Error Enable Interrupt \\[ch3_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch3_error_ena(&self) -> Ch3ErrorEnaR {
        Ch3ErrorEnaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Error Enable Interrupt \\[ch4_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch4_error_ena(&self) -> Ch4ErrorEnaR {
        Ch4ErrorEnaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Error Enable Interrupt \\[ch5_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch5_error_ena(&self) -> Ch5ErrorEnaR {
        Ch5ErrorEnaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Error Enable Interrupt \\[ch6_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch6_error_ena(&self) -> Ch6ErrorEnaR {
        Ch6ErrorEnaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Error Enable Interrupt \\[ch7_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    pub fn ch7_error_ena(&self) -> Ch7ErrorEnaR {
        Ch7ErrorEnaR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Done Enable Interrupt \\[ch0_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_done_ena(&mut self) -> Ch0DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch0DoneEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Done Enable Interrupt \\[ch1_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_done_ena(&mut self) -> Ch1DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch1DoneEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Done Enable Interrupt \\[ch2_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_done_ena(&mut self) -> Ch2DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch2DoneEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Done Enable Interrupt \\[ch3_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_done_ena(&mut self) -> Ch3DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch3DoneEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Done Enable Interrupt \\[ch4_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_done_ena(&mut self) -> Ch4DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch4DoneEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Done Enable Interrupt \\[ch5_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_done_ena(&mut self) -> Ch5DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch5DoneEnaW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Done Enable Interrupt \\[ch6_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_done_ena(&mut self) -> Ch6DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch6DoneEnaW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Done Enable Interrupt \\[ch7_done_ena\\]\n\nAssert to 1 to enable done\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_done_ena(&mut self) -> Ch7DoneEnaW<PcieDmaInterruptEnableSpec> {
        Ch7DoneEnaW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 0 Error Enable Interrupt \\[ch0_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_error_ena(&mut self) -> Ch0ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch0ErrorEnaW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Error Enable Interrupt \\[ch1_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_error_ena(&mut self) -> Ch1ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch1ErrorEnaW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Error Enable Interrupt \\[ch2_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_error_ena(&mut self) -> Ch2ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch2ErrorEnaW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 Error Enable Interrupt \\[ch3_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_error_ena(&mut self) -> Ch3ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch3ErrorEnaW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Error Enable Interrupt \\[ch4_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_error_ena(&mut self) -> Ch4ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch4ErrorEnaW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 5 Error Enable Interrupt \\[ch5_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_error_ena(&mut self) -> Ch5ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch5ErrorEnaW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 6 Error Enable Interrupt \\[ch6_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_error_ena(&mut self) -> Ch6ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch6ErrorEnaW::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 7 Error Enable Interrupt \\[ch7_error_ena\\]\n\nAssert to 1 to enable error\n\ninterrupts to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_error_ena(&mut self) -> Ch7ErrorEnaW<PcieDmaInterruptEnableSpec> {
        Ch7ErrorEnaW::new(self, 15)
    }
}
#[doc = "PCIe DMA Interrupt Enable Register\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaInterruptEnableSpec;
impl crate::RegisterSpec for PcieDmaInterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_interrupt_enable::R`](R) reader structure"]
impl crate::Readable for PcieDmaInterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_interrupt_enable::W`](W) writer structure"]
impl crate::Writable for PcieDmaInterruptEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets PCIE_DMA_INTERRUPT_ENABLE to value 0xffff"]
impl crate::Resettable for PcieDmaInterruptEnableSpec {
    const RESET_VALUE: u32 = 0xffff;
}
