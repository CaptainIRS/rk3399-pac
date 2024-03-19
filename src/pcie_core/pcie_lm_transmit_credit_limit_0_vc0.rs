#[doc = "Register `PCIE_LM_TRANSMIT_CREDIT_LIMIT_0_VC0` reader"]
pub type R = crate::R<PcieLmTransmitCreditLimit0Vc0Spec>;
#[doc = "Field `PPC` reader - Posted Payload Credit VC0 \\[PPC\\]\n\nPosted payload credit limit received\n\nby the core for this link (in units of 4\n\nDwords)."]
pub type PpcR = crate::FieldReader<u16>;
#[doc = "Field `PHC` reader - Posted Header Credit VC0 \\[PHC\\]\n\nPosted header credit limit received\n\nby the core for this link (in number\n\nof packets)."]
pub type PhcR = crate::FieldReader;
#[doc = "Field `NPPC` reader - Non- Posted Payload Credit VC0 \\[NPPC\\]\n\nNon-Posted payload credit limit\n\nreceived by the core for Link 0 (in\n\nunits of 4 Dwords)."]
pub type NppcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Posted Payload Credit VC0 \\[PPC\\]\n\nPosted payload credit limit received\n\nby the core for this link (in units of 4\n\nDwords)."]
    #[inline(always)]
    pub fn ppc(&self) -> PpcR {
        PpcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Posted Header Credit VC0 \\[PHC\\]\n\nPosted header credit limit received\n\nby the core for this link (in number\n\nof packets)."]
    #[inline(always)]
    pub fn phc(&self) -> PhcR {
        PhcR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:31 - Non- Posted Payload Credit VC0 \\[NPPC\\]\n\nNon-Posted payload credit limit\n\nreceived by the core for Link 0 (in\n\nunits of 4 Dwords)."]
    #[inline(always)]
    pub fn nppc(&self) -> NppcR {
        NppcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Transmit Credit Limit Register 0 VC0\n\nNon-Posted payload credit limit\n\nreceived by the core for Link 0 (in\n\nunits of 4 Dwords).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_limit_0_vc0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmTransmitCreditLimit0Vc0Spec;
impl crate::RegisterSpec for PcieLmTransmitCreditLimit0Vc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_transmit_credit_limit_0_vc0::R`](R) reader structure"]
impl crate::Readable for PcieLmTransmitCreditLimit0Vc0Spec {}
#[doc = "`reset()` method sets PCIE_LM_TRANSMIT_CREDIT_LIMIT_0_VC0 to value 0"]
impl crate::Resettable for PcieLmTransmitCreditLimit0Vc0Spec {
    const RESET_VALUE: u32 = 0;
}
