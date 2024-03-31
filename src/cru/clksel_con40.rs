#[doc = "Register `CLKSEL_CON40` reader"]
pub type R = crate::R<ClkselCon40Spec>;
#[doc = "Register `CLKSEL_CON40` writer"]
pub type W = crate::W<ClkselCon40Spec>;
#[doc = "Field `CLK_USB3_OTG0_SUSPEND_DIV_CON` reader - clk_usb3_otg0_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUsb3Otg0SuspendDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLK_USB3_OTG0_SUSPEND_DIV_CON` writer - clk_usb3_otg0_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUsb3Otg0SuspendDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "clk_usb3_otg0_suspend_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUsb3Otg0SuspendSrcSel {
    #[doc = "0: xin_24m"]
    B0 = 0,
    #[doc = "1: clk_32k"]
    B1 = 1,
}
impl From<ClkUsb3Otg0SuspendSrcSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUsb3Otg0SuspendSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_USB3_OTG0_SUSPEND_SRC_SEL` reader - clk_usb3_otg0_suspend_src clock select control register"]
pub type ClkUsb3Otg0SuspendSrcSelR = crate::BitReader<ClkUsb3Otg0SuspendSrcSel>;
impl ClkUsb3Otg0SuspendSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUsb3Otg0SuspendSrcSel {
        match self.bits {
            false => ClkUsb3Otg0SuspendSrcSel::B0,
            true => ClkUsb3Otg0SuspendSrcSel::B1,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUsb3Otg0SuspendSrcSel::B0
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUsb3Otg0SuspendSrcSel::B1
    }
}
#[doc = "Field `CLK_USB3_OTG0_SUSPEND_SRC_SEL` writer - clk_usb3_otg0_suspend_src clock select control register"]
pub type ClkUsb3Otg0SuspendSrcSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUsb3Otg0SuspendSrcSel>;
impl<'a, REG> ClkUsb3Otg0SuspendSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsb3Otg0SuspendSrcSel::B0)
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsb3Otg0SuspendSrcSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - clk_usb3_otg0_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_usb3_otg0_suspend_div_con(&self) -> ClkUsb3Otg0SuspendDivConR {
        ClkUsb3Otg0SuspendDivConR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - clk_usb3_otg0_suspend_src clock select control register"]
    #[inline(always)]
    pub fn clk_usb3_otg0_suspend_src_sel(&self) -> ClkUsb3Otg0SuspendSrcSelR {
        ClkUsb3Otg0SuspendSrcSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - clk_usb3_otg0_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg0_suspend_div_con(&mut self) -> ClkUsb3Otg0SuspendDivConW<ClkselCon40Spec> {
        ClkUsb3Otg0SuspendDivConW::new(self, 0)
    }
    #[doc = "Bit 15 - clk_usb3_otg0_suspend_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg0_suspend_src_sel(&mut self) -> ClkUsb3Otg0SuspendSrcSelW<ClkselCon40Spec> {
        ClkUsb3Otg0SuspendSrcSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon40Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon40Spec;
impl crate::RegisterSpec for ClkselCon40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con40::R`](R) reader structure"]
impl crate::Readable for ClkselCon40Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con40::W`](W) writer structure"]
impl crate::Writable for ClkselCon40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON40 to value 0"]
impl crate::Resettable for ClkselCon40Spec {
    const RESET_VALUE: u32 = 0;
}
