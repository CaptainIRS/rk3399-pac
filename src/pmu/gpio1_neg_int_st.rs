#[doc = "Register `GPIO1_NEG_INT_ST` reader"]
pub type R = crate::R<Gpio1NegIntStSpec>;
#[doc = "Register `GPIO1_NEG_INT_ST` writer"]
pub type W = crate::W<Gpio1NegIntStSpec>;
#[doc = "gpio1a negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aNegIntStatus {
    #[doc = "0: not wakeup by gpio1a negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1a negedge pulse"]
    B1 = 1,
}
impl From<Gpio1aNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_NEG_INT_STATUS` reader - gpio1a negedge pulse interrupt status"]
pub type Gpio1aNegIntStatusR = crate::FieldReader<Gpio1aNegIntStatus>;
impl Gpio1aNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aNegIntStatus> {
        match self.bits {
            0 => Some(Gpio1aNegIntStatus::B0),
            1 => Some(Gpio1aNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1a negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aNegIntStatus::B0
    }
    #[doc = "wakeup by gpio1a negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aNegIntStatus::B1
    }
}
#[doc = "Field `GPIO1A_NEG_INT_STATUS` writer - gpio1a negedge pulse interrupt status"]
pub type Gpio1aNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aNegIntStatus>;
impl<'a, REG> Gpio1aNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1a negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio1a negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aNegIntStatus::B1)
    }
}
#[doc = "gpio1b negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bNegIntStatus {
    #[doc = "0: not wakeup by gpio1b negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1b negedge pulse"]
    B1 = 1,
}
impl From<Gpio1bNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_NEG_INT_STATUS` reader - gpio1b negedge pulse interrupt status"]
pub type Gpio1bNegIntStatusR = crate::FieldReader<Gpio1bNegIntStatus>;
impl Gpio1bNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bNegIntStatus> {
        match self.bits {
            0 => Some(Gpio1bNegIntStatus::B0),
            1 => Some(Gpio1bNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1b negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bNegIntStatus::B0
    }
    #[doc = "wakeup by gpio1b negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bNegIntStatus::B1
    }
}
#[doc = "Field `GPIO1B_NEG_INT_STATUS` writer - gpio1b negedge pulse interrupt status"]
pub type Gpio1bNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bNegIntStatus>;
impl<'a, REG> Gpio1bNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1b negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio1b negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bNegIntStatus::B1)
    }
}
#[doc = "gpio1c negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cNegIntStatus {
    #[doc = "0: not wakeup by gpio1c negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1c negedge pulse"]
    B1 = 1,
}
impl From<Gpio1cNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_NEG_INT_STATUS` reader - gpio1c negedge pulse interrupt status"]
pub type Gpio1cNegIntStatusR = crate::FieldReader<Gpio1cNegIntStatus>;
impl Gpio1cNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cNegIntStatus> {
        match self.bits {
            0 => Some(Gpio1cNegIntStatus::B0),
            1 => Some(Gpio1cNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1c negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cNegIntStatus::B0
    }
    #[doc = "wakeup by gpio1c negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cNegIntStatus::B1
    }
}
#[doc = "Field `GPIO1C_NEG_INT_STATUS` writer - gpio1c negedge pulse interrupt status"]
pub type Gpio1cNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cNegIntStatus>;
impl<'a, REG> Gpio1cNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1c negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio1c negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cNegIntStatus::B1)
    }
}
#[doc = "gpio1d negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1dNegIntStatus {
    #[doc = "0: not wakeup by gpio1d negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1d negedge pulse"]
    B1 = 1,
}
impl From<Gpio1dNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1dNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1D_NEG_INT_STATUS` reader - gpio1d negedge pulse interrupt status"]
pub type Gpio1dNegIntStatusR = crate::FieldReader<Gpio1dNegIntStatus>;
impl Gpio1dNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dNegIntStatus> {
        match self.bits {
            0 => Some(Gpio1dNegIntStatus::B0),
            1 => Some(Gpio1dNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1d negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1dNegIntStatus::B0
    }
    #[doc = "wakeup by gpio1d negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1dNegIntStatus::B1
    }
}
#[doc = "Field `GPIO1D_NEG_INT_STATUS` writer - gpio1d negedge pulse interrupt status"]
pub type Gpio1dNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1dNegIntStatus>;
impl<'a, REG> Gpio1dNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1d negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio1d negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dNegIntStatus::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio1a negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1a_neg_int_status(&self) -> Gpio1aNegIntStatusR {
        Gpio1aNegIntStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio1b negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1b_neg_int_status(&self) -> Gpio1bNegIntStatusR {
        Gpio1bNegIntStatusR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio1c negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1c_neg_int_status(&self) -> Gpio1cNegIntStatusR {
        Gpio1cNegIntStatusR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio1d negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1d_neg_int_status(&self) -> Gpio1dNegIntStatusR {
        Gpio1dNegIntStatusR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio1a negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_neg_int_status(&mut self) -> Gpio1aNegIntStatusW<Gpio1NegIntStSpec> {
        Gpio1aNegIntStatusW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio1b negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_neg_int_status(&mut self) -> Gpio1bNegIntStatusW<Gpio1NegIntStSpec> {
        Gpio1bNegIntStatusW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio1c negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_neg_int_status(&mut self) -> Gpio1cNegIntStatusW<Gpio1NegIntStSpec> {
        Gpio1cNegIntStatusW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio1d negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_neg_int_status(&mut self) -> Gpio1dNegIntStatusW<Gpio1NegIntStSpec> {
        Gpio1dNegIntStatusW::new(self, 24)
    }
}
#[doc = "pmu gpio1 negedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_neg_int_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_neg_int_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1NegIntStSpec;
impl crate::RegisterSpec for Gpio1NegIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1_neg_int_st::R`](R) reader structure"]
impl crate::Readable for Gpio1NegIntStSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1_neg_int_st::W`](W) writer structure"]
impl crate::Writable for Gpio1NegIntStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO1_NEG_INT_ST to value 0"]
impl crate::Resettable for Gpio1NegIntStSpec {
    const RESET_VALUE: u32 = 0;
}
