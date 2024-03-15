#[doc = "Register `ANALOG_CTL_49` reader"]
pub type R = crate::R<AnalogCtl49Spec>;
#[doc = "Register `ANALOG_CTL_49` writer"]
pub type W = crate::W<AnalogCtl49Spec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh2Pc2 {
    #[doc = "1: The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh2Pc2> for bool {
    #[inline(always)]
    fn from(variant: RForceCh2Pc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH2_PC2` reader - "]
pub type RForceCh2Pc2R = crate::BitReader<RForceCh2Pc2>;
impl RForceCh2Pc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh2Pc2 {
        match self.bits {
            true => RForceCh2Pc2::H1,
            false => RForceCh2Pc2::H0,
        }
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh2Pc2::H1
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh2Pc2::H0
    }
}
#[doc = "Field `R_FORCE_CH2_PC2` writer - "]
pub type RForceCh2Pc2W<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh2Pc2>;
impl<'a, REG> RForceCh2Pc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh2Pc2::H1)
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh2Pc2::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh2Emp {
    #[doc = "1: The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh2Emp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh2Emp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH2_EMP` reader - "]
pub type RForceCh2EmpR = crate::BitReader<RForceCh2Emp>;
impl RForceCh2EmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh2Emp {
        match self.bits {
            true => RForceCh2Emp::H1,
            false => RForceCh2Emp::H0,
        }
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh2Emp::H1
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh2Emp::H0
    }
}
#[doc = "Field `R_FORCE_CH2_EMP` writer - "]
pub type RForceCh2EmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh2Emp>;
impl<'a, REG> RForceCh2EmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh2Emp::H1)
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh2Emp::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh2Amp {
    #[doc = "1: The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh2Amp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh2Amp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH2_AMP` reader - "]
pub type RForceCh2AmpR = crate::BitReader<RForceCh2Amp>;
impl RForceCh2AmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh2Amp {
        match self.bits {
            true => RForceCh2Amp::H1,
            false => RForceCh2Amp::H0,
        }
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh2Amp::H1
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh2Amp::H0
    }
}
#[doc = "Field `R_FORCE_CH2_AMP` writer - "]
pub type RForceCh2AmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh2Amp>;
impl<'a, REG> RForceCh2AmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh2Amp::H1)
    }
    #[doc = "The result of ch2 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh2Amp::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh3Pc2 {
    #[doc = "1: The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh3Pc2> for bool {
    #[inline(always)]
    fn from(variant: RForceCh3Pc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH3_PC2` reader - "]
pub type RForceCh3Pc2R = crate::BitReader<RForceCh3Pc2>;
impl RForceCh3Pc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh3Pc2 {
        match self.bits {
            true => RForceCh3Pc2::H1,
            false => RForceCh3Pc2::H0,
        }
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh3Pc2::H1
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh3Pc2::H0
    }
}
#[doc = "Field `R_FORCE_CH3_PC2` writer - "]
pub type RForceCh3Pc2W<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh3Pc2>;
impl<'a, REG> RForceCh3Pc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh3Pc2::H1)
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh3Pc2::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh3Emp {
    #[doc = "1: The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh3Emp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh3Emp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH3_EMP` reader - "]
pub type RForceCh3EmpR = crate::BitReader<RForceCh3Emp>;
impl RForceCh3EmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh3Emp {
        match self.bits {
            true => RForceCh3Emp::H1,
            false => RForceCh3Emp::H0,
        }
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh3Emp::H1
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh3Emp::H0
    }
}
#[doc = "Field `R_FORCE_CH3_EMP` writer - "]
pub type RForceCh3EmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh3Emp>;
impl<'a, REG> RForceCh3EmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh3Emp::H1)
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh3Emp::H0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RForceCh3Amp {
    #[doc = "1: The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    H1 = 1,
    #[doc = "0: The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    H0 = 0,
}
impl From<RForceCh3Amp> for bool {
    #[inline(always)]
    fn from(variant: RForceCh3Amp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_FORCE_CH3_AMP` reader - "]
pub type RForceCh3AmpR = crate::BitReader<RForceCh3Amp>;
impl RForceCh3AmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RForceCh3Amp {
        match self.bits {
            true => RForceCh3Amp::H1,
            false => RForceCh3Amp::H0,
        }
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == RForceCh3Amp::H1
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == RForceCh3Amp::H0
    }
}
#[doc = "Field `R_FORCE_CH3_AMP` writer - "]
pub type RForceCh3AmpW<'a, REG> = crate::BitWriter1C<'a, REG, RForceCh3Amp>;
impl<'a, REG> RForceCh3AmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh3Amp::H1)
    }
    #[doc = "The result of ch3 swing bit is decide by different V_diff and Pre_emphasis"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(RForceCh3Amp::H0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_force_ch2_pc2(&self) -> RForceCh2Pc2R {
        RForceCh2Pc2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_force_ch2_emp(&self) -> RForceCh2EmpR {
        RForceCh2EmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_force_ch2_amp(&self) -> RForceCh2AmpR {
        RForceCh2AmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_force_ch3_pc2(&self) -> RForceCh3Pc2R {
        RForceCh3Pc2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_force_ch3_emp(&self) -> RForceCh3EmpR {
        RForceCh3EmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_force_ch3_amp(&self) -> RForceCh3AmpR {
        RForceCh3AmpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch2_pc2(&mut self) -> RForceCh2Pc2W<AnalogCtl49Spec> {
        RForceCh2Pc2W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch2_emp(&mut self) -> RForceCh2EmpW<AnalogCtl49Spec> {
        RForceCh2EmpW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch2_amp(&mut self) -> RForceCh2AmpW<AnalogCtl49Spec> {
        RForceCh2AmpW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch3_pc2(&mut self) -> RForceCh3Pc2W<AnalogCtl49Spec> {
        RForceCh3Pc2W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch3_emp(&mut self) -> RForceCh3EmpW<AnalogCtl49Spec> {
        RForceCh3EmpW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn r_force_ch3_amp(&mut self) -> RForceCh3AmpW<AnalogCtl49Spec> {
        RForceCh3AmpW::new(self, 5)
    }
}
#[doc = "CH2_CH3_FORCE_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl49Spec;
impl crate::RegisterSpec for AnalogCtl49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_49::R`](R) reader structure"]
impl crate::Readable for AnalogCtl49Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_49::W`](W) writer structure"]
impl crate::Writable for AnalogCtl49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ANALOG_CTL_49 to value 0"]
impl crate::Resettable for AnalogCtl49Spec {
    const RESET_VALUE: u32 = 0;
}
