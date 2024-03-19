#[doc = "Register `PMU_GPIO0_NEG_INT_ST` reader"]
pub type R = crate::R<PmuGpio0NegIntStSpec>;
#[doc = "Register `PMU_GPIO0_NEG_INT_ST` writer"]
pub type W = crate::W<PmuGpio0NegIntStSpec>;
#[doc = "gpio0a negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aNegIntStatus {
    #[doc = "0: not wakeup by gpio0a negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0a negedge pulse"]
    B1 = 1,
}
impl From<Gpio0aNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_NEG_INT_STATUS` reader - gpio0a negedge pulse interrupt status"]
pub type Gpio0aNegIntStatusR = crate::FieldReader<Gpio0aNegIntStatus>;
impl Gpio0aNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aNegIntStatus> {
        match self.bits {
            0 => Some(Gpio0aNegIntStatus::B0),
            1 => Some(Gpio0aNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio0a negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aNegIntStatus::B0
    }
    #[doc = "wakeup by gpio0a negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aNegIntStatus::B1
    }
}
#[doc = "Field `GPIO0A_NEG_INT_STATUS` writer - gpio0a negedge pulse interrupt status"]
pub type Gpio0aNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aNegIntStatus>;
impl<'a, REG> Gpio0aNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio0a negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio0a negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aNegIntStatus::B1)
    }
}
#[doc = "gpio0b negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bNegIntStatus {
    #[doc = "0: not wakeup by gpio0b negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0b negedge pulse"]
    B1 = 1,
}
impl From<Gpio0bNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_NEG_INT_STATUS` reader - gpio0b negedge pulse interrupt status"]
pub type Gpio0bNegIntStatusR = crate::FieldReader<Gpio0bNegIntStatus>;
impl Gpio0bNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bNegIntStatus> {
        match self.bits {
            0 => Some(Gpio0bNegIntStatus::B0),
            1 => Some(Gpio0bNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio0b negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bNegIntStatus::B0
    }
    #[doc = "wakeup by gpio0b negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bNegIntStatus::B1
    }
}
#[doc = "Field `GPIO0B_NEG_INT_STATUS` writer - gpio0b negedge pulse interrupt status"]
pub type Gpio0bNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bNegIntStatus>;
impl<'a, REG> Gpio0bNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio0b negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio0b negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bNegIntStatus::B1)
    }
}
#[doc = "gpio0c negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0cNegIntStatus {
    #[doc = "0: not wakeup by gpio0c negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0c negedge pulse"]
    B1 = 1,
}
impl From<Gpio0cNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0cNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0cNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0C_NEG_INT_STATUS` reader - gpio0c negedge pulse interrupt status"]
pub type Gpio0cNegIntStatusR = crate::FieldReader<Gpio0cNegIntStatus>;
impl Gpio0cNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0cNegIntStatus> {
        match self.bits {
            0 => Some(Gpio0cNegIntStatus::B0),
            1 => Some(Gpio0cNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio0c negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0cNegIntStatus::B0
    }
    #[doc = "wakeup by gpio0c negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0cNegIntStatus::B1
    }
}
#[doc = "Field `GPIO0C_NEG_INT_STATUS` writer - gpio0c negedge pulse interrupt status"]
pub type Gpio0cNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0cNegIntStatus>;
impl<'a, REG> Gpio0cNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio0c negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio0c negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cNegIntStatus::B1)
    }
}
#[doc = "gpio0d negedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dNegIntStatus {
    #[doc = "0: not wakeup by gpio0d negedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0d negedge pulse"]
    B1 = 1,
}
impl From<Gpio0dNegIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dNegIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dNegIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_NEG_INT_STATUS` reader - gpio0d negedge pulse interrupt status"]
pub type Gpio0dNegIntStatusR = crate::FieldReader<Gpio0dNegIntStatus>;
impl Gpio0dNegIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dNegIntStatus> {
        match self.bits {
            0 => Some(Gpio0dNegIntStatus::B0),
            1 => Some(Gpio0dNegIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio0d negedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dNegIntStatus::B0
    }
    #[doc = "wakeup by gpio0d negedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dNegIntStatus::B1
    }
}
#[doc = "Field `GPIO0D_NEG_INT_STATUS` writer - gpio0d negedge pulse interrupt status"]
pub type Gpio0dNegIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dNegIntStatus>;
impl<'a, REG> Gpio0dNegIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio0d negedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dNegIntStatus::B0)
    }
    #[doc = "wakeup by gpio0d negedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dNegIntStatus::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio0a negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0a_neg_int_status(&self) -> Gpio0aNegIntStatusR {
        Gpio0aNegIntStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio0b negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0b_neg_int_status(&self) -> Gpio0bNegIntStatusR {
        Gpio0bNegIntStatusR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio0c negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0c_neg_int_status(&self) -> Gpio0cNegIntStatusR {
        Gpio0cNegIntStatusR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio0d negedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0d_neg_int_status(&self) -> Gpio0dNegIntStatusR {
        Gpio0dNegIntStatusR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio0a negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_neg_int_status(&mut self) -> Gpio0aNegIntStatusW<PmuGpio0NegIntStSpec> {
        Gpio0aNegIntStatusW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio0b negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_neg_int_status(&mut self) -> Gpio0bNegIntStatusW<PmuGpio0NegIntStSpec> {
        Gpio0bNegIntStatusW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio0c negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0c_neg_int_status(&mut self) -> Gpio0cNegIntStatusW<PmuGpio0NegIntStSpec> {
        Gpio0cNegIntStatusW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio0d negedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_neg_int_status(&mut self) -> Gpio0dNegIntStatusW<PmuGpio0NegIntStSpec> {
        Gpio0dNegIntStatusW::new(self, 24)
    }
}
#[doc = "pmu gpio0 negedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_neg_int_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_neg_int_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuGpio0NegIntStSpec;
impl crate::RegisterSpec for PmuGpio0NegIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_gpio0_neg_int_st::R`](R) reader structure"]
impl crate::Readable for PmuGpio0NegIntStSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_gpio0_neg_int_st::W`](W) writer structure"]
impl crate::Writable for PmuGpio0NegIntStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_GPIO0_NEG_INT_ST to value 0"]
impl crate::Resettable for PmuGpio0NegIntStSpec {
    const RESET_VALUE: u32 = 0;
}
