#[doc = "Register `VSM_V_OFFS` reader"]
pub type R = crate::R<VsmVOffsSpec>;
#[doc = "Register `VSM_V_OFFS` writer"]
pub type W = crate::W<VsmVOffsSpec>;
#[doc = "Field `vsm_v_offset` reader - Vertical offset in pixels.\n\n"]
pub type VsmVOffsetR = crate::FieldReader<u16>;
#[doc = "Field `vsm_v_offset` writer - Vertical offset in pixels.\n\n"]
pub type VsmVOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical offset in pixels.\n\n"]
    #[inline(always)]
    pub fn vsm_v_offset(&self) -> VsmVOffsetR {
        VsmVOffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical offset in pixels.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn vsm_v_offset(&mut self) -> VsmVOffsetW<VsmVOffsSpec> {
        VsmVOffsetW::new(self, 0)
    }
}
#[doc = "VSM window vertical offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_v_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_v_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmVOffsSpec;
impl crate::RegisterSpec for VsmVOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_v_offs::R`](R) reader structure"]
impl crate::Readable for VsmVOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_v_offs::W`](W) writer structure"]
impl crate::Writable for VsmVOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_V_OFFS to value 0"]
impl crate::Resettable for VsmVOffsSpec {
    const RESET_VALUE: u32 = 0;
}
