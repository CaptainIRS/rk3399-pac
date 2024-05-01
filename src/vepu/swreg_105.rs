#[doc = "Register `SWREG_105` reader"]
pub type R = crate::R<Swreg105Spec>;
#[doc = "Register `SWREG_105` writer"]
pub type W = crate::W<Swreg105Spec>;
#[doc = "Field `TEST_LEN` reader - test data length"]
pub type TestLenR = crate::FieldReader<u32>;
#[doc = "Field `TEST_LEN` writer - test data length"]
pub type TestLenW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `COHER_TEST_MEM` reader - test memory coherency\n\ntest memory coherency"]
pub type CoherTestMemR = crate::BitReader;
#[doc = "Field `COHER_TEST_MEM` writer - test memory coherency\n\ntest memory coherency"]
pub type CoherTestMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COHER_TEST_REG` reader - test register coherency\n\ntest register coherency"]
pub type CoherTestRegR = crate::BitReader;
#[doc = "Field `COHER_TEST_REG` writer - test register coherency\n\ntest register coherency"]
pub type CoherTestRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_COUNTER` reader - test counter\n\ntest counter"]
pub type TestCounterR = crate::FieldReader;
#[doc = "Field `TEST_COUNTER` writer - test counter\n\ntest counter"]
pub type TestCounterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TEST_IRQ` reader - test irq"]
pub type TestIrqR = crate::BitReader;
#[doc = "Field `TEST_IRQ` writer - test irq"]
pub type TestIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "output swap 32-bits or not flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap32Out {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap 32bit"]
    B1 = 1,
}
impl From<Swap32Out> for bool {
    #[inline(always)]
    fn from(variant: Swap32Out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP32_OUT` reader - output swap 32-bits or not flag"]
pub type Swap32OutR = crate::BitReader<Swap32Out>;
impl Swap32OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap32Out {
        match self.bits {
            false => Swap32Out::B0,
            true => Swap32Out::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swap32Out::B0
    }
    #[doc = "swap 32bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swap32Out::B1
    }
}
#[doc = "Field `SWAP32_OUT` writer - output swap 32-bits or not flag"]
pub type Swap32OutW<'a, REG> = crate::BitWriter<'a, REG, Swap32Out>;
impl<'a, REG> Swap32OutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap32Out::B0)
    }
    #[doc = "swap 32bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap32Out::B1)
    }
}
#[doc = "output swap 16-bits or not flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap16Out {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap 16bit"]
    B1 = 1,
}
impl From<Swap16Out> for bool {
    #[inline(always)]
    fn from(variant: Swap16Out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP16_OUT` reader - output swap 16-bits or not flag"]
pub type Swap16OutR = crate::BitReader<Swap16Out>;
impl Swap16OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap16Out {
        match self.bits {
            false => Swap16Out::B0,
            true => Swap16Out::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swap16Out::B0
    }
    #[doc = "swap 16bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swap16Out::B1
    }
}
#[doc = "Field `SWAP16_OUT` writer - output swap 16-bits or not flag"]
pub type Swap16OutW<'a, REG> = crate::BitWriter<'a, REG, Swap16Out>;
impl<'a, REG> Swap16OutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap16Out::B0)
    }
    #[doc = "swap 16bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap16Out::B1)
    }
}
#[doc = "output swap 8-bits or not flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap8Out {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap 8bit"]
    B1 = 1,
}
impl From<Swap8Out> for bool {
    #[inline(always)]
    fn from(variant: Swap8Out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP8_OUT` reader - output swap 8-bits or not flag"]
pub type Swap8OutR = crate::BitReader<Swap8Out>;
impl Swap8OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap8Out {
        match self.bits {
            false => Swap8Out::B0,
            true => Swap8Out::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swap8Out::B0
    }
    #[doc = "swap 8bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swap8Out::B1
    }
}
#[doc = "Field `SWAP8_OUT` writer - output swap 8-bits or not flag"]
pub type Swap8OutW<'a, REG> = crate::BitWriter<'a, REG, Swap8Out>;
impl<'a, REG> Swap8OutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap8Out::B0)
    }
    #[doc = "swap 8bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap8Out::B1)
    }
}
#[doc = "input swap 32-bits or not flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap32In {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap 32bit"]
    B1 = 1,
}
impl From<Swap32In> for bool {
    #[inline(always)]
    fn from(variant: Swap32In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP32_IN` reader - input swap 32-bits or not flag"]
pub type Swap32InR = crate::BitReader<Swap32In>;
impl Swap32InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap32In {
        match self.bits {
            false => Swap32In::B0,
            true => Swap32In::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swap32In::B0
    }
    #[doc = "swap 32bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swap32In::B1
    }
}
#[doc = "Field `SWAP32_IN` writer - input swap 32-bits or not flag"]
pub type Swap32InW<'a, REG> = crate::BitWriter<'a, REG, Swap32In>;
impl<'a, REG> Swap32InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap32In::B0)
    }
    #[doc = "swap 32bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap32In::B1)
    }
}
#[doc = "input swap 16-bits or not flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap16In {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap 16bit"]
    B1 = 1,
}
impl From<Swap16In> for bool {
    #[inline(always)]
    fn from(variant: Swap16In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP16_IN` reader - input swap 16-bits or not flag"]
pub type Swap16InR = crate::BitReader<Swap16In>;
impl Swap16InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap16In {
        match self.bits {
            false => Swap16In::B0,
            true => Swap16In::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swap16In::B0
    }
    #[doc = "swap 16bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swap16In::B1
    }
}
#[doc = "Field `SWAP16_IN` writer - input swap 16-bits or not flag"]
pub type Swap16InW<'a, REG> = crate::BitWriter<'a, REG, Swap16In>;
impl<'a, REG> Swap16InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap16In::B0)
    }
    #[doc = "swap 16bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap16In::B1)
    }
}
#[doc = "input swap 8-bits or not flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap8In {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap 8bit"]
    B1 = 1,
}
impl From<Swap8In> for bool {
    #[inline(always)]
    fn from(variant: Swap8In) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP8_IN` reader - input swap 8-bits or not flag"]
pub type Swap8InR = crate::BitReader<Swap8In>;
impl Swap8InR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap8In {
        match self.bits {
            false => Swap8In::B0,
            true => Swap8In::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swap8In::B0
    }
    #[doc = "swap 8bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swap8In::B1
    }
}
#[doc = "Field `SWAP8_IN` writer - input swap 8-bits or not flag"]
pub type Swap8InW<'a, REG> = crate::BitWriter<'a, REG, Swap8In>;
impl<'a, REG> Swap8InW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swap8In::B0)
    }
    #[doc = "swap 8bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swap8In::B1)
    }
}
impl R {
    #[doc = "Bits 0:17 - test data length"]
    #[inline(always)]
    pub fn test_len(&self) -> TestLenR {
        TestLenR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 18 - test memory coherency\n\ntest memory coherency"]
    #[inline(always)]
    pub fn coher_test_mem(&self) -> CoherTestMemR {
        CoherTestMemR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - test register coherency\n\ntest register coherency"]
    #[inline(always)]
    pub fn coher_test_reg(&self) -> CoherTestRegR {
        CoherTestRegR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - test counter\n\ntest counter"]
    #[inline(always)]
    pub fn test_counter(&self) -> TestCounterR {
        TestCounterR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - test irq"]
    #[inline(always)]
    pub fn test_irq(&self) -> TestIrqR {
        TestIrqR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - output swap 32-bits or not flag"]
    #[inline(always)]
    pub fn swap32_out(&self) -> Swap32OutR {
        Swap32OutR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - output swap 16-bits or not flag"]
    #[inline(always)]
    pub fn swap16_out(&self) -> Swap16OutR {
        Swap16OutR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - output swap 8-bits or not flag"]
    #[inline(always)]
    pub fn swap8_out(&self) -> Swap8OutR {
        Swap8OutR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - input swap 32-bits or not flag"]
    #[inline(always)]
    pub fn swap32_in(&self) -> Swap32InR {
        Swap32InR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - input swap 16-bits or not flag"]
    #[inline(always)]
    pub fn swap16_in(&self) -> Swap16InR {
        Swap16InR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - input swap 8-bits or not flag"]
    #[inline(always)]
    pub fn swap8_in(&self) -> Swap8InR {
        Swap8InR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - test data length"]
    #[inline(always)]
    #[must_use]
    pub fn test_len(&mut self) -> TestLenW<Swreg105Spec> {
        TestLenW::new(self, 0)
    }
    #[doc = "Bit 18 - test memory coherency\n\ntest memory coherency"]
    #[inline(always)]
    #[must_use]
    pub fn coher_test_mem(&mut self) -> CoherTestMemW<Swreg105Spec> {
        CoherTestMemW::new(self, 18)
    }
    #[doc = "Bit 19 - test register coherency\n\ntest register coherency"]
    #[inline(always)]
    #[must_use]
    pub fn coher_test_reg(&mut self) -> CoherTestRegW<Swreg105Spec> {
        CoherTestRegW::new(self, 19)
    }
    #[doc = "Bits 20:23 - test counter\n\ntest counter"]
    #[inline(always)]
    #[must_use]
    pub fn test_counter(&mut self) -> TestCounterW<Swreg105Spec> {
        TestCounterW::new(self, 20)
    }
    #[doc = "Bit 24 - test irq"]
    #[inline(always)]
    #[must_use]
    pub fn test_irq(&mut self) -> TestIrqW<Swreg105Spec> {
        TestIrqW::new(self, 24)
    }
    #[doc = "Bit 26 - output swap 32-bits or not flag"]
    #[inline(always)]
    #[must_use]
    pub fn swap32_out(&mut self) -> Swap32OutW<Swreg105Spec> {
        Swap32OutW::new(self, 26)
    }
    #[doc = "Bit 27 - output swap 16-bits or not flag"]
    #[inline(always)]
    #[must_use]
    pub fn swap16_out(&mut self) -> Swap16OutW<Swreg105Spec> {
        Swap16OutW::new(self, 27)
    }
    #[doc = "Bit 28 - output swap 8-bits or not flag"]
    #[inline(always)]
    #[must_use]
    pub fn swap8_out(&mut self) -> Swap8OutW<Swreg105Spec> {
        Swap8OutW::new(self, 28)
    }
    #[doc = "Bit 29 - input swap 32-bits or not flag"]
    #[inline(always)]
    #[must_use]
    pub fn swap32_in(&mut self) -> Swap32InW<Swreg105Spec> {
        Swap32InW::new(self, 29)
    }
    #[doc = "Bit 30 - input swap 16-bits or not flag"]
    #[inline(always)]
    #[must_use]
    pub fn swap16_in(&mut self) -> Swap16InW<Swreg105Spec> {
        Swap16InW::new(self, 30)
    }
    #[doc = "Bit 31 - input swap 8-bits or not flag"]
    #[inline(always)]
    #[must_use]
    pub fn swap8_in(&mut self) -> Swap8InW<Swreg105Spec> {
        Swap8InW::new(self, 31)
    }
}
#[doc = "SWAP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_105::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_105::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg105Spec;
impl crate::RegisterSpec for Swreg105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_105::R`](R) reader structure"]
impl crate::Readable for Swreg105Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_105::W`](W) writer structure"]
impl crate::Writable for Swreg105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_105 to value 0"]
impl crate::Resettable for Swreg105Spec {
    const RESET_VALUE: u32 = 0;
}
