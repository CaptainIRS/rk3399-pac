#[doc = "Register `GPIO_INTEN` reader"]
pub type R = crate::R<GpioIntenSpec>;
#[doc = "Register `GPIO_INTEN` writer"]
pub type W = crate::W<GpioIntenSpec>;
#[doc = "Allows each bit of Port A to be configured for interrupts. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntEn {
    #[doc = "0: Configure Port A bit as interrupt"]
    B0 = 0,
    #[doc = "1: Configure Port A bit as interrupt"]
    B1 = 1,
}
impl From<GpioIntEn> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntEn {
    type Ux = u32;
}
#[doc = "Field `GPIO_INT_EN` reader - Allows each bit of Port A to be configured for interrupts. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output."]
pub type GpioIntEnR = crate::FieldReader<GpioIntEn>;
impl GpioIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntEn> {
        match self.bits {
            0 => Some(GpioIntEn::B0),
            1 => Some(GpioIntEn::B1),
            _ => None,
        }
    }
    #[doc = "Configure Port A bit as interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioIntEn::B0
    }
    #[doc = "Configure Port A bit as interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioIntEn::B1
    }
}
#[doc = "Field `GPIO_INT_EN` writer - Allows each bit of Port A to be configured for interrupts. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output."]
pub type GpioIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntEn>;
impl<'a, REG> GpioIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Configure Port A bit as interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntEn::B0)
    }
    #[doc = "Configure Port A bit as interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Allows each bit of Port A to be configured for interrupts. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output."]
    #[inline(always)]
    pub fn gpio_int_en(&self) -> GpioIntEnR {
        GpioIntEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Allows each bit of Port A to be configured for interrupts. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_en(&mut self) -> GpioIntEnW<GpioIntenSpec> {
        GpioIntEnW::new(self, 0)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntenSpec;
impl crate::RegisterSpec for GpioIntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_inten::R`](R) reader structure"]
impl crate::Readable for GpioIntenSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_inten::W`](W) writer structure"]
impl crate::Writable for GpioIntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_INTEN to value 0"]
impl crate::Resettable for GpioIntenSpec {
    const RESET_VALUE: u32 = 0;
}
