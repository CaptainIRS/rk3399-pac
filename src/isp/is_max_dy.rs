#[doc = "Register `IS_MAX_DY` reader"]
pub type R = crate::R<IsMaxDySpec>;
#[doc = "Register `IS_MAX_DY` writer"]
pub type W = crate::W<IsMaxDySpec>;
#[doc = "Field `is_max_dy` reader - maximum allowed accumulated vertical displacement\n\nin lines\n\n"]
pub type IsMaxDyR = crate::FieldReader<u16>;
#[doc = "Field `is_max_dy` writer - maximum allowed accumulated vertical displacement\n\nin lines\n\n"]
pub type IsMaxDyW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - maximum allowed accumulated vertical displacement\n\nin lines\n\n"]
    #[inline(always)]
    pub fn is_max_dy(&self) -> IsMaxDyR {
        IsMaxDyR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - maximum allowed accumulated vertical displacement\n\nin lines\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn is_max_dy(&mut self) -> IsMaxDyW<IsMaxDySpec> {
        IsMaxDyW::new(self, 0)
    }
}
#[doc = "Maximum Vertical Displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_max_dy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_max_dy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsMaxDySpec;
impl crate::RegisterSpec for IsMaxDySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_max_dy::R`](R) reader structure"]
impl crate::Readable for IsMaxDySpec {}
#[doc = "`write(|w| ..)` method takes [`is_max_dy::W`](W) writer structure"]
impl crate::Writable for IsMaxDySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_MAX_DY to value 0"]
impl crate::Resettable for IsMaxDySpec {
    const RESET_VALUE: u32 = 0;
}
