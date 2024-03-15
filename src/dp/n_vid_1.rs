#[doc = "Register `N_VID_1` reader"]
pub type R = crate::R<NVid1Spec>;
#[doc = "Register `N_VID_1` writer"]
pub type W = crate::W<NVid1Spec>;
#[doc = "Field `N_VID_1` reader - N_VID\\[15:8\\]"]
pub type NVid1R = crate::FieldReader;
#[doc = "Field `N_VID_1` writer - N_VID\\[15:8\\]"]
pub type NVid1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N_VID\\[15:8\\]"]
    #[inline(always)]
    pub fn n_vid_1(&self) -> NVid1R {
        NVid1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N_VID\\[15:8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn n_vid_1(&mut self) -> NVid1W<NVid1Spec> {
        NVid1W::new(self, 0)
    }
}
#[doc = "DP N_VID Configure Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_vid_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_vid_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVid1Spec;
impl crate::RegisterSpec for NVid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_vid_1::R`](R) reader structure"]
impl crate::Readable for NVid1Spec {}
#[doc = "`write(|w| ..)` method takes [`n_vid_1::W`](W) writer structure"]
impl crate::Writable for NVid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets N_VID_1 to value 0x80"]
impl crate::Resettable for NVid1Spec {
    const RESET_VALUE: u32 = 0x80;
}
