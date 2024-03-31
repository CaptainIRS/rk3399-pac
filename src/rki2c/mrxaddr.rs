#[doc = "Register `MRXADDR` reader"]
pub type R = crate::R<MrxaddrSpec>;
#[doc = "Register `MRXADDR` writer"]
pub type W = crate::W<MrxaddrSpec>;
#[doc = "Field `SADDR` reader - master address register\n\nthe lowest bit indicate write or read\n\n24 bits address register"]
pub type SaddrR = crate::FieldReader<u32>;
#[doc = "Field `SADDR` writer - master address register\n\nthe lowest bit indicate write or read\n\n24 bits address register"]
pub type SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "address low byte valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addlvld {
    #[doc = "0: invalid"]
    B0 = 0,
    #[doc = "1: valid"]
    B1 = 1,
}
impl From<Addlvld> for bool {
    #[inline(always)]
    fn from(variant: Addlvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDLVLD` reader - address low byte valid"]
pub type AddlvldR = crate::BitReader<Addlvld>;
impl AddlvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addlvld {
        match self.bits {
            false => Addlvld::B0,
            true => Addlvld::B1,
        }
    }
    #[doc = "invalid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Addlvld::B0
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Addlvld::B1
    }
}
#[doc = "Field `ADDLVLD` writer - address low byte valid"]
pub type AddlvldW<'a, REG> = crate::BitWriter<'a, REG, Addlvld>;
impl<'a, REG> AddlvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invalid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Addlvld::B0)
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Addlvld::B1)
    }
}
#[doc = "address middle byte valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addmvld {
    #[doc = "0: invalid"]
    B0 = 0,
    #[doc = "1: valid"]
    B1 = 1,
}
impl From<Addmvld> for bool {
    #[inline(always)]
    fn from(variant: Addmvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDMVLD` reader - address middle byte valid"]
pub type AddmvldR = crate::BitReader<Addmvld>;
impl AddmvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addmvld {
        match self.bits {
            false => Addmvld::B0,
            true => Addmvld::B1,
        }
    }
    #[doc = "invalid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Addmvld::B0
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Addmvld::B1
    }
}
#[doc = "Field `ADDMVLD` writer - address middle byte valid"]
pub type AddmvldW<'a, REG> = crate::BitWriter<'a, REG, Addmvld>;
impl<'a, REG> AddmvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invalid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Addmvld::B0)
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Addmvld::B1)
    }
}
#[doc = "address high byte valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addhvld {
    #[doc = "0: invalid"]
    B0 = 0,
    #[doc = "1: valid"]
    B1 = 1,
}
impl From<Addhvld> for bool {
    #[inline(always)]
    fn from(variant: Addhvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDHVLD` reader - address high byte valid"]
pub type AddhvldR = crate::BitReader<Addhvld>;
impl AddhvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addhvld {
        match self.bits {
            false => Addhvld::B0,
            true => Addhvld::B1,
        }
    }
    #[doc = "invalid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Addhvld::B0
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Addhvld::B1
    }
}
#[doc = "Field `ADDHVLD` writer - address high byte valid"]
pub type AddhvldW<'a, REG> = crate::BitWriter<'a, REG, Addhvld>;
impl<'a, REG> AddhvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invalid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Addhvld::B0)
    }
    #[doc = "valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Addhvld::B1)
    }
}
impl R {
    #[doc = "Bits 0:23 - master address register\n\nthe lowest bit indicate write or read\n\n24 bits address register"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - address low byte valid"]
    #[inline(always)]
    pub fn addlvld(&self) -> AddlvldR {
        AddlvldR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - address middle byte valid"]
    #[inline(always)]
    pub fn addmvld(&self) -> AddmvldR {
        AddmvldR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - address high byte valid"]
    #[inline(always)]
    pub fn addhvld(&self) -> AddhvldR {
        AddhvldR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - master address register\n\nthe lowest bit indicate write or read\n\n24 bits address register"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SaddrW<MrxaddrSpec> {
        SaddrW::new(self, 0)
    }
    #[doc = "Bit 24 - address low byte valid"]
    #[inline(always)]
    #[must_use]
    pub fn addlvld(&mut self) -> AddlvldW<MrxaddrSpec> {
        AddlvldW::new(self, 24)
    }
    #[doc = "Bit 25 - address middle byte valid"]
    #[inline(always)]
    #[must_use]
    pub fn addmvld(&mut self) -> AddmvldW<MrxaddrSpec> {
        AddmvldW::new(self, 25)
    }
    #[doc = "Bit 26 - address high byte valid"]
    #[inline(always)]
    #[must_use]
    pub fn addhvld(&mut self) -> AddhvldW<MrxaddrSpec> {
        AddhvldW::new(self, 26)
    }
}
#[doc = "the slave address accessed for master rx mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrxaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrxaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrxaddrSpec;
impl crate::RegisterSpec for MrxaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrxaddr::R`](R) reader structure"]
impl crate::Readable for MrxaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrxaddr::W`](W) writer structure"]
impl crate::Writable for MrxaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRXADDR to value 0"]
impl crate::Resettable for MrxaddrSpec {
    const RESET_VALUE: u32 = 0;
}
