#[doc = "Register `PMU_CPU1BPM_CON` reader"]
pub type R = crate::R<PmuCpu1bpmConSpec>;
#[doc = "Register `PMU_CPU1BPM_CON` writer"]
pub type W = crate::W<PmuCpu1bpmConSpec>;
#[doc = "cpu_b0 wfi power down enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuB0WfiPwrdnEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuB0WfiPwrdnEn> for bool {
    #[inline(always)]
    fn from(variant: CpuB0WfiPwrdnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_B0_WFI_PWRDN_EN` reader - cpu_b0 wfi power down enable."]
pub type CpuB0WfiPwrdnEnR = crate::BitReader<CpuB0WfiPwrdnEn>;
impl CpuB0WfiPwrdnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuB0WfiPwrdnEn {
        match self.bits {
            false => CpuB0WfiPwrdnEn::B0,
            true => CpuB0WfiPwrdnEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuB0WfiPwrdnEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuB0WfiPwrdnEn::B1
    }
}
#[doc = "Field `CPU_B0_WFI_PWRDN_EN` writer - cpu_b0 wfi power down enable."]
pub type CpuB0WfiPwrdnEnW<'a, REG> = crate::BitWriter<'a, REG, CpuB0WfiPwrdnEn>;
impl<'a, REG> CpuB0WfiPwrdnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuB0WfiPwrdnEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuB0WfiPwrdnEn::B1)
    }
}
#[doc = "cpu b0 interrupt wake enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuB0IntWakeupEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuB0IntWakeupEn> for bool {
    #[inline(always)]
    fn from(variant: CpuB0IntWakeupEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_B0_INT_WAKEUP_EN` reader - cpu b0 interrupt wake enable."]
pub type CpuB0IntWakeupEnR = crate::BitReader<CpuB0IntWakeupEn>;
impl CpuB0IntWakeupEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuB0IntWakeupEn {
        match self.bits {
            false => CpuB0IntWakeupEn::B0,
            true => CpuB0IntWakeupEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuB0IntWakeupEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuB0IntWakeupEn::B1
    }
}
#[doc = "Field `CPU_B0_INT_WAKEUP_EN` writer - cpu b0 interrupt wake enable."]
pub type CpuB0IntWakeupEnW<'a, REG> = crate::BitWriter<'a, REG, CpuB0IntWakeupEn>;
impl<'a, REG> CpuB0IntWakeupEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuB0IntWakeupEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuB0IntWakeupEn::B1)
    }
}
#[doc = "cpu b0 software wakeup source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuB0SftWakeup {
    #[doc = "1: wakeup ;"]
    B1 = 1,
    #[doc = "0: nothing ;"]
    B0 = 0,
}
impl From<CpuB0SftWakeup> for bool {
    #[inline(always)]
    fn from(variant: CpuB0SftWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_B0_SFT_WAKEUP` reader - cpu b0 software wakeup source."]
pub type CpuB0SftWakeupR = crate::BitReader<CpuB0SftWakeup>;
impl CpuB0SftWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuB0SftWakeup {
        match self.bits {
            true => CpuB0SftWakeup::B1,
            false => CpuB0SftWakeup::B0,
        }
    }
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuB0SftWakeup::B1
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuB0SftWakeup::B0
    }
}
#[doc = "Field `CPU_B0_SFT_WAKEUP` writer - cpu b0 software wakeup source."]
pub type CpuB0SftWakeupW<'a, REG> = crate::BitWriter<'a, REG, CpuB0SftWakeup>;
impl<'a, REG> CpuB0SftWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuB0SftWakeup::B1)
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuB0SftWakeup::B0)
    }
}
impl R {
    #[doc = "Bit 0 - cpu_b0 wfi power down enable."]
    #[inline(always)]
    pub fn cpu_b0_wfi_pwrdn_en(&self) -> CpuB0WfiPwrdnEnR {
        CpuB0WfiPwrdnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu b0 interrupt wake enable."]
    #[inline(always)]
    pub fn cpu_b0_int_wakeup_en(&self) -> CpuB0IntWakeupEnR {
        CpuB0IntWakeupEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - cpu b0 software wakeup source."]
    #[inline(always)]
    pub fn cpu_b0_sft_wakeup(&self) -> CpuB0SftWakeupR {
        CpuB0SftWakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cpu_b0 wfi power down enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_b0_wfi_pwrdn_en(&mut self) -> CpuB0WfiPwrdnEnW<PmuCpu1bpmConSpec> {
        CpuB0WfiPwrdnEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cpu b0 interrupt wake enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_b0_int_wakeup_en(&mut self) -> CpuB0IntWakeupEnW<PmuCpu1bpmConSpec> {
        CpuB0IntWakeupEnW::new(self, 1)
    }
    #[doc = "Bit 3 - cpu b0 software wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_b0_sft_wakeup(&mut self) -> CpuB0SftWakeupW<PmuCpu1bpmConSpec> {
        CpuB0SftWakeupW::new(self, 3)
    }
}
#[doc = "pmu cluster_b cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu1bpm_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu1bpm_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuCpu1bpmConSpec;
impl crate::RegisterSpec for PmuCpu1bpmConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_cpu1bpm_con::R`](R) reader structure"]
impl crate::Readable for PmuCpu1bpmConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_cpu1bpm_con::W`](W) writer structure"]
impl crate::Writable for PmuCpu1bpmConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_CPU1BPM_CON to value 0"]
impl crate::Resettable for PmuCpu1bpmConSpec {
    const RESET_VALUE: u32 = 0;
}
