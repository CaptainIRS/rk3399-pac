#[doc = "Register `GPIO_DEBOUNCE` reader"]
pub type R = crate::R<GpioDebounceSpec>;
#[doc = "Register `GPIO_DEBOUNCE` writer"]
pub type W = crate::W<GpioDebounceSpec>;
#[doc = "Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioDebounce {
    #[doc = "0: Enable debounce"]
    B0 = 0,
    #[doc = "1: Enable debounce"]
    B1 = 1,
}
impl From<GpioDebounce> for u32 {
    #[inline(always)]
    fn from(variant: GpioDebounce) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioDebounce {
    type Ux = u32;
}
#[doc = "Field `GPIO_DEBOUNCE` reader - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
pub type GpioDebounceR = crate::FieldReader<GpioDebounce>;
impl GpioDebounceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioDebounce> {
        match self.bits {
            0 => Some(GpioDebounce::B0),
            1 => Some(GpioDebounce::B1),
            _ => None,
        }
    }
    #[doc = "Enable debounce"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioDebounce::B0
    }
    #[doc = "Enable debounce"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioDebounce::B1
    }
}
#[doc = "Field `GPIO_DEBOUNCE` writer - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
pub type GpioDebounceW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioDebounce>;
impl<'a, REG> GpioDebounceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Enable debounce"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioDebounce::B0)
    }
    #[doc = "Enable debounce"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioDebounce::B1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
    #[inline(always)]
    pub fn gpio_debounce(&self) -> GpioDebounceR {
        GpioDebounceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_debounce(&mut self) -> GpioDebounceW<GpioDebounceSpec> {
        GpioDebounceW::new(self, 0)
    }
}
#[doc = "Debounce enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_debounce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_debounce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDebounceSpec;
impl crate::RegisterSpec for GpioDebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_debounce::R`](R) reader structure"]
impl crate::Readable for GpioDebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_debounce::W`](W) writer structure"]
impl crate::Writable for GpioDebounceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_DEBOUNCE to value 0"]
impl crate::Resettable for GpioDebounceSpec {
    const RESET_VALUE: u32 = 0;
}
