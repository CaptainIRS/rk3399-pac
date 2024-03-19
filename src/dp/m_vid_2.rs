#[doc = "Register `M_VID_2` reader"]
pub type R = crate::R<MVid2Spec>;
#[doc = "Register `M_VID_2` writer"]
pub type W = crate::W<MVid2Spec>;
#[doc = "Field `M_VID_2` reader - M_VID \\[23:16\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
pub type MVid2R = crate::FieldReader;
#[doc = "Field `M_VID_2` writer - M_VID \\[23:16\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
pub type MVid2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - M_VID \\[23:16\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
    #[inline(always)]
    pub fn m_vid_2(&self) -> MVid2R {
        MVid2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - M_VID \\[23:16\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_2(&mut self) -> MVid2W<MVid2Spec> {
        MVid2W::new(self, 0)
    }
}
#[doc = "DP M_VID Configure Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVid2Spec;
impl crate::RegisterSpec for MVid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_vid_2::R`](R) reader structure"]
impl crate::Readable for MVid2Spec {}
#[doc = "`write(|w| ..)` method takes [`m_vid_2::W`](W) writer structure"]
impl crate::Writable for MVid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M_VID_2 to value 0"]
impl crate::Resettable for MVid2Spec {
    const RESET_VALUE: u32 = 0;
}
