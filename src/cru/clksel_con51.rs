#[doc = "Register `CLKSEL_CON51` reader"]
pub type R = crate::R<ClkselCon51Spec>;
#[doc = "Register `CLKSEL_CON51` writer"]
pub type W = crate::W<ClkselCon51Spec>;
#[doc = "Field `CLK_VOP0_PWM_DIV_CON` reader - vop0_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkVop0PwmDivConR = crate::FieldReader;
#[doc = "Field `CLK_VOP0_PWM_DIV_CON` writer - vop0_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkVop0PwmDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "vop0_pwm_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkVop0PwmSrcSel {
    #[doc = "0: VPLL"]
    B00 = 0,
    #[doc = "1: CPLL"]
    B01 = 1,
    #[doc = "2: GPLL"]
    B1x = 2,
}
impl From<ClkVop0PwmSrcSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkVop0PwmSrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkVop0PwmSrcSel {
    type Ux = u8;
}
#[doc = "Field `CLK_VOP0_PWM_SRC_SEL` reader - vop0_pwm_src clock select control register"]
pub type ClkVop0PwmSrcSelR = crate::FieldReader<ClkVop0PwmSrcSel>;
impl ClkVop0PwmSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkVop0PwmSrcSel> {
        match self.bits {
            0 => Some(ClkVop0PwmSrcSel::B00),
            1 => Some(ClkVop0PwmSrcSel::B01),
            2 => Some(ClkVop0PwmSrcSel::B1x),
            _ => None,
        }
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkVop0PwmSrcSel::B00
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkVop0PwmSrcSel::B01
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkVop0PwmSrcSel::B1x
    }
}
#[doc = "Field `CLK_VOP0_PWM_SRC_SEL` writer - vop0_pwm_src clock select control register"]
pub type ClkVop0PwmSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkVop0PwmSrcSel>;
impl<'a, REG> ClkVop0PwmSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVop0PwmSrcSel::B00)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVop0PwmSrcSel::B01)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVop0PwmSrcSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - vop0_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_vop0_pwm_div_con(&self) -> ClkVop0PwmDivConR {
        ClkVop0PwmDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - vop0_pwm_src clock select control register"]
    #[inline(always)]
    pub fn clk_vop0_pwm_src_sel(&self) -> ClkVop0PwmSrcSelR {
        ClkVop0PwmSrcSelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - vop0_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vop0_pwm_div_con(&mut self) -> ClkVop0PwmDivConW<ClkselCon51Spec> {
        ClkVop0PwmDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - vop0_pwm_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vop0_pwm_src_sel(&mut self) -> ClkVop0PwmSrcSelW<ClkselCon51Spec> {
        ClkVop0PwmSrcSelW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon51Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon51Spec;
impl crate::RegisterSpec for ClkselCon51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con51::R`](R) reader structure"]
impl crate::Readable for ClkselCon51Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con51::W`](W) writer structure"]
impl crate::Writable for ClkselCon51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON51 to value 0x05"]
impl crate::Resettable for ClkselCon51Spec {
    const RESET_VALUE: u32 = 0x05;
}
