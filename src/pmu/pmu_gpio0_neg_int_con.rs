#[doc = "Register `PMU_GPIO0_NEG_INT_CON` reader"]
pub type R = crate::R<PmuGpio0NegIntConSpec>;
#[doc = "Register `PMU_GPIO0_NEG_INT_CON` writer"]
pub type W = crate::W<PmuGpio0NegIntConSpec>;
#[doc = "gpio0a negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0aNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_NEG_INT_EN` reader - gpio0a negedge pulse interrupt enable"]
pub type Gpio0aNegIntEnR = crate::FieldReader<Gpio0aNegIntEn>;
impl Gpio0aNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aNegIntEn> {
        match self.bits {
            0 => Some(Gpio0aNegIntEn::B0),
            1 => Some(Gpio0aNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aNegIntEn::B1
    }
}
#[doc = "Field `GPIO0A_NEG_INT_EN` writer - gpio0a negedge pulse interrupt enable"]
pub type Gpio0aNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aNegIntEn>;
impl<'a, REG> Gpio0aNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aNegIntEn::B1)
    }
}
#[doc = "gpio0b negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0bNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_NEG_INT_EN` reader - gpio0b negedge pulse interrupt enable"]
pub type Gpio0bNegIntEnR = crate::FieldReader<Gpio0bNegIntEn>;
impl Gpio0bNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bNegIntEn> {
        match self.bits {
            0 => Some(Gpio0bNegIntEn::B0),
            1 => Some(Gpio0bNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bNegIntEn::B1
    }
}
#[doc = "Field `GPIO0B_NEG_INT_EN` writer - gpio0b negedge pulse interrupt enable"]
pub type Gpio0bNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bNegIntEn>;
impl<'a, REG> Gpio0bNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bNegIntEn::B1)
    }
}
#[doc = "gpio0c negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0cNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0cNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0cNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0cNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0C_NEG_INT_EN` reader - gpio0c negedge pulse interrupt enable"]
pub type Gpio0cNegIntEnR = crate::FieldReader<Gpio0cNegIntEn>;
impl Gpio0cNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0cNegIntEn> {
        match self.bits {
            0 => Some(Gpio0cNegIntEn::B0),
            1 => Some(Gpio0cNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0cNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0cNegIntEn::B1
    }
}
#[doc = "Field `GPIO0C_NEG_INT_EN` writer - gpio0c negedge pulse interrupt enable"]
pub type Gpio0cNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0cNegIntEn>;
impl<'a, REG> Gpio0cNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cNegIntEn::B1)
    }
}
#[doc = "gpio0d negedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dNegIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0dNegIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dNegIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dNegIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_NEG_INT_EN` reader - gpio0d negedge pulse interrupt enable"]
pub type Gpio0dNegIntEnR = crate::FieldReader<Gpio0dNegIntEn>;
impl Gpio0dNegIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dNegIntEn> {
        match self.bits {
            0 => Some(Gpio0dNegIntEn::B0),
            1 => Some(Gpio0dNegIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dNegIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dNegIntEn::B1
    }
}
#[doc = "Field `GPIO0D_NEG_INT_EN` writer - gpio0d negedge pulse interrupt enable"]
pub type Gpio0dNegIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dNegIntEn>;
impl<'a, REG> Gpio0dNegIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dNegIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dNegIntEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio0a negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0a_neg_int_en(&self) -> Gpio0aNegIntEnR {
        Gpio0aNegIntEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio0b negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0b_neg_int_en(&self) -> Gpio0bNegIntEnR {
        Gpio0bNegIntEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio0c negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0c_neg_int_en(&self) -> Gpio0cNegIntEnR {
        Gpio0cNegIntEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio0d negedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0d_neg_int_en(&self) -> Gpio0dNegIntEnR {
        Gpio0dNegIntEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio0a negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_neg_int_en(&mut self) -> Gpio0aNegIntEnW<PmuGpio0NegIntConSpec> {
        Gpio0aNegIntEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio0b negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_neg_int_en(&mut self) -> Gpio0bNegIntEnW<PmuGpio0NegIntConSpec> {
        Gpio0bNegIntEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio0c negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0c_neg_int_en(&mut self) -> Gpio0cNegIntEnW<PmuGpio0NegIntConSpec> {
        Gpio0cNegIntEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio0d negedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_neg_int_en(&mut self) -> Gpio0dNegIntEnW<PmuGpio0NegIntConSpec> {
        Gpio0dNegIntEnW::new(self, 24)
    }
}
#[doc = "pmu gpio0 negedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_neg_int_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_neg_int_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuGpio0NegIntConSpec;
impl crate::RegisterSpec for PmuGpio0NegIntConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_gpio0_neg_int_con::R`](R) reader structure"]
impl crate::Readable for PmuGpio0NegIntConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_gpio0_neg_int_con::W`](W) writer structure"]
impl crate::Writable for PmuGpio0NegIntConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_GPIO0_NEG_INT_CON to value 0"]
impl crate::Resettable for PmuGpio0NegIntConSpec {
    const RESET_VALUE: u32 = 0;
}
