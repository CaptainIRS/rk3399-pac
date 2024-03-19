#[doc = "Register `PMU_CPU2APM_CON` reader"]
pub type R = crate::R<PmuCpu2apmConSpec>;
#[doc = "Register `PMU_CPU2APM_CON` writer"]
pub type W = crate::W<PmuCpu2apmConSpec>;
#[doc = "cpu_l2 wfi power down enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL2WfiPwrdnEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL2WfiPwrdnEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL2WfiPwrdnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L2_WFI_PWRDN_EN` reader - cpu_l2 wfi power down enable."]
pub type CpuL2WfiPwrdnEnR = crate::BitReader<CpuL2WfiPwrdnEn>;
impl CpuL2WfiPwrdnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL2WfiPwrdnEn {
        match self.bits {
            false => CpuL2WfiPwrdnEn::B0,
            true => CpuL2WfiPwrdnEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL2WfiPwrdnEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL2WfiPwrdnEn::B1
    }
}
#[doc = "Field `CPU_L2_WFI_PWRDN_EN` writer - cpu_l2 wfi power down enable."]
pub type CpuL2WfiPwrdnEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL2WfiPwrdnEn>;
impl<'a, REG> CpuL2WfiPwrdnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL2WfiPwrdnEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL2WfiPwrdnEn::B1)
    }
}
#[doc = "cpu l2 interrupt wake enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL2IntWakeupEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL2IntWakeupEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL2IntWakeupEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L2_INT_WAKEUP_EN` reader - cpu l2 interrupt wake enable."]
pub type CpuL2IntWakeupEnR = crate::BitReader<CpuL2IntWakeupEn>;
impl CpuL2IntWakeupEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL2IntWakeupEn {
        match self.bits {
            false => CpuL2IntWakeupEn::B0,
            true => CpuL2IntWakeupEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL2IntWakeupEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL2IntWakeupEn::B1
    }
}
#[doc = "Field `CPU_L2_INT_WAKEUP_EN` writer - cpu l2 interrupt wake enable."]
pub type CpuL2IntWakeupEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL2IntWakeupEn>;
impl<'a, REG> CpuL2IntWakeupEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL2IntWakeupEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL2IntWakeupEn::B1)
    }
}
#[doc = "cpu l2 software wakeup source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL2SftWakeup {
    #[doc = "1: wakeup ;"]
    B1 = 1,
    #[doc = "0: nothing ;"]
    B0 = 0,
}
impl From<CpuL2SftWakeup> for bool {
    #[inline(always)]
    fn from(variant: CpuL2SftWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L2_SFT_WAKEUP` reader - cpu l2 software wakeup source."]
pub type CpuL2SftWakeupR = crate::BitReader<CpuL2SftWakeup>;
impl CpuL2SftWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL2SftWakeup {
        match self.bits {
            true => CpuL2SftWakeup::B1,
            false => CpuL2SftWakeup::B0,
        }
    }
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL2SftWakeup::B1
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL2SftWakeup::B0
    }
}
#[doc = "Field `CPU_L2_SFT_WAKEUP` writer - cpu l2 software wakeup source."]
pub type CpuL2SftWakeupW<'a, REG> = crate::BitWriter<'a, REG, CpuL2SftWakeup>;
impl<'a, REG> CpuL2SftWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL2SftWakeup::B1)
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL2SftWakeup::B0)
    }
}
impl R {
    #[doc = "Bit 0 - cpu_l2 wfi power down enable."]
    #[inline(always)]
    pub fn cpu_l2_wfi_pwrdn_en(&self) -> CpuL2WfiPwrdnEnR {
        CpuL2WfiPwrdnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu l2 interrupt wake enable."]
    #[inline(always)]
    pub fn cpu_l2_int_wakeup_en(&self) -> CpuL2IntWakeupEnR {
        CpuL2IntWakeupEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - cpu l2 software wakeup source."]
    #[inline(always)]
    pub fn cpu_l2_sft_wakeup(&self) -> CpuL2SftWakeupR {
        CpuL2SftWakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cpu_l2 wfi power down enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l2_wfi_pwrdn_en(&mut self) -> CpuL2WfiPwrdnEnW<PmuCpu2apmConSpec> {
        CpuL2WfiPwrdnEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cpu l2 interrupt wake enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l2_int_wakeup_en(&mut self) -> CpuL2IntWakeupEnW<PmuCpu2apmConSpec> {
        CpuL2IntWakeupEnW::new(self, 1)
    }
    #[doc = "Bit 3 - cpu l2 software wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l2_sft_wakeup(&mut self) -> CpuL2SftWakeupW<PmuCpu2apmConSpec> {
        CpuL2SftWakeupW::new(self, 3)
    }
}
#[doc = "pmu cpu2 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu2apm_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu2apm_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuCpu2apmConSpec;
impl crate::RegisterSpec for PmuCpu2apmConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_cpu2apm_con::R`](R) reader structure"]
impl crate::Readable for PmuCpu2apmConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_cpu2apm_con::W`](W) writer structure"]
impl crate::Writable for PmuCpu2apmConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_CPU2APM_CON to value 0"]
impl crate::Resettable for PmuCpu2apmConSpec {
    const RESET_VALUE: u32 = 0;
}
