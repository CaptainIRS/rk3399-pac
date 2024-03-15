#[doc = "Register `PMU_CPU1APM_CON` reader"]
pub type R = crate::R<PmuCpu1apmConSpec>;
#[doc = "Register `PMU_CPU1APM_CON` writer"]
pub type W = crate::W<PmuCpu1apmConSpec>;
#[doc = "cpu_l1 wfi power down enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL1WfiPwrdnEn {
    #[doc = "0: enable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL1WfiPwrdnEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL1WfiPwrdnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L1_WFI_PWRDN_EN` reader - cpu_l1 wfi power down enable."]
pub type CpuL1WfiPwrdnEnR = crate::BitReader<CpuL1WfiPwrdnEn>;
impl CpuL1WfiPwrdnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL1WfiPwrdnEn {
        match self.bits {
            false => CpuL1WfiPwrdnEn::B0,
            true => CpuL1WfiPwrdnEn::B1,
        }
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL1WfiPwrdnEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL1WfiPwrdnEn::B1
    }
}
#[doc = "Field `CPU_L1_WFI_PWRDN_EN` writer - cpu_l1 wfi power down enable."]
pub type CpuL1WfiPwrdnEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL1WfiPwrdnEn>;
impl<'a, REG> CpuL1WfiPwrdnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL1WfiPwrdnEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL1WfiPwrdnEn::B1)
    }
}
#[doc = "cpu l1 interrupt wake enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL1IntWakeupEn {
    #[doc = "0: enable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL1IntWakeupEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL1IntWakeupEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L1_INT_WAKEUP_EN` reader - cpu l1 interrupt wake enable."]
pub type CpuL1IntWakeupEnR = crate::BitReader<CpuL1IntWakeupEn>;
impl CpuL1IntWakeupEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL1IntWakeupEn {
        match self.bits {
            false => CpuL1IntWakeupEn::B0,
            true => CpuL1IntWakeupEn::B1,
        }
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL1IntWakeupEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL1IntWakeupEn::B1
    }
}
#[doc = "Field `CPU_L1_INT_WAKEUP_EN` writer - cpu l1 interrupt wake enable."]
pub type CpuL1IntWakeupEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL1IntWakeupEn>;
impl<'a, REG> CpuL1IntWakeupEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL1IntWakeupEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL1IntWakeupEn::B1)
    }
}
#[doc = "cpu l1 software wakeup source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL1SftWakeup {
    #[doc = "1: nothing ;"]
    B1 = 1,
    #[doc = "0: nothing ;"]
    B0 = 0,
}
impl From<CpuL1SftWakeup> for bool {
    #[inline(always)]
    fn from(variant: CpuL1SftWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L1_SFT_WAKEUP` reader - cpu l1 software wakeup source."]
pub type CpuL1SftWakeupR = crate::BitReader<CpuL1SftWakeup>;
impl CpuL1SftWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL1SftWakeup {
        match self.bits {
            true => CpuL1SftWakeup::B1,
            false => CpuL1SftWakeup::B0,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL1SftWakeup::B1
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL1SftWakeup::B0
    }
}
#[doc = "Field `CPU_L1_SFT_WAKEUP` writer - cpu l1 software wakeup source."]
pub type CpuL1SftWakeupW<'a, REG> = crate::BitWriter<'a, REG, CpuL1SftWakeup>;
impl<'a, REG> CpuL1SftWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL1SftWakeup::B1)
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL1SftWakeup::B0)
    }
}
impl R {
    #[doc = "Bit 0 - cpu_l1 wfi power down enable."]
    #[inline(always)]
    pub fn cpu_l1_wfi_pwrdn_en(&self) -> CpuL1WfiPwrdnEnR {
        CpuL1WfiPwrdnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu l1 interrupt wake enable."]
    #[inline(always)]
    pub fn cpu_l1_int_wakeup_en(&self) -> CpuL1IntWakeupEnR {
        CpuL1IntWakeupEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - cpu l1 software wakeup source."]
    #[inline(always)]
    pub fn cpu_l1_sft_wakeup(&self) -> CpuL1SftWakeupR {
        CpuL1SftWakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cpu_l1 wfi power down enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l1_wfi_pwrdn_en(&mut self) -> CpuL1WfiPwrdnEnW<PmuCpu1apmConSpec> {
        CpuL1WfiPwrdnEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cpu l1 interrupt wake enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l1_int_wakeup_en(&mut self) -> CpuL1IntWakeupEnW<PmuCpu1apmConSpec> {
        CpuL1IntWakeupEnW::new(self, 1)
    }
    #[doc = "Bit 3 - cpu l1 software wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l1_sft_wakeup(&mut self) -> CpuL1SftWakeupW<PmuCpu1apmConSpec> {
        CpuL1SftWakeupW::new(self, 3)
    }
}
#[doc = "pmu cpu1 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu1apm_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu1apm_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuCpu1apmConSpec;
impl crate::RegisterSpec for PmuCpu1apmConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_cpu1apm_con::R`](R) reader structure"]
impl crate::Readable for PmuCpu1apmConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_cpu1apm_con::W`](W) writer structure"]
impl crate::Writable for PmuCpu1apmConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_CPU1APM_CON to value 0"]
impl crate::Resettable for PmuCpu1apmConSpec {
    const RESET_VALUE: u32 = 0;
}
