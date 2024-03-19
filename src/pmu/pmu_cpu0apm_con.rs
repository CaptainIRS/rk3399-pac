#[doc = "Register `PMU_CPU0APM_CON` reader"]
pub type R = crate::R<PmuCpu0apmConSpec>;
#[doc = "Register `PMU_CPU0APM_CON` writer"]
pub type W = crate::W<PmuCpu0apmConSpec>;
#[doc = "cpu_l0 wfi power down enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL0WfiPwrdnEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL0WfiPwrdnEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL0WfiPwrdnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L0_WFI_PWRDN_EN` reader - cpu_l0 wfi power down enable."]
pub type CpuL0WfiPwrdnEnR = crate::BitReader<CpuL0WfiPwrdnEn>;
impl CpuL0WfiPwrdnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL0WfiPwrdnEn {
        match self.bits {
            false => CpuL0WfiPwrdnEn::B0,
            true => CpuL0WfiPwrdnEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL0WfiPwrdnEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL0WfiPwrdnEn::B1
    }
}
#[doc = "Field `CPU_L0_WFI_PWRDN_EN` writer - cpu_l0 wfi power down enable."]
pub type CpuL0WfiPwrdnEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL0WfiPwrdnEn>;
impl<'a, REG> CpuL0WfiPwrdnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL0WfiPwrdnEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL0WfiPwrdnEn::B1)
    }
}
#[doc = "cpu l0 interrupt wake enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL0IntWakeupEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL0IntWakeupEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL0IntWakeupEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L0_INT_WAKEUP_EN` reader - cpu l0 interrupt wake enable."]
pub type CpuL0IntWakeupEnR = crate::BitReader<CpuL0IntWakeupEn>;
impl CpuL0IntWakeupEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL0IntWakeupEn {
        match self.bits {
            false => CpuL0IntWakeupEn::B0,
            true => CpuL0IntWakeupEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL0IntWakeupEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL0IntWakeupEn::B1
    }
}
#[doc = "Field `CPU_L0_INT_WAKEUP_EN` writer - cpu l0 interrupt wake enable."]
pub type CpuL0IntWakeupEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL0IntWakeupEn>;
impl<'a, REG> CpuL0IntWakeupEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL0IntWakeupEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL0IntWakeupEn::B1)
    }
}
#[doc = "cpu l0 software wakeup source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL0SftWakeup {
    #[doc = "1: wakeup ;"]
    B1 = 1,
    #[doc = "0: nothing ;"]
    B0 = 0,
}
impl From<CpuL0SftWakeup> for bool {
    #[inline(always)]
    fn from(variant: CpuL0SftWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L0_SFT_WAKEUP` reader - cpu l0 software wakeup source."]
pub type CpuL0SftWakeupR = crate::BitReader<CpuL0SftWakeup>;
impl CpuL0SftWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL0SftWakeup {
        match self.bits {
            true => CpuL0SftWakeup::B1,
            false => CpuL0SftWakeup::B0,
        }
    }
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL0SftWakeup::B1
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL0SftWakeup::B0
    }
}
#[doc = "Field `CPU_L0_SFT_WAKEUP` writer - cpu l0 software wakeup source."]
pub type CpuL0SftWakeupW<'a, REG> = crate::BitWriter<'a, REG, CpuL0SftWakeup>;
impl<'a, REG> CpuL0SftWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL0SftWakeup::B1)
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL0SftWakeup::B0)
    }
}
impl R {
    #[doc = "Bit 0 - cpu_l0 wfi power down enable."]
    #[inline(always)]
    pub fn cpu_l0_wfi_pwrdn_en(&self) -> CpuL0WfiPwrdnEnR {
        CpuL0WfiPwrdnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu l0 interrupt wake enable."]
    #[inline(always)]
    pub fn cpu_l0_int_wakeup_en(&self) -> CpuL0IntWakeupEnR {
        CpuL0IntWakeupEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - cpu l0 software wakeup source."]
    #[inline(always)]
    pub fn cpu_l0_sft_wakeup(&self) -> CpuL0SftWakeupR {
        CpuL0SftWakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cpu_l0 wfi power down enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l0_wfi_pwrdn_en(&mut self) -> CpuL0WfiPwrdnEnW<PmuCpu0apmConSpec> {
        CpuL0WfiPwrdnEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cpu l0 interrupt wake enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l0_int_wakeup_en(&mut self) -> CpuL0IntWakeupEnW<PmuCpu0apmConSpec> {
        CpuL0IntWakeupEnW::new(self, 1)
    }
    #[doc = "Bit 3 - cpu l0 software wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l0_sft_wakeup(&mut self) -> CpuL0SftWakeupW<PmuCpu0apmConSpec> {
        CpuL0SftWakeupW::new(self, 3)
    }
}
#[doc = "pmu cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu0apm_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu0apm_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuCpu0apmConSpec;
impl crate::RegisterSpec for PmuCpu0apmConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_cpu0apm_con::R`](R) reader structure"]
impl crate::Readable for PmuCpu0apmConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_cpu0apm_con::W`](W) writer structure"]
impl crate::Writable for PmuCpu0apmConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_CPU0APM_CON to value 0"]
impl crate::Resettable for PmuCpu0apmConSpec {
    const RESET_VALUE: u32 = 0;
}
