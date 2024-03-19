#[doc = "Register `CRU_CLKSEL_CON52` reader"]
pub type R = crate::R<CruClkselCon52Spec>;
#[doc = "Register `CRU_CLKSEL_CON52` writer"]
pub type W = crate::W<CruClkselCon52Spec>;
#[doc = "Field `CLK_VOP1_PWM_DIV_CON` reader - vop1_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkVop1PwmDivConR = crate::FieldReader;
#[doc = "Field `CLK_VOP1_PWM_DIV_CON` writer - vop1_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkVop1PwmDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "vop1_pwm_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkVop1PwmSrcSel {
    #[doc = "0: VPLL"]
    B00 = 0,
    #[doc = "1: CPLL"]
    B01 = 1,
    #[doc = "2: GPLL"]
    B1x = 2,
}
impl From<ClkVop1PwmSrcSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkVop1PwmSrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkVop1PwmSrcSel {
    type Ux = u8;
}
#[doc = "Field `CLK_VOP1_PWM_SRC_SEL` reader - vop1_pwm_src clock select control register"]
pub type ClkVop1PwmSrcSelR = crate::FieldReader<ClkVop1PwmSrcSel>;
impl ClkVop1PwmSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkVop1PwmSrcSel> {
        match self.bits {
            0 => Some(ClkVop1PwmSrcSel::B00),
            1 => Some(ClkVop1PwmSrcSel::B01),
            2 => Some(ClkVop1PwmSrcSel::B1x),
            _ => None,
        }
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkVop1PwmSrcSel::B00
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkVop1PwmSrcSel::B01
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkVop1PwmSrcSel::B1x
    }
}
#[doc = "Field `CLK_VOP1_PWM_SRC_SEL` writer - vop1_pwm_src clock select control register"]
pub type ClkVop1PwmSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkVop1PwmSrcSel>;
impl<'a, REG> ClkVop1PwmSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVop1PwmSrcSel::B00)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVop1PwmSrcSel::B01)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVop1PwmSrcSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - vop1_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_vop1_pwm_div_con(&self) -> ClkVop1PwmDivConR {
        ClkVop1PwmDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - vop1_pwm_src clock select control register"]
    #[inline(always)]
    pub fn clk_vop1_pwm_src_sel(&self) -> ClkVop1PwmSrcSelR {
        ClkVop1PwmSrcSelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - vop1_pwm divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vop1_pwm_div_con(&mut self) -> ClkVop1PwmDivConW<CruClkselCon52Spec> {
        ClkVop1PwmDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - vop1_pwm_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vop1_pwm_src_sel(&mut self) -> ClkVop1PwmSrcSelW<CruClkselCon52Spec> {
        ClkVop1PwmSrcSelW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon52Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon52Spec;
impl crate::RegisterSpec for CruClkselCon52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con52::R`](R) reader structure"]
impl crate::Readable for CruClkselCon52Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con52::W`](W) writer structure"]
impl crate::Writable for CruClkselCon52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON52 to value 0x05"]
impl crate::Resettable for CruClkselCon52Spec {
    const RESET_VALUE: u32 = 0x05;
}
