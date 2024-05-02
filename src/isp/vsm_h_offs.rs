#[doc = "Register `VSM_H_OFFS` reader"]
pub type R = crate::R<VsmHOffsSpec>;
#[doc = "Register `VSM_H_OFFS` writer"]
pub type W = crate::W<VsmHOffsSpec>;
#[doc = "Field `vsm_h_offset` reader - Horizontal offset in pixels.\n\n"]
pub type VsmHOffsetR = crate::FieldReader<u16>;
#[doc = "Field `vsm_h_offset` writer - Horizontal offset in pixels.\n\n"]
pub type VsmHOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal offset in pixels.\n\n"]
    #[inline(always)]
    pub fn vsm_h_offset(&self) -> VsmHOffsetR {
        VsmHOffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal offset in pixels.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn vsm_h_offset(&mut self) -> VsmHOffsetW<VsmHOffsSpec> {
        VsmHOffsetW::new(self, 0)
    }
}
#[doc = "VSM window horizontal offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_h_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_h_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmHOffsSpec;
impl crate::RegisterSpec for VsmHOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_h_offs::R`](R) reader structure"]
impl crate::Readable for VsmHOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_h_offs::W`](W) writer structure"]
impl crate::Writable for VsmHOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_H_OFFS to value 0"]
impl crate::Resettable for VsmHOffsSpec {
    const RESET_VALUE: u32 = 0;
}
