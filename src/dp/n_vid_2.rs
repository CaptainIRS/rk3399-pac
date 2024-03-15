#[doc = "Register `N_VID_2` reader"]
pub type R = crate::R<NVid2Spec>;
#[doc = "Register `N_VID_2` writer"]
pub type W = crate::W<NVid2Spec>;
#[doc = "Field `N_VID_2` reader - N_VID\\[23:16\\]"]
pub type NVid2R = crate::FieldReader;
#[doc = "Field `N_VID_2` writer - N_VID\\[23:16\\]"]
pub type NVid2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N_VID\\[23:16\\]"]
    #[inline(always)]
    pub fn n_vid_2(&self) -> NVid2R {
        NVid2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N_VID\\[23:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn n_vid_2(&mut self) -> NVid2W<NVid2Spec> {
        NVid2W::new(self, 0)
    }
}
#[doc = "DP N_VID Configure Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_vid_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_vid_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVid2Spec;
impl crate::RegisterSpec for NVid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_vid_2::R`](R) reader structure"]
impl crate::Readable for NVid2Spec {}
#[doc = "`write(|w| ..)` method takes [`n_vid_2::W`](W) writer structure"]
impl crate::Writable for NVid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets N_VID_2 to value 0"]
impl crate::Resettable for NVid2Spec {
    const RESET_VALUE: u32 = 0;
}
