#[doc = "Register `VSM_V_SIZE` reader"]
pub type R = crate::R<VsmVSizeSpec>;
#[doc = "Register `VSM_V_SIZE` writer"]
pub type W = crate::W<VsmVSizeSpec>;
#[doc = "Field `vsm_v_size` reader - Vertical size."]
pub type VsmVSizeR = crate::FieldReader<u16>;
#[doc = "Field `vsm_v_size` writer - Vertical size."]
pub type VsmVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Vertical size."]
    #[inline(always)]
    pub fn vsm_v_size(&self) -> VsmVSizeR {
        VsmVSizeR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Vertical size."]
    #[inline(always)]
    #[must_use]
    pub fn vsm_v_size(&mut self) -> VsmVSizeW<VsmVSizeSpec> {
        VsmVSizeW::new(self, 1)
    }
}
#[doc = "Vertical measure window size\n\nNote: only even values are allowed: vsm_v_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmVSizeSpec;
impl crate::RegisterSpec for VsmVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_v_size::R`](R) reader structure"]
impl crate::Readable for VsmVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_v_size::W`](W) writer structure"]
impl crate::Writable for VsmVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_V_SIZE to value 0"]
impl crate::Resettable for VsmVSizeSpec {
    const RESET_VALUE: u32 = 0;
}
