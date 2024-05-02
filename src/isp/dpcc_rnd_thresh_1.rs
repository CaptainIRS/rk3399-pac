#[doc = "Register `DPCC_RND_THRESH_1` reader"]
pub type R = crate::R<DpccRndThresh1Spec>;
#[doc = "Register `DPCC_RND_THRESH_1` writer"]
pub type W = crate::W<DpccRndThresh1Spec>;
#[doc = "Field `RND_THR_1_G` reader - green"]
pub type RndThr1GR = crate::FieldReader;
#[doc = "Field `RND_THR_1_G` writer - green"]
pub type RndThr1GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RND_THR_1_RB` reader - red/blue"]
pub type RndThr1RbR = crate::FieldReader;
#[doc = "Field `RND_THR_1_RB` writer - red/blue"]
pub type RndThr1RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - green"]
    #[inline(always)]
    pub fn rnd_thr_1_g(&self) -> RndThr1GR {
        RndThr1GR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - red/blue"]
    #[inline(always)]
    pub fn rnd_thr_1_rb(&self) -> RndThr1RbR {
        RndThr1RbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - green"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_thr_1_g(&mut self) -> RndThr1GW<DpccRndThresh1Spec> {
        RndThr1GW::new(self, 0)
    }
    #[doc = "Bits 8:15 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_thr_1_rb(&mut self) -> RndThr1RbW<DpccRndThresh1Spec> {
        RndThr1RbW::new(self, 8)
    }
}
#[doc = "Rank Neighbor Difference threshold for set 1\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_thresh_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_thresh_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRndThresh1Spec;
impl crate::RegisterSpec for DpccRndThresh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rnd_thresh_1::R`](R) reader structure"]
impl crate::Readable for DpccRndThresh1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rnd_thresh_1::W`](W) writer structure"]
impl crate::Writable for DpccRndThresh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RND_THRESH_1 to value 0"]
impl crate::Resettable for DpccRndThresh1Spec {
    const RESET_VALUE: u32 = 0;
}
