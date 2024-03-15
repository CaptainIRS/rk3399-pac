#[doc = "Register `PMU_GPIO1_NEG_INT_CON` reader"]
pub type R = crate::R<PmuGpio1NegIntConSpec>;
#[doc = "Register `PMU_GPIO1_NEG_INT_CON` writer"]
pub type W = crate::W<PmuGpio1NegIntConSpec>;
#[doc = "gpio1a negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1aNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_NEG_INT_EN` reader - gpio1a negedge pulse interrupt enable"]
pub type Gpio1aNegIntEnR = crate::FieldReader<Gpio1aNegIntEn>;
impl Gpio1aNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aNegIntEn> {
        match self.bits {
            0 => Some(Gpio1aNegIntEn::B0),
            1 => Some(Gpio1aNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aNegIntEn::B1
    }
}
#[doc = "Field `GPIO1A_NEG_INT_EN` writer - gpio1a negedge pulse interrupt enable"]
pub type Gpio1aNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aNegIntEn>;
impl<'a, REG> Gpio1aNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aNegIntEn::B1)
    }
}
#[doc = "gpio1b negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1bNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_NEG_INT_EN` reader - gpio1b negedge pulse interrupt enable"]
pub type Gpio1bNegIntEnR = crate::FieldReader<Gpio1bNegIntEn>;
impl Gpio1bNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bNegIntEn> {
        match self.bits {
            0 => Some(Gpio1bNegIntEn::B0),
            1 => Some(Gpio1bNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bNegIntEn::B1
    }
}
#[doc = "Field `GPIO1B_NEG_INT_EN` writer - gpio1b negedge pulse interrupt enable"]
pub type Gpio1bNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bNegIntEn>;
impl<'a, REG> Gpio1bNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bNegIntEn::B1)
    }
}
#[doc = "gpio1c negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1cNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_NEG_INT_EN` reader - gpio1c negedge pulse interrupt enable"]
pub type Gpio1cNegIntEnR = crate::FieldReader<Gpio1cNegIntEn>;
impl Gpio1cNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cNegIntEn> {
        match self.bits {
            0 => Some(Gpio1cNegIntEn::B0),
            1 => Some(Gpio1cNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cNegIntEn::B1
    }
}
#[doc = "Field `GPIO1C_NEG_INT_EN` writer - gpio1c negedge pulse interrupt enable"]
pub type Gpio1cNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cNegIntEn>;
impl<'a, REG> Gpio1cNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cNegIntEn::B1)
    }
}
#[doc = "gpio1d negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1dNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio1dNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1dNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO1D_NEG_INT_EN` reader - gpio1d negedge pulse interrupt enable"]
pub type Gpio1dNegIntEnR = crate::FieldReader<Gpio1dNegIntEn>;
impl Gpio1dNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dNegIntEn> {
        match self.bits {
            0 => Some(Gpio1dNegIntEn::B0),
            1 => Some(Gpio1dNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1dNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1dNegIntEn::B1
    }
}
#[doc = "Field `GPIO1D_NEG_INT_EN` writer - gpio1d negedge pulse interrupt enable"]
pub type Gpio1dNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1dNegIntEn>;
impl<'a, REG> Gpio1dNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dNegIntEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio1a negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1a_neg_int_en(&self) -> Gpio1aNegIntEnR {
        Gpio1aNegIntEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio1b negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1b_neg_int_en(&self) -> Gpio1bNegIntEnR {
        Gpio1bNegIntEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio1c negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1c_neg_int_en(&self) -> Gpio1cNegIntEnR {
        Gpio1cNegIntEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio1d negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio1d_neg_int_en(&self) -> Gpio1dNegIntEnR {
        Gpio1dNegIntEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio1a negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_neg_int_en(&mut self) -> Gpio1aNegIntEnW<PmuGpio1NegIntConSpec> {
        Gpio1aNegIntEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio1b negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_neg_int_en(&mut self) -> Gpio1bNegIntEnW<PmuGpio1NegIntConSpec> {
        Gpio1bNegIntEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio1c negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_neg_int_en(&mut self) -> Gpio1cNegIntEnW<PmuGpio1NegIntConSpec> {
        Gpio1cNegIntEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio1d negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_neg_int_en(&mut self) -> Gpio1dNegIntEnW<PmuGpio1NegIntConSpec> {
        Gpio1dNegIntEnW::new(self, 24)
    }
}
#[doc = "pmu gpio1 negedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio1_neg_int_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio1_neg_int_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuGpio1NegIntConSpec;
impl crate::RegisterSpec for PmuGpio1NegIntConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_gpio1_neg_int_con::R`](R) reader structure"]
impl crate::Readable for PmuGpio1NegIntConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_gpio1_neg_int_con::W`](W) writer structure"]
impl crate::Writable for PmuGpio1NegIntConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_GPIO1_NEG_INT_CON to value 0"]
impl crate::Resettable for PmuGpio1NegIntConSpec {
    const RESET_VALUE: u32 = 0;
}
