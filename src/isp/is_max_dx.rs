#[doc = "Register `IS_MAX_DX` reader"]
pub type R = crate::R<IsMaxDxSpec>;
#[doc = "Register `IS_MAX_DX` writer"]
pub type W = crate::W<IsMaxDxSpec>;
#[doc = "Field `is_max_dx` reader - maximum allowed accumulated horizontal\n\ndisplacement in pixels\n\n"]
pub type IsMaxDxR = crate::FieldReader<u16>;
#[doc = "Field `is_max_dx` writer - maximum allowed accumulated horizontal\n\ndisplacement in pixels\n\n"]
pub type IsMaxDxW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - maximum allowed accumulated horizontal\n\ndisplacement in pixels\n\n"]
    #[inline(always)]
    pub fn is_max_dx(&self) -> IsMaxDxR {
        IsMaxDxR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - maximum allowed accumulated horizontal\n\ndisplacement in pixels\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn is_max_dx(&mut self) -> IsMaxDxW<IsMaxDxSpec> {
        IsMaxDxW::new(self, 0)
    }
}
#[doc = "Maximum Horizontal Displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_max_dx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_max_dx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsMaxDxSpec;
impl crate::RegisterSpec for IsMaxDxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_max_dx::R`](R) reader structure"]
impl crate::Readable for IsMaxDxSpec {}
#[doc = "`write(|w| ..)` method takes [`is_max_dx::W`](W) writer structure"]
impl crate::Writable for IsMaxDxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_MAX_DX to value 0"]
impl crate::Resettable for IsMaxDxSpec {
    const RESET_VALUE: u32 = 0;
}
