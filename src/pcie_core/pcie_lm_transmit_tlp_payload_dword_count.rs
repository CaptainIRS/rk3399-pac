#[doc = "Register `PCIE_LM_TRANSMIT_TLP_PAYLOAD_DWORD_COUNT` reader"]
pub type R = crate::R<PcieLmTransmitTlpPayloadDwordCountSpec>;
#[doc = "Register `PCIE_LM_TRANSMIT_TLP_PAYLOAD_DWORD_COUNT` writer"]
pub type W = crate::W<PcieLmTransmitTlpPayloadDwordCountSpec>;
#[doc = "Field `TTPBC` reader - Transmit TLP Payload Byte Count \\[TTPBC\\]
Count of TLPs payload Dwords transmitted"]
pub type TtpbcR = crate::FieldReader<u32>;
#[doc = "Field `TTPBC` writer - Transmit TLP Payload Byte Count \\[TTPBC\\]
Count of TLPs payload Dwords transmitted"]
pub type TtpbcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit TLP Payload Byte Count \\[TTPBC\\]
Count of TLPs payload Dwords transmitted"]
    #[inline(always)]
    pub fn ttpbc(&self) -> TtpbcR {
        TtpbcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit TLP Payload Byte Count \\[TTPBC\\]
Count of TLPs payload Dwords transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn ttpbc(&mut self) -> TtpbcW<PcieLmTransmitTlpPayloadDwordCountSpec> {
        TtpbcW::new(self, 0)
    }
}
#[doc = "Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_tlp_payload_dword_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_tlp_payload_dword_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmTransmitTlpPayloadDwordCountSpec;
impl crate::RegisterSpec for PcieLmTransmitTlpPayloadDwordCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_transmit_tlp_payload_dword_count::R`](R) reader structure"]
impl crate::Readable for PcieLmTransmitTlpPayloadDwordCountSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_transmit_tlp_payload_dword_count::W`](W) writer structure"]
impl crate::Writable for PcieLmTransmitTlpPayloadDwordCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets PCIE_LM_TRANSMIT_TLP_PAYLOAD_DWORD_COUNT to value 0"]
impl crate::Resettable for PcieLmTransmitTlpPayloadDwordCountSpec {
    const RESET_VALUE: u32 = 0;
}
