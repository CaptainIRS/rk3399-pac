#[doc = "Register `FC_CTRLQHIGH` reader"]
pub type R = crate::R<FcCtrlqhighSpec>;
#[doc = "Register `FC_CTRLQHIGH` writer"]
pub type W = crate::W<FcCtrlqhighSpec>;
#[doc = "Field `ONHIGHATTENDED` reader - Configures the number of high priority packets or audio sample packets consecutively attended before checking low priority queue status. Valid range is from 5'd1 to 5'd31."]
pub type OnhighattendedR = crate::FieldReader;
#[doc = "Field `ONHIGHATTENDED` writer - Configures the number of high priority packets or audio sample packets consecutively attended before checking low priority queue status. Valid range is from 5'd1 to 5'd31."]
pub type OnhighattendedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Configures the number of high priority packets or audio sample packets consecutively attended before checking low priority queue status. Valid range is from 5'd1 to 5'd31."]
    #[inline(always)]
    pub fn onhighattended(&self) -> OnhighattendedR {
        OnhighattendedR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the number of high priority packets or audio sample packets consecutively attended before checking low priority queue status. Valid range is from 5'd1 to 5'd31."]
    #[inline(always)]
    #[must_use]
    pub fn onhighattended(&mut self) -> OnhighattendedW<FcCtrlqhighSpec> {
        OnhighattendedW::new(self, 0)
    }
}
#[doc = "Configures the number of high priority packets or audio sample packets consecutively attended before checking low priority queue status. Valid range is from 5'd1 to 5'd31.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ctrlqhigh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ctrlqhigh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCtrlqhighSpec;
impl crate::RegisterSpec for FcCtrlqhighSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_ctrlqhigh::R`](R) reader structure"]
impl crate::Readable for FcCtrlqhighSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_ctrlqhigh::W`](W) writer structure"]
impl crate::Writable for FcCtrlqhighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_CTRLQHIGH to value 0x0f"]
impl crate::Resettable for FcCtrlqhighSpec {
    const RESET_VALUE: u8 = 0x0f;
}
