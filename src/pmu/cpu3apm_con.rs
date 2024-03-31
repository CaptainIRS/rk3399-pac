#[doc = "Register `CPU3APM_CON` reader"]
pub type R = crate::R<Cpu3apmConSpec>;
#[doc = "Register `CPU3APM_CON` writer"]
pub type W = crate::W<Cpu3apmConSpec>;
#[doc = "cpu_l3 wfi power down enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL3WfiPwrdnEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL3WfiPwrdnEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL3WfiPwrdnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L3_WFI_PWRDN_EN` reader - cpu_l3 wfi power down enable."]
pub type CpuL3WfiPwrdnEnR = crate::BitReader<CpuL3WfiPwrdnEn>;
impl CpuL3WfiPwrdnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL3WfiPwrdnEn {
        match self.bits {
            false => CpuL3WfiPwrdnEn::B0,
            true => CpuL3WfiPwrdnEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL3WfiPwrdnEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL3WfiPwrdnEn::B1
    }
}
#[doc = "Field `CPU_L3_WFI_PWRDN_EN` writer - cpu_l3 wfi power down enable."]
pub type CpuL3WfiPwrdnEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL3WfiPwrdnEn>;
impl<'a, REG> CpuL3WfiPwrdnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL3WfiPwrdnEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL3WfiPwrdnEn::B1)
    }
}
#[doc = "cpu l3 interrupt wake enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL3IntWakeupEn {
    #[doc = "0: disable ;"]
    B0 = 0,
    #[doc = "1: enable ;"]
    B1 = 1,
}
impl From<CpuL3IntWakeupEn> for bool {
    #[inline(always)]
    fn from(variant: CpuL3IntWakeupEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L3_INT_WAKEUP_EN` reader - cpu l3 interrupt wake enable."]
pub type CpuL3IntWakeupEnR = crate::BitReader<CpuL3IntWakeupEn>;
impl CpuL3IntWakeupEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL3IntWakeupEn {
        match self.bits {
            false => CpuL3IntWakeupEn::B0,
            true => CpuL3IntWakeupEn::B1,
        }
    }
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL3IntWakeupEn::B0
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL3IntWakeupEn::B1
    }
}
#[doc = "Field `CPU_L3_INT_WAKEUP_EN` writer - cpu l3 interrupt wake enable."]
pub type CpuL3IntWakeupEnW<'a, REG> = crate::BitWriter<'a, REG, CpuL3IntWakeupEn>;
impl<'a, REG> CpuL3IntWakeupEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL3IntWakeupEn::B0)
    }
    #[doc = "enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL3IntWakeupEn::B1)
    }
}
#[doc = "cpu l3 software wakeup source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuL3SftWakeup {
    #[doc = "1: wakeup ;"]
    B1 = 1,
    #[doc = "0: nothing ;"]
    B0 = 0,
}
impl From<CpuL3SftWakeup> for bool {
    #[inline(always)]
    fn from(variant: CpuL3SftWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_L3_SFT_WAKEUP` reader - cpu l3 software wakeup source."]
pub type CpuL3SftWakeupR = crate::BitReader<CpuL3SftWakeup>;
impl CpuL3SftWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuL3SftWakeup {
        match self.bits {
            true => CpuL3SftWakeup::B1,
            false => CpuL3SftWakeup::B0,
        }
    }
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CpuL3SftWakeup::B1
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CpuL3SftWakeup::B0
    }
}
#[doc = "Field `CPU_L3_SFT_WAKEUP` writer - cpu l3 software wakeup source."]
pub type CpuL3SftWakeupW<'a, REG> = crate::BitWriter<'a, REG, CpuL3SftWakeup>;
impl<'a, REG> CpuL3SftWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wakeup ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL3SftWakeup::B1)
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CpuL3SftWakeup::B0)
    }
}
impl R {
    #[doc = "Bit 0 - cpu_l3 wfi power down enable."]
    #[inline(always)]
    pub fn cpu_l3_wfi_pwrdn_en(&self) -> CpuL3WfiPwrdnEnR {
        CpuL3WfiPwrdnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu l3 interrupt wake enable."]
    #[inline(always)]
    pub fn cpu_l3_int_wakeup_en(&self) -> CpuL3IntWakeupEnR {
        CpuL3IntWakeupEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - cpu l3 software wakeup source."]
    #[inline(always)]
    pub fn cpu_l3_sft_wakeup(&self) -> CpuL3SftWakeupR {
        CpuL3SftWakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cpu_l3 wfi power down enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l3_wfi_pwrdn_en(&mut self) -> CpuL3WfiPwrdnEnW<Cpu3apmConSpec> {
        CpuL3WfiPwrdnEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cpu l3 interrupt wake enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l3_int_wakeup_en(&mut self) -> CpuL3IntWakeupEnW<Cpu3apmConSpec> {
        CpuL3IntWakeupEnW::new(self, 1)
    }
    #[doc = "Bit 3 - cpu l3 software wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_l3_sft_wakeup(&mut self) -> CpuL3SftWakeupW<Cpu3apmConSpec> {
        CpuL3SftWakeupW::new(self, 3)
    }
}
#[doc = "pmu cpu3 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu3apm_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu3apm_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu3apmConSpec;
impl crate::RegisterSpec for Cpu3apmConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu3apm_con::R`](R) reader structure"]
impl crate::Readable for Cpu3apmConSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu3apm_con::W`](W) writer structure"]
impl crate::Writable for Cpu3apmConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU3APM_CON to value 0"]
impl crate::Resettable for Cpu3apmConSpec {
    const RESET_VALUE: u32 = 0;
}
