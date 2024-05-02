#[doc = "Register `SUPER_IMP_OFFSET_Y` reader"]
pub type R = crate::R<SuperImpOffsetYSpec>;
#[doc = "Register `SUPER_IMP_OFFSET_Y` writer"]
pub type W = crate::W<SuperImpOffsetYSpec>;
#[doc = "Field `offset_y` reader - Offset Y"]
pub type OffsetYR = crate::FieldReader<u16>;
#[doc = "Field `offset_y` writer - Offset Y"]
pub type OffsetYW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Offset Y"]
    #[inline(always)]
    pub fn offset_y(&self) -> OffsetYR {
        OffsetYR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Offset Y"]
    #[inline(always)]
    #[must_use]
    pub fn offset_y(&mut self) -> OffsetYW<SuperImpOffsetYSpec> {
        OffsetYW::new(self, 0)
    }
}
#[doc = "Offset y register\n\nNote: the offset_y is positive and refers to the \n\n\n\nreference image \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`super_imp_offset_y::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`super_imp_offset_y::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SuperImpOffsetYSpec;
impl crate::RegisterSpec for SuperImpOffsetYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`super_imp_offset_y::R`](R) reader structure"]
impl crate::Readable for SuperImpOffsetYSpec {}
#[doc = "`write(|w| ..)` method takes [`super_imp_offset_y::W`](W) writer structure"]
impl crate::Writable for SuperImpOffsetYSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUPER_IMP_OFFSET_Y to value 0"]
impl crate::Resettable for SuperImpOffsetYSpec {
    const RESET_VALUE: u32 = 0;
}
