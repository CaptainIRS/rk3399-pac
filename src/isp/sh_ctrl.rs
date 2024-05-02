#[doc = "Register `SH_CTRL` reader"]
pub type R = crate::R<ShCtrlSpec>;
#[doc = "Register `SH_CTRL` writer"]
pub type W = crate::W<ShCtrlSpec>;
#[doc = "mechanical shutter enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShEn {
    #[doc = "0: mechanical shutter function is disabled"]
    B0 = 0,
    #[doc = "1: mechanical shutter function is enabled"]
    B1 = 1,
}
impl From<ShEn> for bool {
    #[inline(always)]
    fn from(variant: ShEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sh_en` reader - mechanical shutter enable"]
pub type ShEnR = crate::BitReader<ShEn>;
impl ShEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShEn {
        match self.bits {
            false => ShEn::B0,
            true => ShEn::B1,
        }
    }
    #[doc = "mechanical shutter function is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ShEn::B0
    }
    #[doc = "mechanical shutter function is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ShEn::B1
    }
}
#[doc = "Field `sh_en` writer - mechanical shutter enable"]
pub type ShEnW<'a, REG> = crate::BitWriter<'a, REG, ShEn>;
impl<'a, REG> ShEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "mechanical shutter function is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ShEn::B0)
    }
    #[doc = "mechanical shutter function is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ShEn::B1)
    }
}
#[doc = "mechanical shutter repetition enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShRepEn {
    #[doc = "0: shutter is opened only once"]
    B0 = 0,
    #[doc = "1: shutter is opened with the repetition rate of the trigger signal"]
    B1 = 1,
}
impl From<ShRepEn> for bool {
    #[inline(always)]
    fn from(variant: ShRepEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sh_rep_en` reader - mechanical shutter repetition enable"]
pub type ShRepEnR = crate::BitReader<ShRepEn>;
impl ShRepEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShRepEn {
        match self.bits {
            false => ShRepEn::B0,
            true => ShRepEn::B1,
        }
    }
    #[doc = "shutter is opened only once"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ShRepEn::B0
    }
    #[doc = "shutter is opened with the repetition rate of the trigger signal"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ShRepEn::B1
    }
}
#[doc = "Field `sh_rep_en` writer - mechanical shutter repetition enable"]
pub type ShRepEnW<'a, REG> = crate::BitWriter<'a, REG, ShRepEn>;
impl<'a, REG> ShRepEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "shutter is opened only once"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ShRepEn::B0)
    }
    #[doc = "shutter is opened with the repetition rate of the trigger signal"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ShRepEn::B1)
    }
}
#[doc = "mechanical shutter trigger source\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShTrigSrc {
    #[doc = "0: use “vds_vsync” for trigger event"]
    B0 = 0,
    #[doc = "1: use “shutter_trig” for trigger event"]
    B1 = 1,
}
impl From<ShTrigSrc> for bool {
    #[inline(always)]
    fn from(variant: ShTrigSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sh_trig_src` reader - mechanical shutter trigger source"]
pub type ShTrigSrcR = crate::BitReader<ShTrigSrc>;
impl ShTrigSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShTrigSrc {
        match self.bits {
            false => ShTrigSrc::B0,
            true => ShTrigSrc::B1,
        }
    }
    #[doc = "use “vds_vsync” for trigger event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ShTrigSrc::B0
    }
    #[doc = "use “shutter_trig” for trigger event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ShTrigSrc::B1
    }
}
#[doc = "Field `sh_trig_src` writer - mechanical shutter trigger source"]
pub type ShTrigSrcW<'a, REG> = crate::BitWriter<'a, REG, ShTrigSrc>;
impl<'a, REG> ShTrigSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use “vds_vsync” for trigger event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ShTrigSrc::B0)
    }
    #[doc = "use “shutter_trig” for trigger event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ShTrigSrc::B1)
    }
}
#[doc = "mechanical shutter trigger edge\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShTrigEn {
    #[doc = "0: use negative edge of trigger signal"]
    B0 = 0,
    #[doc = "1: use positive edge of trigger signal"]
    B1 = 1,
}
impl From<ShTrigEn> for bool {
    #[inline(always)]
    fn from(variant: ShTrigEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sh_trig_en` reader - mechanical shutter trigger edge"]
pub type ShTrigEnR = crate::BitReader<ShTrigEn>;
impl ShTrigEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShTrigEn {
        match self.bits {
            false => ShTrigEn::B0,
            true => ShTrigEn::B1,
        }
    }
    #[doc = "use negative edge of trigger signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ShTrigEn::B0
    }
    #[doc = "use positive edge of trigger signal"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ShTrigEn::B1
    }
}
#[doc = "Field `sh_trig_en` writer - mechanical shutter trigger edge"]
pub type ShTrigEnW<'a, REG> = crate::BitWriter<'a, REG, ShTrigEn>;
impl<'a, REG> ShTrigEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use negative edge of trigger signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ShTrigEn::B0)
    }
    #[doc = "use positive edge of trigger signal"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ShTrigEn::B1)
    }
}
#[doc = "shutter_open polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShOpenPol {
    #[doc = "0: shutter_open is high active"]
    B0 = 0,
    #[doc = "1: shutter_open is low active"]
    B1 = 1,
}
impl From<ShOpenPol> for bool {
    #[inline(always)]
    fn from(variant: ShOpenPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sh_open_pol` reader - shutter_open polarity"]
pub type ShOpenPolR = crate::BitReader<ShOpenPol>;
impl ShOpenPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShOpenPol {
        match self.bits {
            false => ShOpenPol::B0,
            true => ShOpenPol::B1,
        }
    }
    #[doc = "shutter_open is high active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ShOpenPol::B0
    }
    #[doc = "shutter_open is low active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ShOpenPol::B1
    }
}
#[doc = "Field `sh_open_pol` writer - shutter_open polarity"]
pub type ShOpenPolW<'a, REG> = crate::BitWriter<'a, REG, ShOpenPol>;
impl<'a, REG> ShOpenPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "shutter_open is high active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ShOpenPol::B0)
    }
    #[doc = "shutter_open is low active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ShOpenPol::B1)
    }
}
impl R {
    #[doc = "Bit 0 - mechanical shutter enable"]
    #[inline(always)]
    pub fn sh_en(&self) -> ShEnR {
        ShEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mechanical shutter repetition enable"]
    #[inline(always)]
    pub fn sh_rep_en(&self) -> ShRepEnR {
        ShRepEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - mechanical shutter trigger source"]
    #[inline(always)]
    pub fn sh_trig_src(&self) -> ShTrigSrcR {
        ShTrigSrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - mechanical shutter trigger edge"]
    #[inline(always)]
    pub fn sh_trig_en(&self) -> ShTrigEnR {
        ShTrigEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - shutter_open polarity"]
    #[inline(always)]
    pub fn sh_open_pol(&self) -> ShOpenPolR {
        ShOpenPolR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mechanical shutter enable"]
    #[inline(always)]
    #[must_use]
    pub fn sh_en(&mut self) -> ShEnW<ShCtrlSpec> {
        ShEnW::new(self, 0)
    }
    #[doc = "Bit 1 - mechanical shutter repetition enable"]
    #[inline(always)]
    #[must_use]
    pub fn sh_rep_en(&mut self) -> ShRepEnW<ShCtrlSpec> {
        ShRepEnW::new(self, 1)
    }
    #[doc = "Bit 2 - mechanical shutter trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn sh_trig_src(&mut self) -> ShTrigSrcW<ShCtrlSpec> {
        ShTrigSrcW::new(self, 2)
    }
    #[doc = "Bit 3 - mechanical shutter trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn sh_trig_en(&mut self) -> ShTrigEnW<ShCtrlSpec> {
        ShTrigEnW::new(self, 3)
    }
    #[doc = "Bit 4 - shutter_open polarity"]
    #[inline(always)]
    #[must_use]
    pub fn sh_open_pol(&mut self) -> ShOpenPolW<ShCtrlSpec> {
        ShOpenPolW::new(self, 4)
    }
}
#[doc = "mechanical shutter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShCtrlSpec;
impl crate::RegisterSpec for ShCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sh_ctrl::R`](R) reader structure"]
impl crate::Readable for ShCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sh_ctrl::W`](W) writer structure"]
impl crate::Writable for ShCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SH_CTRL to value 0"]
impl crate::Resettable for ShCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
