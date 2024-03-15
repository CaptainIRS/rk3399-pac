#[doc = "Register `PMU_WAKEUP_CFG3` reader"]
pub type R = crate::R<PmuWakeupCfg3Spec>;
#[doc = "Register `PMU_WAKEUP_CFG3` writer"]
pub type W = crate::W<PmuWakeupCfg3Spec>;
#[doc = "gpio1a negedge pulse wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aNegedgeEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1aNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_NEGEDGE_EN` reader - gpio1a negedge pulse wakeup enable"]
pub type Gpio1aNegedgeEnR = crate::FieldReader<Gpio1aNegedgeEn>;
impl Gpio1aNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aNegedgeEn> {
        match self.bits {
            0 => Some(Gpio1aNegedgeEn::B0),
            1 => Some(Gpio1aNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aNegedgeEn::B1
    }
}
#[doc = "Field `GPIO1A_NEGEDGE_EN` writer - gpio1a negedge pulse wakeup enable"]
pub type Gpio1aNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aNegedgeEn>;
impl<'a, REG> Gpio1aNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aNegedgeEn::B1)
    }
}
#[doc = "gpio1b negedge pulse wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bNegedgeEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1bNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_NEGEDGE_EN` reader - gpio1b negedge pulse wakeup enable"]
pub type Gpio1bNegedgeEnR = crate::FieldReader<Gpio1bNegedgeEn>;
impl Gpio1bNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bNegedgeEn> {
        match self.bits {
            0 => Some(Gpio1bNegedgeEn::B0),
            1 => Some(Gpio1bNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bNegedgeEn::B1
    }
}
#[doc = "Field `GPIO1B_NEGEDGE_EN` writer - gpio1b negedge pulse wakeup enable"]
pub type Gpio1bNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bNegedgeEn>;
impl<'a, REG> Gpio1bNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bNegedgeEn::B1)
    }
}
#[doc = "gpio1c negedge pulse wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cNegedgeEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1cNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_NEGEDGE_EN` reader - gpio1c negedge pulse wakeup enable"]
pub type Gpio1cNegedgeEnR = crate::FieldReader<Gpio1cNegedgeEn>;
impl Gpio1cNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cNegedgeEn> {
        match self.bits {
            0 => Some(Gpio1cNegedgeEn::B0),
            1 => Some(Gpio1cNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cNegedgeEn::B1
    }
}
#[doc = "Field `GPIO1C_NEGEDGE_EN` writer - gpio1c negedge pulse wakeup enable"]
pub type Gpio1cNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cNegedgeEn>;
impl<'a, REG> Gpio1cNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cNegedgeEn::B1)
    }
}
#[doc = "gpio1d negedge pulse wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1dNegedgeEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1dNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1dNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1D_NEGEDGE_EN` reader - gpio1d negedge pulse wakeup enable"]
pub type Gpio1dNegedgeEnR = crate::FieldReader<Gpio1dNegedgeEn>;
impl Gpio1dNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dNegedgeEn> {
        match self.bits {
            0 => Some(Gpio1dNegedgeEn::B0),
            1 => Some(Gpio1dNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1dNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1dNegedgeEn::B1
    }
}
#[doc = "Field `GPIO1D_NEGEDGE_EN` writer - gpio1d negedge pulse wakeup enable"]
pub type Gpio1dNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1dNegedgeEn>;
impl<'a, REG> Gpio1dNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dNegedgeEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio1a negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1a_negedge_en(&self) -> Gpio1aNegedgeEnR {
        Gpio1aNegedgeEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio1b negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1b_negedge_en(&self) -> Gpio1bNegedgeEnR {
        Gpio1bNegedgeEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio1c negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1c_negedge_en(&self) -> Gpio1cNegedgeEnR {
        Gpio1cNegedgeEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio1d negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1d_negedge_en(&self) -> Gpio1dNegedgeEnR {
        Gpio1dNegedgeEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio1a negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_negedge_en(&mut self) -> Gpio1aNegedgeEnW<PmuWakeupCfg3Spec> {
        Gpio1aNegedgeEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio1b negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_negedge_en(&mut self) -> Gpio1bNegedgeEnW<PmuWakeupCfg3Spec> {
        Gpio1bNegedgeEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio1c negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_negedge_en(&mut self) -> Gpio1cNegedgeEnW<PmuWakeupCfg3Spec> {
        Gpio1cNegedgeEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio1d negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_negedge_en(&mut self) -> Gpio1dNegedgeEnW<PmuWakeupCfg3Spec> {
        Gpio1dNegedgeEnW::new(self, 24)
    }
}
#[doc = "pmu wakeup configure register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuWakeupCfg3Spec;
impl crate::RegisterSpec for PmuWakeupCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_wakeup_cfg3::R`](R) reader structure"]
impl crate::Readable for PmuWakeupCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pmu_wakeup_cfg3::W`](W) writer structure"]
impl crate::Writable for PmuWakeupCfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_WAKEUP_CFG3 to value 0"]
impl crate::Resettable for PmuWakeupCfg3Spec {
    const RESET_VALUE: u32 = 0;
}
