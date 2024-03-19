#[doc = "Register `N_VID_0` reader"]
pub type R = crate::R<NVid0Spec>;
#[doc = "Register `N_VID_0` writer"]
pub type W = crate::W<NVid0Spec>;
#[doc = "Field `N_VID_0` reader - N_VID\\[7:0\\]
\n\nThe maximum value of M_VID is 0xFFFF in \n\nASYNC mode."]
pub type NVid0R = crate::FieldReader;
#[doc = "Field `N_VID_0` writer - N_VID\\[7:0\\]
\n\nThe maximum value of M_VID is 0xFFFF in \n\nASYNC mode."]
pub type NVid0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N_VID\\[7:0\\]
\n\nThe maximum value of M_VID is 0xFFFF in \n\nASYNC mode."]
    #[inline(always)]
    pub fn n_vid_0(&self) -> NVid0R {
        NVid0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N_VID\\[7:0\\]
\n\nThe maximum value of M_VID is 0xFFFF in \n\nASYNC mode."]
    #[inline(always)]
    #[must_use]
    pub fn n_vid_0(&mut self) -> NVid0W<NVid0Spec> {
        NVid0W::new(self, 0)
    }
}
#[doc = "DP N_VID Configure Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_vid_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_vid_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVid0Spec;
impl crate::RegisterSpec for NVid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_vid_0::R`](R) reader structure"]
impl crate::Readable for NVid0Spec {}
#[doc = "`write(|w| ..)` method takes [`n_vid_0::W`](W) writer structure"]
impl crate::Writable for NVid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets N_VID_0 to value 0"]
impl crate::Resettable for NVid0Spec {
    const RESET_VALUE: u32 = 0;
}
