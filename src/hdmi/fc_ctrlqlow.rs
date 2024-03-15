#[doc = "Register `FC_CTRLQLOW` reader"]
pub type R = crate::R<FcCtrlqlowSpec>;
#[doc = "Register `FC_CTRLQLOW` writer"]
pub type W = crate::W<FcCtrlqlowSpec>;
#[doc = "Field `ONLOWATTENDED` reader - Configures the number of low priority packets or null packets consecutively attended before checking high priority queue status or audio samples availability. Valid range is from 5'd1 to 5'd31."]
pub type OnlowattendedR = crate::FieldReader;
#[doc = "Field `ONLOWATTENDED` writer - Configures the number of low priority packets or null packets consecutively attended before checking high priority queue status or audio samples availability. Valid range is from 5'd1 to 5'd31."]
pub type OnlowattendedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Configures the number of low priority packets or null packets consecutively attended before checking high priority queue status or audio samples availability. Valid range is from 5'd1 to 5'd31."]
    #[inline(always)]
    pub fn onlowattended(&self) -> OnlowattendedR {
        OnlowattendedR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the number of low priority packets or null packets consecutively attended before checking high priority queue status or audio samples availability. Valid range is from 5'd1 to 5'd31."]
    #[inline(always)]
    #[must_use]
    pub fn onlowattended(&mut self) -> OnlowattendedW<FcCtrlqlowSpec> {
        OnlowattendedW::new(self, 0)
    }
}
#[doc = "Configures the number of low priority packets or null packets consecutively attended before checking high priority queue status or audio samples availability. Valid range is from 5'd1 to 5'd31.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ctrlqlow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ctrlqlow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCtrlqlowSpec;
impl crate::RegisterSpec for FcCtrlqlowSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_ctrlqlow::R`](R) reader structure"]
impl crate::Readable for FcCtrlqlowSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_ctrlqlow::W`](W) writer structure"]
impl crate::Writable for FcCtrlqlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_CTRLQLOW to value 0x03"]
impl crate::Resettable for FcCtrlqlowSpec {
    const RESET_VALUE: u8 = 0x03;
}
