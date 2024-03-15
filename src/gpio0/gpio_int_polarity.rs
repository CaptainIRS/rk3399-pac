#[doc = "Register `GPIO_INT_POLARITY` reader"]
pub type R = crate::R<GpioIntPolaritySpec>;
#[doc = "Register `GPIO_INT_POLARITY` writer"]
pub type W = crate::W<GpioIntPolaritySpec>;
#[doc = "Controls the polarity of edge or level sensitivity that can occur on input of Port A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntPolarity {
    #[doc = "0: Active-high"]
    B0 = 0,
    #[doc = "1: Active-high"]
    B1 = 1,
}
impl From<GpioIntPolarity> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntPolarity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntPolarity {
    type Ux = u32;
}
#[doc = "Field `GPIO_INT_POLARITY` reader - Controls the polarity of edge or level sensitivity that can occur on input of Port A."]
pub type GpioIntPolarityR = crate::FieldReader<GpioIntPolarity>;
impl GpioIntPolarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntPolarity> {
        match self.bits {
            0 => Some(GpioIntPolarity::B0),
            1 => Some(GpioIntPolarity::B1),
            _ => None,
        }
    }
    #[doc = "Active-high"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioIntPolarity::B0
    }
    #[doc = "Active-high"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioIntPolarity::B1
    }
}
#[doc = "Field `GPIO_INT_POLARITY` writer - Controls the polarity of edge or level sensitivity that can occur on input of Port A."]
pub type GpioIntPolarityW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntPolarity>;
impl<'a, REG> GpioIntPolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Active-high"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntPolarity::B0)
    }
    #[doc = "Active-high"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntPolarity::B1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls the polarity of edge or level sensitivity that can occur on input of Port A."]
    #[inline(always)]
    pub fn gpio_int_polarity(&self) -> GpioIntPolarityR {
        GpioIntPolarityR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the polarity of edge or level sensitivity that can occur on input of Port A."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_polarity(&mut self) -> GpioIntPolarityW<GpioIntPolaritySpec> {
        GpioIntPolarityW::new(self, 0)
    }
}
#[doc = "Interrupt polarity register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int_polarity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_int_polarity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntPolaritySpec;
impl crate::RegisterSpec for GpioIntPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_polarity::R`](R) reader structure"]
impl crate::Readable for GpioIntPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_polarity::W`](W) writer structure"]
impl crate::Writable for GpioIntPolaritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_INT_POLARITY to value 0"]
impl crate::Resettable for GpioIntPolaritySpec {
    const RESET_VALUE: u32 = 0;
}
