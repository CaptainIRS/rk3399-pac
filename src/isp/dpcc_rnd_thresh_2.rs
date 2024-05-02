#[doc = "Register `DPCC_RND_THRESH_2` reader"]
pub type R = crate::R<DpccRndThresh2Spec>;
#[doc = "Register `DPCC_RND_THRESH_2` writer"]
pub type W = crate::W<DpccRndThresh2Spec>;
#[doc = "Field `RND_THR_2_G` reader - green"]
pub type RndThr2GR = crate::FieldReader;
#[doc = "Field `RND_THR_2_G` writer - green"]
pub type RndThr2GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RND_THR_2_RB` reader - red/blue"]
pub type RndThr2RbR = crate::FieldReader;
#[doc = "Field `RND_THR_2_RB` writer - red/blue"]
pub type RndThr2RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - green"]
    #[inline(always)]
    pub fn rnd_thr_2_g(&self) -> RndThr2GR {
        RndThr2GR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - red/blue"]
    #[inline(always)]
    pub fn rnd_thr_2_rb(&self) -> RndThr2RbR {
        RndThr2RbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - green"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_thr_2_g(&mut self) -> RndThr2GW<DpccRndThresh2Spec> {
        RndThr2GW::new(self, 0)
    }
    #[doc = "Bits 8:15 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_thr_2_rb(&mut self) -> RndThr2RbW<DpccRndThresh2Spec> {
        RndThr2RbW::new(self, 8)
    }
}
#[doc = "Rank Neighbor Difference threshold for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_thresh_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_thresh_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRndThresh2Spec;
impl crate::RegisterSpec for DpccRndThresh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rnd_thresh_2::R`](R) reader structure"]
impl crate::Readable for DpccRndThresh2Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rnd_thresh_2::W`](W) writer structure"]
impl crate::Writable for DpccRndThresh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RND_THRESH_2 to value 0"]
impl crate::Resettable for DpccRndThresh2Spec {
    const RESET_VALUE: u32 = 0;
}
