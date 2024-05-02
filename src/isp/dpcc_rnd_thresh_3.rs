#[doc = "Register `DPCC_RND_THRESH_3` reader"]
pub type R = crate::R<DpccRndThresh3Spec>;
#[doc = "Register `DPCC_RND_THRESH_3` writer"]
pub type W = crate::W<DpccRndThresh3Spec>;
#[doc = "Field `RND_THR_3_G` reader - green"]
pub type RndThr3GR = crate::FieldReader;
#[doc = "Field `RND_THR_3_G` writer - green"]
pub type RndThr3GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RND_THR_3_RB` reader - red/blue"]
pub type RndThr3RbR = crate::FieldReader;
#[doc = "Field `RND_THR_3_RB` writer - red/blue"]
pub type RndThr3RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - green"]
    #[inline(always)]
    pub fn rnd_thr_3_g(&self) -> RndThr3GR {
        RndThr3GR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - red/blue"]
    #[inline(always)]
    pub fn rnd_thr_3_rb(&self) -> RndThr3RbR {
        RndThr3RbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - green"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_thr_3_g(&mut self) -> RndThr3GW<DpccRndThresh3Spec> {
        RndThr3GW::new(self, 0)
    }
    #[doc = "Bits 8:15 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_thr_3_rb(&mut self) -> RndThr3RbW<DpccRndThresh3Spec> {
        RndThr3RbW::new(self, 8)
    }
}
#[doc = "Rank Neighbor Difference threshold for set 3\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_thresh_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_thresh_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRndThresh3Spec;
impl crate::RegisterSpec for DpccRndThresh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rnd_thresh_3::R`](R) reader structure"]
impl crate::Readable for DpccRndThresh3Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rnd_thresh_3::W`](W) writer structure"]
impl crate::Writable for DpccRndThresh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RND_THRESH_3 to value 0"]
impl crate::Resettable for DpccRndThresh3Spec {
    const RESET_VALUE: u32 = 0;
}
