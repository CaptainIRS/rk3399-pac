#[doc = "Register `M_VID_1` reader"]
pub type R = crate::R<MVid1Spec>;
#[doc = "Register `M_VID_1` writer"]
pub type W = crate::W<MVid1Spec>;
#[doc = "Field `M_VID_1` reader - M_VID \\[15:8\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
pub type MVid1R = crate::FieldReader;
#[doc = "Field `M_VID_1` writer - M_VID \\[15:8\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
pub type MVid1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - M_VID \\[15:8\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
    #[inline(always)]
    pub fn m_vid_1(&self) -> MVid1R {
        MVid1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - M_VID \\[15:8\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_1(&mut self) -> MVid1W<MVid1Spec> {
        MVid1W::new(self, 0)
    }
}
#[doc = "DP M_VID Configure Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVid1Spec;
impl crate::RegisterSpec for MVid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_vid_1::R`](R) reader structure"]
impl crate::Readable for MVid1Spec {}
#[doc = "`write(|w| ..)` method takes [`m_vid_1::W`](W) writer structure"]
impl crate::Writable for MVid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M_VID_1 to value 0"]
impl crate::Resettable for MVid1Spec {
    const RESET_VALUE: u32 = 0;
}
