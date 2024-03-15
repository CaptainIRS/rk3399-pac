#[doc = "Register `PMU_PWRDN_CON1` reader"]
pub type R = crate::R<PmuPwrdnCon1Spec>;
#[doc = "Register `PMU_PWRDN_CON1` writer"]
pub type W = crate::W<PmuPwrdnCon1Spec>;
#[doc = "vd_scu_l power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VdScuLEnable {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<VdScuLEnable> for bool {
    #[inline(always)]
    fn from(variant: VdScuLEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VD_SCU_L_ENABLE` reader - vd_scu_l power down enable"]
pub type VdScuLEnableR = crate::BitReader<VdScuLEnable>;
impl VdScuLEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VdScuLEnable {
        match self.bits {
            false => VdScuLEnable::B0,
            true => VdScuLEnable::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VdScuLEnable::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VdScuLEnable::B1
    }
}
#[doc = "Field `VD_SCU_L_ENABLE` writer - vd_scu_l power down enable"]
pub type VdScuLEnableW<'a, REG> = crate::BitWriter<'a, REG, VdScuLEnable>;
impl<'a, REG> VdScuLEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VdScuLEnable::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VdScuLEnable::B1)
    }
}
#[doc = "vd_scu_b power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VdScuBPwrdwn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<VdScuBPwrdwn> for bool {
    #[inline(always)]
    fn from(variant: VdScuBPwrdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VD_SCU_B_PWRDWN` reader - vd_scu_b power down enable"]
pub type VdScuBPwrdwnR = crate::BitReader<VdScuBPwrdwn>;
impl VdScuBPwrdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VdScuBPwrdwn {
        match self.bits {
            false => VdScuBPwrdwn::B0,
            true => VdScuBPwrdwn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VdScuBPwrdwn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VdScuBPwrdwn::B1
    }
}
#[doc = "Field `VD_SCU_B_PWRDWN` writer - vd_scu_b power down enable"]
pub type VdScuBPwrdwnW<'a, REG> = crate::BitWriter<'a, REG, VdScuBPwrdwn>;
impl<'a, REG> VdScuBPwrdwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VdScuBPwrdwn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VdScuBPwrdwn::B1)
    }
}
#[doc = "vd_center power down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VdCenterPwrdwn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<VdCenterPwrdwn> for bool {
    #[inline(always)]
    fn from(variant: VdCenterPwrdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VD_CENTER_PWRDWN` reader - vd_center power down enable"]
pub type VdCenterPwrdwnR = crate::BitReader<VdCenterPwrdwn>;
impl VdCenterPwrdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VdCenterPwrdwn {
        match self.bits {
            false => VdCenterPwrdwn::B0,
            true => VdCenterPwrdwn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VdCenterPwrdwn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VdCenterPwrdwn::B1
    }
}
#[doc = "Field `VD_CENTER_PWRDWN` writer - vd_center power down enable"]
pub type VdCenterPwrdwnW<'a, REG> = crate::BitWriter<'a, REG, VdCenterPwrdwn>;
impl<'a, REG> VdCenterPwrdwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VdCenterPwrdwn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VdCenterPwrdwn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - vd_scu_l power down enable"]
    #[inline(always)]
    pub fn vd_scu_l_enable(&self) -> VdScuLEnableR {
        VdScuLEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - vd_scu_b power down enable"]
    #[inline(always)]
    pub fn vd_scu_b_pwrdwn(&self) -> VdScuBPwrdwnR {
        VdScuBPwrdwnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - vd_center power down enable"]
    #[inline(always)]
    pub fn vd_center_pwrdwn(&self) -> VdCenterPwrdwnR {
        VdCenterPwrdwnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - vd_scu_l power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn vd_scu_l_enable(&mut self) -> VdScuLEnableW<PmuPwrdnCon1Spec> {
        VdScuLEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - vd_scu_b power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn vd_scu_b_pwrdwn(&mut self) -> VdScuBPwrdwnW<PmuPwrdnCon1Spec> {
        VdScuBPwrdwnW::new(self, 1)
    }
    #[doc = "Bit 2 - vd_center power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn vd_center_pwrdwn(&mut self) -> VdCenterPwrdwnW<PmuPwrdnCon1Spec> {
        VdCenterPwrdwnW::new(self, 2)
    }
}
#[doc = "pmu power down configure register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuPwrdnCon1Spec;
impl crate::RegisterSpec for PmuPwrdnCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_pwrdn_con1::R`](R) reader structure"]
impl crate::Readable for PmuPwrdnCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmu_pwrdn_con1::W`](W) writer structure"]
impl crate::Writable for PmuPwrdnCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_PWRDN_CON1 to value 0"]
impl crate::Resettable for PmuPwrdnCon1Spec {
    const RESET_VALUE: u32 = 0;
}
