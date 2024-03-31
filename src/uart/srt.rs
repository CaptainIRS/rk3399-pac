#[doc = "Register `SRT` reader"]
pub type R = crate::R<SrtSpec>;
#[doc = "Register `SRT` writer"]
pub type W = crate::W<SrtSpec>;
#[doc = "Field `SHADOW_RCVR_TRIGGER` reader - Shadow RCVR Trigger.\n\nThis is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])."]
pub type ShadowRcvrTriggerR = crate::BitReader;
#[doc = "Field `SHADOW_RCVR_TRIGGER` writer - Shadow RCVR Trigger.\n\nThis is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])."]
pub type ShadowRcvrTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow RCVR Trigger.\n\nThis is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])."]
    #[inline(always)]
    pub fn shadow_rcvr_trigger(&self) -> ShadowRcvrTriggerR {
        ShadowRcvrTriggerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow RCVR Trigger.\n\nThis is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_rcvr_trigger(&mut self) -> ShadowRcvrTriggerW<SrtSpec> {
        ShadowRcvrTriggerW::new(self, 0)
    }
}
#[doc = "Shadow RCVR Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtSpec;
impl crate::RegisterSpec for SrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srt::R`](R) reader structure"]
impl crate::Readable for SrtSpec {}
#[doc = "`write(|w| ..)` method takes [`srt::W`](W) writer structure"]
impl crate::Writable for SrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRT to value 0"]
impl crate::Resettable for SrtSpec {
    const RESET_VALUE: u32 = 0;
}
