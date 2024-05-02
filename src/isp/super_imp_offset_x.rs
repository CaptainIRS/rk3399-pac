#[doc = "Register `SUPER_IMP_OFFSET_X` reader"]
pub type R = crate::R<SuperImpOffsetXSpec>;
#[doc = "Register `SUPER_IMP_OFFSET_X` writer"]
pub type W = crate::W<SuperImpOffsetXSpec>;
#[doc = "Field `offset_x` reader - Offset X\n\nnote: the bit 0 is don‟t care (write 1 doesn‟t have\n\nany effect, the read access always gives „0‟)\n\nnote: the offset_x is positive and refers to the reference image"]
pub type OffsetXR = crate::FieldReader<u16>;
#[doc = "Field `offset_x` writer - Offset X\n\nnote: the bit 0 is don‟t care (write 1 doesn‟t have\n\nany effect, the read access always gives „0‟)\n\nnote: the offset_x is positive and refers to the reference image"]
pub type OffsetXW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 1:13 - Offset X\n\nnote: the bit 0 is don‟t care (write 1 doesn‟t have\n\nany effect, the read access always gives „0‟)\n\nnote: the offset_x is positive and refers to the reference image"]
    #[inline(always)]
    pub fn offset_x(&self) -> OffsetXR {
        OffsetXR::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:13 - Offset X\n\nnote: the bit 0 is don‟t care (write 1 doesn‟t have\n\nany effect, the read access always gives „0‟)\n\nnote: the offset_x is positive and refers to the reference image"]
    #[inline(always)]
    #[must_use]
    pub fn offset_x(&mut self) -> OffsetXW<SuperImpOffsetXSpec> {
        OffsetXW::new(self, 1)
    }
}
#[doc = "Offset x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`super_imp_offset_x::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`super_imp_offset_x::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SuperImpOffsetXSpec;
impl crate::RegisterSpec for SuperImpOffsetXSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`super_imp_offset_x::R`](R) reader structure"]
impl crate::Readable for SuperImpOffsetXSpec {}
#[doc = "`write(|w| ..)` method takes [`super_imp_offset_x::W`](W) writer structure"]
impl crate::Writable for SuperImpOffsetXSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUPER_IMP_OFFSET_X to value 0"]
impl crate::Resettable for SuperImpOffsetXSpec {
    const RESET_VALUE: u32 = 0;
}
