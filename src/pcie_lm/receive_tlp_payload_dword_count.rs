#[doc = "Register `RECEIVE_TLP_PAYLOAD_DWORD_COUNT` reader"]
pub type R = crate::R<ReceiveTlpPayloadDwordCountSpec>;
#[doc = "Register `RECEIVE_TLP_PAYLOAD_DWORD_COUNT` writer"]
pub type W = crate::W<ReceiveTlpPayloadDwordCountSpec>;
#[doc = "Field `RTPDC` reader - Receive TLP Payload Byte Count \\[RTPDC\\]
Count of TLP payload Dwords received"]
pub type RtpdcR = crate::FieldReader<u32>;
#[doc = "Field `RTPDC` writer - Receive TLP Payload Byte Count \\[RTPDC\\]
Count of TLP payload Dwords received"]
pub type RtpdcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive TLP Payload Byte Count \\[RTPDC\\]
Count of TLP payload Dwords received"]
    #[inline(always)]
    pub fn rtpdc(&self) -> RtpdcR {
        RtpdcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive TLP Payload Byte Count \\[RTPDC\\]
Count of TLP payload Dwords received"]
    #[inline(always)]
    #[must_use]
    pub fn rtpdc(&mut self) -> RtpdcW<ReceiveTlpPayloadDwordCountSpec> {
        RtpdcW::new(self, 0)
    }
}
#[doc = "Receive TLP Payload Dword Count Register Count of TLP payload Dwords received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_tlp_payload_dword_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_tlp_payload_dword_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveTlpPayloadDwordCountSpec;
impl crate::RegisterSpec for ReceiveTlpPayloadDwordCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_tlp_payload_dword_count::R`](R) reader structure"]
impl crate::Readable for ReceiveTlpPayloadDwordCountSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_tlp_payload_dword_count::W`](W) writer structure"]
impl crate::Writable for ReceiveTlpPayloadDwordCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets RECEIVE_TLP_PAYLOAD_DWORD_COUNT to value 0"]
impl crate::Resettable for ReceiveTlpPayloadDwordCountSpec {
    const RESET_VALUE: u32 = 0;
}
