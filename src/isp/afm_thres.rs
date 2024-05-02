#[doc = "Register `AFM_THRES` reader"]
pub type R = crate::R<AfmThresSpec>;
#[doc = "Register `AFM_THRES` writer"]
pub type W = crate::W<AfmThresSpec>;
#[doc = "Field `afm_thres` reader - AF measurement threshold\n\nThis register defines a threshold which can be\n\nused for minimizing the influence of noise in the\n\nmeasurement result.\n\n"]
pub type AfmThresR = crate::FieldReader<u16>;
#[doc = "Field `afm_thres` writer - AF measurement threshold\n\nThis register defines a threshold which can be\n\nused for minimizing the influence of noise in the\n\nmeasurement result.\n\n"]
pub type AfmThresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AF measurement threshold\n\nThis register defines a threshold which can be\n\nused for minimizing the influence of noise in the\n\nmeasurement result.\n\n"]
    #[inline(always)]
    pub fn afm_thres(&self) -> AfmThresR {
        AfmThresR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AF measurement threshold\n\nThis register defines a threshold which can be\n\nused for minimizing the influence of noise in the\n\nmeasurement result.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn afm_thres(&mut self) -> AfmThresW<AfmThresSpec> {
        AfmThresW::new(self, 0)
    }
}
#[doc = "Threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_thres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_thres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmThresSpec;
impl crate::RegisterSpec for AfmThresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_thres::R`](R) reader structure"]
impl crate::Readable for AfmThresSpec {}
#[doc = "`write(|w| ..)` method takes [`afm_thres::W`](W) writer structure"]
impl crate::Writable for AfmThresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_THRES to value 0"]
impl crate::Resettable for AfmThresSpec {
    const RESET_VALUE: u32 = 0;
}
