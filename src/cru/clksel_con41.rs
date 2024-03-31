#[doc = "Register `CLKSEL_CON41` reader"]
pub type R = crate::R<ClkselCon41Spec>;
#[doc = "Register `CLKSEL_CON41` writer"]
pub type W = crate::W<ClkselCon41Spec>;
#[doc = "Field `CLK_USB3_OTG1_SUSPEND_DIV_CON` reader - clk_usb3_otg1_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUsb3Otg1SuspendDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLK_USB3_OTG1_SUSPEND_DIV_CON` writer - clk_usb3_otg1_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUsb3Otg1SuspendDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "clk_usb3_otg1_suspend_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUsb3Otg1SuspendSrcSel {
    #[doc = "0: xin_24m"]
    B0 = 0,
    #[doc = "1: clk_32k"]
    B1 = 1,
}
impl From<ClkUsb3Otg1SuspendSrcSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUsb3Otg1SuspendSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_USB3_OTG1_SUSPEND_SRC_SEL` reader - clk_usb3_otg1_suspend_src clock select control register"]
pub type ClkUsb3Otg1SuspendSrcSelR = crate::BitReader<ClkUsb3Otg1SuspendSrcSel>;
impl ClkUsb3Otg1SuspendSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUsb3Otg1SuspendSrcSel {
        match self.bits {
            false => ClkUsb3Otg1SuspendSrcSel::B0,
            true => ClkUsb3Otg1SuspendSrcSel::B1,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUsb3Otg1SuspendSrcSel::B0
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUsb3Otg1SuspendSrcSel::B1
    }
}
#[doc = "Field `CLK_USB3_OTG1_SUSPEND_SRC_SEL` writer - clk_usb3_otg1_suspend_src clock select control register"]
pub type ClkUsb3Otg1SuspendSrcSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUsb3Otg1SuspendSrcSel>;
impl<'a, REG> ClkUsb3Otg1SuspendSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsb3Otg1SuspendSrcSel::B0)
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsb3Otg1SuspendSrcSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - clk_usb3_otg1_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_usb3_otg1_suspend_div_con(&self) -> ClkUsb3Otg1SuspendDivConR {
        ClkUsb3Otg1SuspendDivConR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - clk_usb3_otg1_suspend_src clock select control register"]
    #[inline(always)]
    pub fn clk_usb3_otg1_suspend_src_sel(&self) -> ClkUsb3Otg1SuspendSrcSelR {
        ClkUsb3Otg1SuspendSrcSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - clk_usb3_otg1_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg1_suspend_div_con(&mut self) -> ClkUsb3Otg1SuspendDivConW<ClkselCon41Spec> {
        ClkUsb3Otg1SuspendDivConW::new(self, 0)
    }
    #[doc = "Bit 15 - clk_usb3_otg1_suspend_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg1_suspend_src_sel(&mut self) -> ClkUsb3Otg1SuspendSrcSelW<ClkselCon41Spec> {
        ClkUsb3Otg1SuspendSrcSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon41Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon41Spec;
impl crate::RegisterSpec for ClkselCon41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con41::R`](R) reader structure"]
impl crate::Readable for ClkselCon41Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con41::W`](W) writer structure"]
impl crate::Writable for ClkselCon41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON41 to value 0"]
impl crate::Resettable for ClkselCon41Spec {
    const RESET_VALUE: u32 = 0;
}
