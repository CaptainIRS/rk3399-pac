#[doc = "Register `GPIO_INTMASK` reader"]
pub type R = crate::R<GpioIntmaskSpec>;
#[doc = "Register `GPIO_INTMASK` writer"]
pub type W = crate::W<GpioIntmaskSpec>;
#[doc = "Controls whether an interrupt on Port A can create an\n\ninterrupt for the interrupt controller by not masking it. Whenever\n\na 1 is written to a bit in this register, it masks the interrupt\n\ngeneration capability for this signal; otherwise interrupts are\n\nallowed through.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntMask {
    #[doc = "0: Interrupt bits are unmasked (default)"]
    B0 = 0,
    #[doc = "1: Mask interrupt"]
    B1 = 1,
}
impl From<GpioIntMask> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntMask {
    type Ux = u32;
}
#[doc = "Field `GPIO_INT_MASK` reader - Controls whether an interrupt on Port A can create an\n\ninterrupt for the interrupt controller by not masking it. Whenever\n\na 1 is written to a bit in this register, it masks the interrupt\n\ngeneration capability for this signal; otherwise interrupts are\n\nallowed through."]
pub type GpioIntMaskR = crate::FieldReader<GpioIntMask>;
impl GpioIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntMask> {
        match self.bits {
            0 => Some(GpioIntMask::B0),
            1 => Some(GpioIntMask::B1),
            _ => None,
        }
    }
    #[doc = "Interrupt bits are unmasked (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioIntMask::B0
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioIntMask::B1
    }
}
#[doc = "Field `GPIO_INT_MASK` writer - Controls whether an interrupt on Port A can create an\n\ninterrupt for the interrupt controller by not masking it. Whenever\n\na 1 is written to a bit in this register, it masks the interrupt\n\ngeneration capability for this signal; otherwise interrupts are\n\nallowed through."]
pub type GpioIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntMask>;
impl<'a, REG> GpioIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupt bits are unmasked (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntMask::B0)
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntMask::B1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls whether an interrupt on Port A can create an\n\ninterrupt for the interrupt controller by not masking it. Whenever\n\na 1 is written to a bit in this register, it masks the interrupt\n\ngeneration capability for this signal; otherwise interrupts are\n\nallowed through."]
    #[inline(always)]
    pub fn gpio_int_mask(&self) -> GpioIntMaskR {
        GpioIntMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls whether an interrupt on Port A can create an\n\ninterrupt for the interrupt controller by not masking it. Whenever\n\na 1 is written to a bit in this register, it masks the interrupt\n\ngeneration capability for this signal; otherwise interrupts are\n\nallowed through."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_mask(&mut self) -> GpioIntMaskW<GpioIntmaskSpec> {
        GpioIntMaskW::new(self, 0)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_intmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_intmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntmaskSpec;
impl crate::RegisterSpec for GpioIntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_intmask::R`](R) reader structure"]
impl crate::Readable for GpioIntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_intmask::W`](W) writer structure"]
impl crate::Writable for GpioIntmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_INTMASK to value 0"]
impl crate::Resettable for GpioIntmaskSpec {
    const RESET_VALUE: u32 = 0;
}
