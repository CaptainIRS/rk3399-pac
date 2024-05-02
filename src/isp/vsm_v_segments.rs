#[doc = "Register `VSM_V_SEGMENTS` reader"]
pub type R = crate::R<VsmVSegmentsSpec>;
#[doc = "Register `VSM_V_SEGMENTS` writer"]
pub type W = crate::W<VsmVSegmentsSpec>;
#[doc = "Field `vsm_v_segments` reader - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in vertical direction."]
pub type VsmVSegmentsR = crate::FieldReader;
#[doc = "Field `vsm_v_segments` writer - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in vertical direction."]
pub type VsmVSegmentsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in vertical direction."]
    #[inline(always)]
    pub fn vsm_v_segments(&self) -> VsmVSegmentsR {
        VsmVSegmentsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in vertical direction."]
    #[inline(always)]
    #[must_use]
    pub fn vsm_v_segments(&mut self) -> VsmVSegmentsW<VsmVSegmentsSpec> {
        VsmVSegmentsW::new(self, 0)
    }
}
#[doc = "Iteration 1 vertical segments\n\nNote: number of 1st iteration sample points = vsm_v_segments + 1 \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_v_segments::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_v_segments::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmVSegmentsSpec;
impl crate::RegisterSpec for VsmVSegmentsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_v_segments::R`](R) reader structure"]
impl crate::Readable for VsmVSegmentsSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_v_segments::W`](W) writer structure"]
impl crate::Writable for VsmVSegmentsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_V_SEGMENTS to value 0"]
impl crate::Resettable for VsmVSegmentsSpec {
    const RESET_VALUE: u32 = 0;
}
