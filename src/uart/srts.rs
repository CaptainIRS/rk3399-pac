#[doc = "Register `SRTS` reader"]
pub type R = crate::R<SrtsSpec>;
#[doc = "Register `SRTS` writer"]
pub type W = crate::W<SrtsSpec>;
#[doc = "Field `SHADOW_REQ_TO_SEND` reader - Shadow Request to Send.\n\nThis is a shadow register for the RTS bit (MCR\\[1\\]), this can be\n\nused to remove the burden of having to performing a read-\n\nmodify-write on the MCR."]
pub type ShadowReqToSendR = crate::BitReader;
#[doc = "Field `SHADOW_REQ_TO_SEND` writer - Shadow Request to Send.\n\nThis is a shadow register for the RTS bit (MCR\\[1\\]), this can be\n\nused to remove the burden of having to performing a read-\n\nmodify-write on the MCR."]
pub type ShadowReqToSendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow Request to Send.\n\nThis is a shadow register for the RTS bit (MCR\\[1\\]), this can be\n\nused to remove the burden of having to performing a read-\n\nmodify-write on the MCR."]
    #[inline(always)]
    pub fn shadow_req_to_send(&self) -> ShadowReqToSendR {
        ShadowReqToSendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Request to Send.\n\nThis is a shadow register for the RTS bit (MCR\\[1\\]), this can be\n\nused to remove the burden of having to performing a read-\n\nmodify-write on the MCR."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_req_to_send(&mut self) -> ShadowReqToSendW<SrtsSpec> {
        ShadowReqToSendW::new(self, 0)
    }
}
#[doc = "Shadow Request to Send\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtsSpec;
impl crate::RegisterSpec for SrtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srts::R`](R) reader structure"]
impl crate::Readable for SrtsSpec {}
#[doc = "`write(|w| ..)` method takes [`srts::W`](W) writer structure"]
impl crate::Writable for SrtsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRTS to value 0"]
impl crate::Resettable for SrtsSpec {
    const RESET_VALUE: u32 = 0;
}
