#[doc = "Register `PMU_WAKEUP_CFG2` reader"]
pub type R = crate::R<PmuWakeupCfg2Spec>;
#[doc = "Register `PMU_WAKEUP_CFG2` writer"]
pub type W = crate::W<PmuWakeupCfg2Spec>;
#[doc = "gpio1a posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1aPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_POSEDGE_EN` reader - gpio1a posedge pulse wakeup enable"]
pub type Gpio1aPosedgeEnR = crate::FieldReader<Gpio1aPosedgeEn>;
impl Gpio1aPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aPosedgeEn> {
        match self.bits {
            0 => Some(Gpio1aPosedgeEn::B0),
            1 => Some(Gpio1aPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aPosedgeEn::B1
    }
}
#[doc = "Field `GPIO1A_POSEDGE_EN` writer - gpio1a posedge pulse wakeup enable"]
pub type Gpio1aPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aPosedgeEn>;
impl<'a, REG> Gpio1aPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aPosedgeEn::B1)
    }
}
#[doc = "gpio1b posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1bPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_POSEDGE_EN` reader - gpio1b posedge pulse wakeup enable"]
pub type Gpio1bPosedgeEnR = crate::FieldReader<Gpio1bPosedgeEn>;
impl Gpio1bPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bPosedgeEn> {
        match self.bits {
            0 => Some(Gpio1bPosedgeEn::B0),
            1 => Some(Gpio1bPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bPosedgeEn::B1
    }
}
#[doc = "Field `GPIO1B_POSEDGE_EN` writer - gpio1b posedge pulse wakeup enable"]
pub type Gpio1bPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bPosedgeEn>;
impl<'a, REG> Gpio1bPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bPosedgeEn::B1)
    }
}
#[doc = "gpio1c posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1cPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_POSEDGE_EN` reader - gpio1c posedge pulse wakeup enable"]
pub type Gpio1cPosedgeEnR = crate::FieldReader<Gpio1cPosedgeEn>;
impl Gpio1cPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cPosedgeEn> {
        match self.bits {
            0 => Some(Gpio1cPosedgeEn::B0),
            1 => Some(Gpio1cPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cPosedgeEn::B1
    }
}
#[doc = "Field `GPIO1C_POSEDGE_EN` writer - gpio1c posedge pulse wakeup enable"]
pub type Gpio1cPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cPosedgeEn>;
impl<'a, REG> Gpio1cPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cPosedgeEn::B1)
    }
}
#[doc = "gpio1d posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1dPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1dPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1dPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1D_POSEDGE_EN` reader - gpio1d posedge pulse wakeup enable"]
pub type Gpio1dPosedgeEnR = crate::FieldReader<Gpio1dPosedgeEn>;
impl Gpio1dPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dPosedgeEn> {
        match self.bits {
            0 => Some(Gpio1dPosedgeEn::B0),
            1 => Some(Gpio1dPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1dPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1dPosedgeEn::B1
    }
}
#[doc = "Field `GPIO1D_POSEDGE_EN` writer - gpio1d posedge pulse wakeup enable"]
pub type Gpio1dPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1dPosedgeEn>;
impl<'a, REG> Gpio1dPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dPosedgeEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio1a posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1a_posedge_en(&self) -> Gpio1aPosedgeEnR {
        Gpio1aPosedgeEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio1b posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1b_posedge_en(&self) -> Gpio1bPosedgeEnR {
        Gpio1bPosedgeEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio1c posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1c_posedge_en(&self) -> Gpio1cPosedgeEnR {
        Gpio1cPosedgeEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio1d posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio1d_posedge_en(&self) -> Gpio1dPosedgeEnR {
        Gpio1dPosedgeEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio1a posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_posedge_en(&mut self) -> Gpio1aPosedgeEnW<PmuWakeupCfg2Spec> {
        Gpio1aPosedgeEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio1b posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_posedge_en(&mut self) -> Gpio1bPosedgeEnW<PmuWakeupCfg2Spec> {
        Gpio1bPosedgeEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio1c posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_posedge_en(&mut self) -> Gpio1cPosedgeEnW<PmuWakeupCfg2Spec> {
        Gpio1cPosedgeEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio1d posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_posedge_en(&mut self) -> Gpio1dPosedgeEnW<PmuWakeupCfg2Spec> {
        Gpio1dPosedgeEnW::new(self, 24)
    }
}
#[doc = "pmu wakeup configure register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuWakeupCfg2Spec;
impl crate::RegisterSpec for PmuWakeupCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_wakeup_cfg2::R`](R) reader structure"]
impl crate::Readable for PmuWakeupCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmu_wakeup_cfg2::W`](W) writer structure"]
impl crate::Writable for PmuWakeupCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_WAKEUP_CFG2 to value 0"]
impl crate::Resettable for PmuWakeupCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
