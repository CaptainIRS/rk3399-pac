#[doc = "Register `M_VID_MON` reader"]
pub type R = crate::R<MVidMonSpec>;
#[doc = "Register `M_VID_MON` writer"]
pub type W = crate::W<MVidMonSpec>;
#[doc = "Field `M_VID_MON` reader - This register shows M_VID value which is actually \n\ntransmitted to Rx for monitoring purpose."]
pub type MVidMonR = crate::FieldReader<u32>;
#[doc = "Field `M_VID_MON` writer - This register shows M_VID value which is actually \n\ntransmitted to Rx for monitoring purpose."]
pub type MVidMonW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - This register shows M_VID value which is actually \n\ntransmitted to Rx for monitoring purpose."]
    #[inline(always)]
    pub fn m_vid_mon(&self) -> MVidMonR {
        MVidMonR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register shows M_VID value which is actually \n\ntransmitted to Rx for monitoring purpose."]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_mon(&mut self) -> MVidMonW<MVidMonSpec> {
        MVidMonW::new(self, 0)
    }
}
#[doc = "DP M_VID value monitoring register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_mon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_mon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVidMonSpec;
impl crate::RegisterSpec for MVidMonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_vid_mon::R`](R) reader structure"]
impl crate::Readable for MVidMonSpec {}
#[doc = "`write(|w| ..)` method takes [`m_vid_mon::W`](W) writer structure"]
impl crate::Writable for MVidMonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M_VID_MON to value 0"]
impl crate::Resettable for MVidMonSpec {
    const RESET_VALUE: u32 = 0;
}
