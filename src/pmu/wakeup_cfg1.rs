#[doc = "Register `WAKEUP_CFG1` reader"]
pub type R = crate::R<WakeupCfg1Spec>;
#[doc = "Register `WAKEUP_CFG1` writer"]
pub type W = crate::W<WakeupCfg1Spec>;
#[doc = "gpio0a negedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aNegedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0aNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_NEGEDGE_EN` reader - gpio0a negedge pulse wakeup enable"]
pub type Gpio0aNegedgeEnR = crate::FieldReader<Gpio0aNegedgeEn>;
impl Gpio0aNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aNegedgeEn> {
        match self.bits {
            0 => Some(Gpio0aNegedgeEn::B0),
            1 => Some(Gpio0aNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aNegedgeEn::B1
    }
}
#[doc = "Field `GPIO0A_NEGEDGE_EN` writer - gpio0a negedge pulse wakeup enable"]
pub type Gpio0aNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aNegedgeEn>;
impl<'a, REG> Gpio0aNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aNegedgeEn::B1)
    }
}
#[doc = "gpio0b negedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bNegedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0bNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_NEGEDGE_EN` reader - gpio0b negedge pulse wakeup enable"]
pub type Gpio0bNegedgeEnR = crate::FieldReader<Gpio0bNegedgeEn>;
impl Gpio0bNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bNegedgeEn> {
        match self.bits {
            0 => Some(Gpio0bNegedgeEn::B0),
            1 => Some(Gpio0bNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bNegedgeEn::B1
    }
}
#[doc = "Field `GPIO0B_NEGEDGE_EN` writer - gpio0b negedge pulse wakeup enable"]
pub type Gpio0bNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bNegedgeEn>;
impl<'a, REG> Gpio0bNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bNegedgeEn::B1)
    }
}
#[doc = "gpio0c negedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0cNegedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0cNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0cNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0cNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0C_NEGEDGE_EN` reader - gpio0c negedge pulse wakeup enable"]
pub type Gpio0cNegedgeEnR = crate::FieldReader<Gpio0cNegedgeEn>;
impl Gpio0cNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0cNegedgeEn> {
        match self.bits {
            0 => Some(Gpio0cNegedgeEn::B0),
            1 => Some(Gpio0cNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0cNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0cNegedgeEn::B1
    }
}
#[doc = "Field `GPIO0C_NEGEDGE_EN` writer - gpio0c negedge pulse wakeup enable"]
pub type Gpio0cNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0cNegedgeEn>;
impl<'a, REG> Gpio0cNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cNegedgeEn::B1)
    }
}
#[doc = "gpio0d negedge pulse wakeup enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dNegedgeEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0dNegedgeEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dNegedgeEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dNegedgeEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_NEGEDGE_EN` reader - gpio0d negedge pulse wakeup enable"]
pub type Gpio0dNegedgeEnR = crate::FieldReader<Gpio0dNegedgeEn>;
impl Gpio0dNegedgeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dNegedgeEn> {
        match self.bits {
            0 => Some(Gpio0dNegedgeEn::B0),
            1 => Some(Gpio0dNegedgeEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dNegedgeEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dNegedgeEn::B1
    }
}
#[doc = "Field `GPIO0D_NEGEDGE_EN` writer - gpio0d negedge pulse wakeup enable"]
pub type Gpio0dNegedgeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dNegedgeEn>;
impl<'a, REG> Gpio0dNegedgeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dNegedgeEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dNegedgeEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio0a negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0a_negedge_en(&self) -> Gpio0aNegedgeEnR {
        Gpio0aNegedgeEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio0b negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0b_negedge_en(&self) -> Gpio0bNegedgeEnR {
        Gpio0bNegedgeEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio0c negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0c_negedge_en(&self) -> Gpio0cNegedgeEnR {
        Gpio0cNegedgeEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio0d negedge pulse wakeup enable"]
    #[inline(always)]
    pub fn gpio0d_negedge_en(&self) -> Gpio0dNegedgeEnR {
        Gpio0dNegedgeEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio0a negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_negedge_en(&mut self) -> Gpio0aNegedgeEnW<WakeupCfg1Spec> {
        Gpio0aNegedgeEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio0b negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_negedge_en(&mut self) -> Gpio0bNegedgeEnW<WakeupCfg1Spec> {
        Gpio0bNegedgeEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio0c negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0c_negedge_en(&mut self) -> Gpio0cNegedgeEnW<WakeupCfg1Spec> {
        Gpio0cNegedgeEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio0d negedge pulse wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_negedge_en(&mut self) -> Gpio0dNegedgeEnW<WakeupCfg1Spec> {
        Gpio0dNegedgeEnW::new(self, 24)
    }
}
#[doc = "pmu wakeup configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupCfg1Spec;
impl crate::RegisterSpec for WakeupCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_cfg1::R`](R) reader structure"]
impl crate::Readable for WakeupCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_cfg1::W`](W) writer structure"]
impl crate::Writable for WakeupCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP_CFG1 to value 0"]
impl crate::Resettable for WakeupCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
