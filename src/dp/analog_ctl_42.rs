#[doc = "Register `ANALOG_CTL_42` reader"]
pub type R = crate::R<AnalogCtl42Spec>;
#[doc = "Register `ANALOG_CTL_42` writer"]
pub type W = crate::W<AnalogCtl42Spec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh0Pc2 {
    #[doc = "1: The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh0Pc2> for bool {
    #[inline(always)]
    fn from(variant: RForceCh0Pc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH0_PC2` reader - "]
pub type RForceCh0Pc2R = crate::BitReader<RForceCh0Pc2>;
impl RForceCh0Pc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh0Pc2 {
        match self.bits {
            true => RForceCh0Pc2::H1,
            false => RForceCh0Pc2::H0,
        }
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh0Pc2::H1
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh0Pc2::H0
    }
}
#[doc = "Field `R_FORCE_CH0_PC2` writer - "]
pub type RForceCh0Pc2W<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh0Pc2>;
impl<'a, REG> RForceCh0Pc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh0Pc2::H1)
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh0Pc2::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh0Emp {
    #[doc = "1: The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh0Emp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh0Emp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH0_EMP` reader - "]
pub type RForceCh0EmpR = crate::BitReader<RForceCh0Emp>;
impl RForceCh0EmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh0Emp {
        match self.bits {
            true => RForceCh0Emp::H1,
            false => RForceCh0Emp::H0,
        }
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh0Emp::H1
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh0Emp::H0
    }
}
#[doc = "Field `R_FORCE_CH0_EMP` writer - "]
pub type RForceCh0EmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh0Emp>;
impl<'a, REG> RForceCh0EmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh0Emp::H1)
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh0Emp::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh0Amp {
    #[doc = "1: The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh0Amp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh0Amp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH0_AMP` reader - "]
pub type RForceCh0AmpR = crate::BitReader<RForceCh0Amp>;
impl RForceCh0AmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh0Amp {
        match self.bits {
            true => RForceCh0Amp::H1,
            false => RForceCh0Amp::H0,
        }
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh0Amp::H1
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh0Amp::H0
    }
}
#[doc = "Field `R_FORCE_CH0_AMP` writer - "]
pub type RForceCh0AmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh0Amp>;
impl<'a, REG> RForceCh0AmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh0Amp::H1)
    }
    #[doc = "The result of ch0 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh0Amp::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh1Pc2 {
    #[doc = "1: The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh1Pc2> for bool {
    #[inline(always)]
    fn from(variant: RForceCh1Pc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH1_PC2` reader - "]
pub type RForceCh1Pc2R = crate::BitReader<RForceCh1Pc2>;
impl RForceCh1Pc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh1Pc2 {
        match self.bits {
            true => RForceCh1Pc2::H1,
            false => RForceCh1Pc2::H0,
        }
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh1Pc2::H1
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh1Pc2::H0
    }
}
#[doc = "Field `R_FORCE_CH1_PC2` writer - "]
pub type RForceCh1Pc2W<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh1Pc2>;
impl<'a, REG> RForceCh1Pc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh1Pc2::H1)
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh1Pc2::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh1Emp {
    #[doc = "1: The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh1Emp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh1Emp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH1_EMP` reader - "]
pub type RForceCh1EmpR = crate::BitReader<RForceCh1Emp>;
impl RForceCh1EmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh1Emp {
        match self.bits {
            true => RForceCh1Emp::H1,
            false => RForceCh1Emp::H0,
        }
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh1Emp::H1
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh1Emp::H0
    }
}
#[doc = "Field `R_FORCE_CH1_EMP` writer - "]
pub type RForceCh1EmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh1Emp>;
impl<'a, REG> RForceCh1EmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh1Emp::H1)
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh1Emp::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh1Amp {
    #[doc = "1: The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh1Amp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh1Amp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH1_AMP` reader - "]
pub type RForceCh1AmpR = crate::BitReader<RForceCh1Amp>;
impl RForceCh1AmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh1Amp {
        match self.bits {
            true => RForceCh1Amp::H1,
            false => RForceCh1Amp::H0,
        }
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh1Amp::H1
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh1Amp::H0
    }
}
#[doc = "Field `R_FORCE_CH1_AMP` writer - "]
pub type RForceCh1AmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh1Amp>;
impl<'a, REG> RForceCh1AmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh1Amp::H1)
    }
    #[doc = "The result of ch1 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh1Amp::H0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_force_ch0_pc2(&self) -> RForceCh0Pc2R {
        RForceCh0Pc2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_force_ch0_emp(&self) -> RForceCh0EmpR {
        RForceCh0EmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_force_ch0_amp(&self) -> RForceCh0AmpR {
        RForceCh0AmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_force_ch1_pc2(&self) -> RForceCh1Pc2R {
        RForceCh1Pc2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_force_ch1_emp(&self) -> RForceCh1EmpR {
        RForceCh1EmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_force_ch1_amp(&self) -> RForceCh1AmpR {
        RForceCh1AmpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch0_pc2(&mut self) -> RForceCh0Pc2W<AnalogCtl42Spec> {
        RForceCh0Pc2W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch0_emp(&mut self) -> RForceCh0EmpW<AnalogCtl42Spec> {
        RForceCh0EmpW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch0_amp(&mut self) -> RForceCh0AmpW<AnalogCtl42Spec> {
        RForceCh0AmpW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch1_pc2(&mut self) -> RForceCh1Pc2W<AnalogCtl42Spec> {
        RForceCh1Pc2W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch1_emp(&mut self) -> RForceCh1EmpW<AnalogCtl42Spec> {
        RForceCh1EmpW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch1_amp(&mut self) -> RForceCh1AmpW<AnalogCtl42Spec> {
        RForceCh1AmpW::new(self, 5)
    }
}
#[doc = "CH0_CH1_FORCE_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl42Spec;
impl crate::RegisterSpec for AnalogCtl42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_42::R`](R) reader structure"]
impl crate::Readable for AnalogCtl42Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_42::W`](W) writer structure"]
impl crate::Writable for AnalogCtl42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_42 to value 0"]
impl crate::Resettable for AnalogCtl42Spec {
    const RESET_VALUE: u32 = 0;
}
