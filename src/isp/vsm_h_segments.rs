#[doc = "Register `VSM_H_SEGMENTS` reader"]
pub type R = crate::R<VsmHSegmentsSpec>;
#[doc = "Register `VSM_H_SEGMENTS` writer"]
pub type W = crate::W<VsmHSegmentsSpec>;
#[doc = "Field `vsm_h_segments` reader - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in horizontal direction."]
pub type VsmHSegmentsR = crate::FieldReader;
#[doc = "Field `vsm_h_segments` writer - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in horizontal direction."]
pub type VsmHSegmentsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in horizontal direction."]
    #[inline(always)]
    pub fn vsm_h_segments(&self) -> VsmHSegmentsR {
        VsmHSegmentsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - number of 16 point wide segments enclosed by the\n\nfirst iteration sample points in horizontal direction."]
    #[inline(always)]
    #[must_use]
    pub fn vsm_h_segments(&mut self) -> VsmHSegmentsW<VsmHSegmentsSpec> {
        VsmHSegmentsW::new(self, 0)
    }
}
#[doc = "Iteration 1 horizontal segments\n\nNote: number of 1st iteration sample points = vsm_h_segments + 1 \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_h_segments::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_h_segments::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmHSegmentsSpec;
impl crate::RegisterSpec for VsmHSegmentsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_h_segments::R`](R) reader structure"]
impl crate::Readable for VsmHSegmentsSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_h_segments::W`](W) writer structure"]
impl crate::Writable for VsmHSegmentsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_H_SEGMENTS to value 0"]
impl crate::Resettable for VsmHSegmentsSpec {
    const RESET_VALUE: u32 = 0;
}
