#[doc = "Register `GPIO1_POS_INT_CON` reader"]
pub type R = crate::R<Gpio1PosIntConSpec>;
#[doc = "Register `GPIO1_POS_INT_CON` writer"]
pub type W = crate::W<Gpio1PosIntConSpec>;
#[doc = "gpio1a posedge pulse interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aPosIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1aPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_POS_INT_EN` reader - gpio1a posedge pulse interrupt enable"]
pub type Gpio1aPosIntEnR = crate::FieldReader<Gpio1aPosIntEn>;
impl Gpio1aPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aPosIntEn> {
        match self.bits {
            0 => Some(Gpio1aPosIntEn::B0),
            1 => Some(Gpio1aPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aPosIntEn::B1
    }
}
#[doc = "Field `GPIO1A_POS_INT_EN` writer - gpio1a posedge pulse interrupt enable"]
pub type Gpio1aPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aPosIntEn>;
impl<'a, REG> Gpio1aPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aPosIntEn::B1)
    }
}
#[doc = "gpio1b posedge pulse interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bPosIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1bPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_POS_INT_EN` reader - gpio1b posedge pulse interrupt enable"]
pub type Gpio1bPosIntEnR = crate::FieldReader<Gpio1bPosIntEn>;
impl Gpio1bPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bPosIntEn> {
        match self.bits {
            0 => Some(Gpio1bPosIntEn::B0),
            1 => Some(Gpio1bPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bPosIntEn::B1
    }
}
#[doc = "Field `GPIO1B_POS_INT_EN` writer - gpio1b posedge pulse interrupt enable"]
pub type Gpio1bPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bPosIntEn>;
impl<'a, REG> Gpio1bPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bPosIntEn::B1)
    }
}
#[doc = "gpio1c posedge pulse interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cPosIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1cPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_POS_INT_EN` reader - gpio1c posedge pulse interrupt enable"]
pub type Gpio1cPosIntEnR = crate::FieldReader<Gpio1cPosIntEn>;
impl Gpio1cPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cPosIntEn> {
        match self.bits {
            0 => Some(Gpio1cPosIntEn::B0),
            1 => Some(Gpio1cPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cPosIntEn::B1
    }
}
#[doc = "Field `GPIO1C_POS_INT_EN` writer - gpio1c posedge pulse interrupt enable"]
pub type Gpio1cPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cPosIntEn>;
impl<'a, REG> Gpio1cPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cPosIntEn::B1)
    }
}
#[doc = "gpio1d posedge pulse interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1dPosIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1dPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1dPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1D_POS_INT_EN` reader - gpio1d posedge pulse interrupt enable"]
pub type Gpio1dPosIntEnR = crate::FieldReader<Gpio1dPosIntEn>;
impl Gpio1dPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dPosIntEn> {
        match self.bits {
            0 => Some(Gpio1dPosIntEn::B0),
            1 => Some(Gpio1dPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1dPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1dPosIntEn::B1
    }
}
#[doc = "Field `GPIO1D_POS_INT_EN` writer - gpio1d posedge pulse interrupt enable"]
pub type Gpio1dPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1dPosIntEn>;
impl<'a, REG> Gpio1dPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dPosIntEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio1a posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1a_pos_int_en(&self) -> Gpio1aPosIntEnR {
        Gpio1aPosIntEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio1b posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1b_pos_int_en(&self) -> Gpio1bPosIntEnR {
        Gpio1bPosIntEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio1c posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1c_pos_int_en(&self) -> Gpio1cPosIntEnR {
        Gpio1cPosIntEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio1d posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1d_pos_int_en(&self) -> Gpio1dPosIntEnR {
        Gpio1dPosIntEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio1a posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_pos_int_en(&mut self) -> Gpio1aPosIntEnW<Gpio1PosIntConSpec> {
        Gpio1aPosIntEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio1b posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_pos_int_en(&mut self) -> Gpio1bPosIntEnW<Gpio1PosIntConSpec> {
        Gpio1bPosIntEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio1c posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_pos_int_en(&mut self) -> Gpio1cPosIntEnW<Gpio1PosIntConSpec> {
        Gpio1cPosIntEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio1d posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_pos_int_en(&mut self) -> Gpio1dPosIntEnW<Gpio1PosIntConSpec> {
        Gpio1dPosIntEnW::new(self, 24)
    }
}
#[doc = "pmu gpio1 posedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_pos_int_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_pos_int_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1PosIntConSpec;
impl crate::RegisterSpec for Gpio1PosIntConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1_pos_int_con::R`](R) reader structure"]
impl crate::Readable for Gpio1PosIntConSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1_pos_int_con::W`](W) writer structure"]
impl crate::Writable for Gpio1PosIntConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO1_POS_INT_CON to value 0"]
impl crate::Resettable for Gpio1PosIntConSpec {
    const RESET_VALUE: u32 = 0;
}
