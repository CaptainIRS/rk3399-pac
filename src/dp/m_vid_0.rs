#[doc = "Register `M_VID_0` reader"]
pub type R = crate::R<MVid0Spec>;
#[doc = "Register `M_VID_0` writer"]
pub type W = crate::W<MVid0Spec>;
#[doc = "Field `M_VID_0` reader - M_VID \\[7:0\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
pub type MVid0R = crate::FieldReader;
#[doc = "Field `M_VID_0` writer - M_VID \\[7:0\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
pub type MVid0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - M_VID \\[7:0\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
    #[inline(always)]
    pub fn m_vid_0(&self) -> MVid0R {
        MVid0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - M_VID \\[7:0\\]. If FIX_M_VID is 1, this \n\nM_VID is used. Otherwise the M_VID value \n\nwhich chip calculated is used"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_0(&mut self) -> MVid0W<MVid0Spec> {
        MVid0W::new(self, 0)
    }
}
#[doc = "DP M_VID Configure Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVid0Spec;
impl crate::RegisterSpec for MVid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_vid_0::R`](R) reader structure"]
impl crate::Readable for MVid0Spec {}
#[doc = "`write(|w| ..)` method takes [`m_vid_0::W`](W) writer structure"]
impl crate::Writable for MVid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M_VID_0 to value 0"]
impl crate::Resettable for MVid0Spec {
    const RESET_VALUE: u32 = 0;
}
