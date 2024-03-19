#[doc = "Register `RKI2C_MRXRADDR` reader"]
pub type R = crate::R<Rki2cMrxraddrSpec>;
#[doc = "Register `RKI2C_MRXRADDR` writer"]
pub type W = crate::W<Rki2cMrxraddrSpec>;
#[doc = "Field `SRADDR` reader - slave register address accessed\n\n24 bits register address"]
pub type SraddrR = crate::FieldReader<u32>;
#[doc = "Field `SRADDR` writer - slave register address accessed\n\n24 bits register address"]
pub type SraddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "address low byte valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sraddlvld {
    #[doc = "0: invalid"]
    B0 = 0,
    #[doc = "1: valid"]
    B1 = 1,
}
impl From<Sraddlvld> for bool {
    #[inline(always)]
    fn from(variant: Sraddlvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRADDLVLD` reader - address low byte valid"]
pub type SraddlvldR = crate::BitReader<Sraddlvld>;
impl SraddlvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sraddlvld {
        match self.bits {
            false => Sraddlvld::B0,
            true => Sraddlvld::B1,
        }
    }
    #[doc = "invalid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sraddlvld::B0
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sraddlvld::B1
    }
}
#[doc = "Field `SRADDLVLD` writer - address low byte valid"]
pub type SraddlvldW<'a, REG> = crate::BitWriter<'a, REG, Sraddlvld>;
impl<'a, REG> SraddlvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invalid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sraddlvld::B0)
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sraddlvld::B1)
    }
}
#[doc = "address middle byte valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sraddmvld {
    #[doc = "0: invalid"]
    B0 = 0,
    #[doc = "1: valid"]
    B1 = 1,
}
impl From<Sraddmvld> for bool {
    #[inline(always)]
    fn from(variant: Sraddmvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRADDMVLD` reader - address middle byte valid"]
pub type SraddmvldR = crate::BitReader<Sraddmvld>;
impl SraddmvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sraddmvld {
        match self.bits {
            false => Sraddmvld::B0,
            true => Sraddmvld::B1,
        }
    }
    #[doc = "invalid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sraddmvld::B0
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sraddmvld::B1
    }
}
#[doc = "Field `SRADDMVLD` writer - address middle byte valid"]
pub type SraddmvldW<'a, REG> = crate::BitWriter<'a, REG, Sraddmvld>;
impl<'a, REG> SraddmvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invalid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sraddmvld::B0)
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sraddmvld::B1)
    }
}
#[doc = "address high byte valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sraddhvld {
    #[doc = "0: invalid"]
    B0 = 0,
    #[doc = "1: valid"]
    B1 = 1,
}
impl From<Sraddhvld> for bool {
    #[inline(always)]
    fn from(variant: Sraddhvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRADDHVLD` reader - address high byte valid"]
pub type SraddhvldR = crate::BitReader<Sraddhvld>;
impl SraddhvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sraddhvld {
        match self.bits {
            false => Sraddhvld::B0,
            true => Sraddhvld::B1,
        }
    }
    #[doc = "invalid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sraddhvld::B0
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sraddhvld::B1
    }
}
#[doc = "Field `SRADDHVLD` writer - address high byte valid"]
pub type SraddhvldW<'a, REG> = crate::BitWriter<'a, REG, Sraddhvld>;
impl<'a, REG> SraddhvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invalid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sraddhvld::B0)
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sraddhvld::B1)
    }
}
impl R {
    #[doc = "Bits 0:23 - slave register address accessed\n\n24 bits register address"]
    #[inline(always)]
    pub fn sraddr(&self) -> SraddrR {
        SraddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - address low byte valid"]
    #[inline(always)]
    pub fn sraddlvld(&self) -> SraddlvldR {
        SraddlvldR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - address middle byte valid"]
    #[inline(always)]
    pub fn sraddmvld(&self) -> SraddmvldR {
        SraddmvldR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - address high byte valid"]
    #[inline(always)]
    pub fn sraddhvld(&self) -> SraddhvldR {
        SraddhvldR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - slave register address accessed\n\n24 bits register address"]
    #[inline(always)]
    #[must_use]
    pub fn sraddr(&mut self) -> SraddrW<Rki2cMrxraddrSpec> {
        SraddrW::new(self, 0)
    }
    #[doc = "Bit 24 - address low byte valid"]
    #[inline(always)]
    #[must_use]
    pub fn sraddlvld(&mut self) -> SraddlvldW<Rki2cMrxraddrSpec> {
        SraddlvldW::new(self, 24)
    }
    #[doc = "Bit 25 - address middle byte valid"]
    #[inline(always)]
    #[must_use]
    pub fn sraddmvld(&mut self) -> SraddmvldW<Rki2cMrxraddrSpec> {
        SraddmvldW::new(self, 25)
    }
    #[doc = "Bit 26 - address high byte valid"]
    #[inline(always)]
    #[must_use]
    pub fn sraddhvld(&mut self) -> SraddhvldW<Rki2cMrxraddrSpec> {
        SraddhvldW::new(self, 26)
    }
}
#[doc = "the slave register address accessed for master rx mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_mrxraddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_mrxraddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cMrxraddrSpec;
impl crate::RegisterSpec for Rki2cMrxraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_mrxraddr::R`](R) reader structure"]
impl crate::Readable for Rki2cMrxraddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_mrxraddr::W`](W) writer structure"]
impl crate::Writable for Rki2cMrxraddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_MRXRADDR to value 0"]
impl crate::Resettable for Rki2cMrxraddrSpec {
    const RESET_VALUE: u32 = 0;
}
