#[doc = "Register `PMU_WAKEUP_CFG0` reader"]
pub type R = crate::R<PmuWakeupCfg0Spec>;
#[doc = "Register `PMU_WAKEUP_CFG0` writer"]
pub type W = crate::W<PmuWakeupCfg0Spec>;
#[doc = "gpio0a posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0aPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_POSEDGE_EN` reader - gpio0a posedge pulse wakeup enable"]
pub type Gpio0aPosedgeEnR = crate::FieldReader<Gpio0aPosedgeEn>;
impl Gpio0aPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aPosedgeEn> {
        match self.bits {
            0 => Some(Gpio0aPosedgeEn::B0),
            1 => Some(Gpio0aPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aPosedgeEn::B1
    }
}
#[doc = "Field `GPIO0A_POSEDGE_EN` writer - gpio0a posedge pulse wakeup enable"]
pub type Gpio0aPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aPosedgeEn>;
impl<'a, REG> Gpio0aPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aPosedgeEn::B1)
    }
}
#[doc = "gpio0b posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0bPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_POSEDGE_EN` reader - gpio0b posedge pulse wakeup enable"]
pub type Gpio0bPosedgeEnR = crate::FieldReader<Gpio0bPosedgeEn>;
impl Gpio0bPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bPosedgeEn> {
        match self.bits {
            0 => Some(Gpio0bPosedgeEn::B0),
            1 => Some(Gpio0bPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bPosedgeEn::B1
    }
}
#[doc = "Field `GPIO0B_POSEDGE_EN` writer - gpio0b posedge pulse wakeup enable"]
pub type Gpio0bPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bPosedgeEn>;
impl<'a, REG> Gpio0bPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bPosedgeEn::B1)
    }
}
#[doc = "gpio0c posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0cPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0cPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0cPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0cPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0C_POSEDGE_EN` reader - gpio0c posedge pulse wakeup enable"]
pub type Gpio0cPosedgeEnR = crate::FieldReader<Gpio0cPosedgeEn>;
impl Gpio0cPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0cPosedgeEn> {
        match self.bits {
            0 => Some(Gpio0cPosedgeEn::B0),
            1 => Some(Gpio0cPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0cPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0cPosedgeEn::B1
    }
}
#[doc = "Field `GPIO0C_POSEDGE_EN` writer - gpio0c posedge pulse wakeup enable"]
pub type Gpio0cPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0cPosedgeEn>;
impl<'a, REG> Gpio0cPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cPosedgeEn::B1)
    }
}
#[doc = "gpio0d posedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dPosedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0dPosedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dPosedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dPosedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_POSEDGE_EN` reader - gpio0d posedge pulse wakeup enable"]
pub type Gpio0dPosedgeEnR = crate::FieldReader<Gpio0dPosedgeEn>;
impl Gpio0dPosedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dPosedgeEn> {
        match self.bits {
            0 => Some(Gpio0dPosedgeEn::B0),
            1 => Some(Gpio0dPosedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dPosedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dPosedgeEn::B1
    }
}
#[doc = "Field `GPIO0D_POSEDGE_EN` writer - gpio0d posedge pulse wakeup enable"]
pub type Gpio0dPosedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dPosedgeEn>;
impl<'a, REG> Gpio0dPosedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dPosedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dPosedgeEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio0a posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0a_posedge_en(&self) -> Gpio0aPosedgeEnR {
        Gpio0aPosedgeEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio0b posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0b_posedge_en(&self) -> Gpio0bPosedgeEnR {
        Gpio0bPosedgeEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio0c posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0c_posedge_en(&self) -> Gpio0cPosedgeEnR {
        Gpio0cPosedgeEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio0d posedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0d_posedge_en(&self) -> Gpio0dPosedgeEnR {
        Gpio0dPosedgeEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio0a posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_posedge_en(&mut self) -> Gpio0aPosedgeEnW<PmuWakeupCfg0Spec> {
        Gpio0aPosedgeEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio0b posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_posedge_en(&mut self) -> Gpio0bPosedgeEnW<PmuWakeupCfg0Spec> {
        Gpio0bPosedgeEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio0c posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0c_posedge_en(&mut self) -> Gpio0cPosedgeEnW<PmuWakeupCfg0Spec> {
        Gpio0cPosedgeEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio0d posedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_posedge_en(&mut self) -> Gpio0dPosedgeEnW<PmuWakeupCfg0Spec> {
        Gpio0dPosedgeEnW::new(self, 24)
    }
}
#[doc = "pmu wakeup configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuWakeupCfg0Spec;
impl crate::RegisterSpec for PmuWakeupCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_wakeup_cfg0::R`](R) reader structure"]
impl crate::Readable for PmuWakeupCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmu_wakeup_cfg0::W`](W) writer structure"]
impl crate::Writable for PmuWakeupCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_WAKEUP_CFG0 to value 0"]
impl crate::Resettable for PmuWakeupCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
