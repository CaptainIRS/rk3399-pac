#[doc = "Register `GPIO_INTTYPE_LEVEL` reader"]
pub type R = crate::R<GpioInttypeLevelSpec>;
#[doc = "Register `GPIO_INTTYPE_LEVEL` writer"]
pub type W = crate::W<GpioInttypeLevelSpec>;
#[doc = "Controls the type of interrupt that can occur on Port A.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioInttypeLevel {
    #[doc = "0: Level-sensitive (default)"]
    B0 = 0,
    #[doc = "1: Edge-sensitive"]
    B1 = 1,
}
impl From<GpioInttypeLevel> for u32 {
    #[inline(always)]
    fn from(variant: GpioInttypeLevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioInttypeLevel {
    type Ux = u32;
}
#[doc = "Field `GPIO_INTTYPE_LEVEL` reader - Controls the type of interrupt that can occur on Port A."]
pub type GpioInttypeLevelR = crate::FieldReader<GpioInttypeLevel>;
impl GpioInttypeLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioInttypeLevel> {
        match self.bits {
            0 => Some(GpioInttypeLevel::B0),
            1 => Some(GpioInttypeLevel::B1),
            _ => None,
        }
    }
    #[doc = "Level-sensitive (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioInttypeLevel::B0
    }
    #[doc = "Edge-sensitive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioInttypeLevel::B1
    }
}
#[doc = "Field `GPIO_INTTYPE_LEVEL` writer - Controls the type of interrupt that can occur on Port A."]
pub type GpioInttypeLevelW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioInttypeLevel>;
impl<'a, REG> GpioInttypeLevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Level-sensitive (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInttypeLevel::B0)
    }
    #[doc = "Edge-sensitive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInttypeLevel::B1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls the type of interrupt that can occur on Port A."]
    #[inline(always)]
    pub fn gpio_inttype_level(&self) -> GpioInttypeLevelR {
        GpioInttypeLevelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the type of interrupt that can occur on Port A."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inttype_level(&mut self) -> GpioInttypeLevelW<GpioInttypeLevelSpec> {
        GpioInttypeLevelW::new(self, 0)
    }
}
#[doc = "Interrupt level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_inttype_level::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_inttype_level::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioInttypeLevelSpec;
impl crate::RegisterSpec for GpioInttypeLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_inttype_level::R`](R) reader structure"]
impl crate::Readable for GpioInttypeLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_inttype_level::W`](W) writer structure"]
impl crate::Writable for GpioInttypeLevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_INTTYPE_LEVEL to value 0"]
impl crate::Resettable for GpioInttypeLevelSpec {
    const RESET_VALUE: u32 = 0;
}
