#[doc = "Register `PCIE_LM_RECEIVE_CREDIT_LIMIT_0_VC0` reader"]
pub type R = crate::R<PcieLmReceiveCreditLimit0Vc0Spec>;
#[doc = "Register `PCIE_LM_RECEIVE_CREDIT_LIMIT_0_VC0` writer"]
pub type W = crate::W<PcieLmReceiveCreditLimit0Vc0Spec>;
#[doc = "Field `PPC` reader - Posted Payload Credit VC0 \\[PPC\\]\n\nPosted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
pub type PpcR = crate::FieldReader<u16>;
#[doc = "Field `PPC` writer - Posted Payload Credit VC0 \\[PPC\\]\n\nPosted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
pub type PpcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHC` reader - Posted Header Credit VC0 \\[PHC\\]\n\nPosted header credit limit advertised\n\nby the core for VC 0 (in number of\n\npackets)."]
pub type PhcR = crate::FieldReader;
#[doc = "Field `PHC` writer - Posted Header Credit VC0 \\[PHC\\]\n\nPosted header credit limit advertised\n\nby the core for VC 0 (in number of\n\npackets)."]
pub type PhcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NPPC` reader - Non- Posted Payload Credit VC0 \\[NPPC\\]\n\nNon-Posted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
pub type NppcR = crate::FieldReader<u16>;
#[doc = "Field `NPPC` writer - Non- Posted Payload Credit VC0 \\[NPPC\\]\n\nNon-Posted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
pub type NppcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Posted Payload Credit VC0 \\[PPC\\]\n\nPosted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
    #[inline(always)]
    pub fn ppc(&self) -> PpcR {
        PpcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Posted Header Credit VC0 \\[PHC\\]\n\nPosted header credit limit advertised\n\nby the core for VC 0 (in number of\n\npackets)."]
    #[inline(always)]
    pub fn phc(&self) -> PhcR {
        PhcR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:31 - Non- Posted Payload Credit VC0 \\[NPPC\\]\n\nNon-Posted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
    #[inline(always)]
    pub fn nppc(&self) -> NppcR {
        NppcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Posted Payload Credit VC0 \\[PPC\\]\n\nPosted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
    #[inline(always)]
    #[must_use]
    pub fn ppc(&mut self) -> PpcW<PcieLmReceiveCreditLimit0Vc0Spec> {
        PpcW::new(self, 0)
    }
    #[doc = "Bits 12:19 - Posted Header Credit VC0 \\[PHC\\]\n\nPosted header credit limit advertised\n\nby the core for VC 0 (in number of\n\npackets)."]
    #[inline(always)]
    #[must_use]
    pub fn phc(&mut self) -> PhcW<PcieLmReceiveCreditLimit0Vc0Spec> {
        PhcW::new(self, 12)
    }
    #[doc = "Bits 20:31 - Non- Posted Payload Credit VC0 \\[NPPC\\]\n\nNon-Posted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords)."]
    #[inline(always)]
    #[must_use]
    pub fn nppc(&mut self) -> NppcW<PcieLmReceiveCreditLimit0Vc0Spec> {
        NppcW::new(self, 20)
    }
}
#[doc = "Receive Credit Limit Register 0 VC0\n\nNon-Posted payload credit limit\n\nadvertised by the core for VC 0 (in\n\nunits of 4 Dwords).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_credit_limit_0_vc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_receive_credit_limit_0_vc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmReceiveCreditLimit0Vc0Spec;
impl crate::RegisterSpec for PcieLmReceiveCreditLimit0Vc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_receive_credit_limit_0_vc0::R`](R) reader structure"]
impl crate::Readable for PcieLmReceiveCreditLimit0Vc0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_receive_credit_limit_0_vc0::W`](W) writer structure"]
impl crate::Writable for PcieLmReceiveCreditLimit0Vc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_RECEIVE_CREDIT_LIMIT_0_VC0 to value 0x0202_00e0"]
impl crate::Resettable for PcieLmReceiveCreditLimit0Vc0Spec {
    const RESET_VALUE: u32 = 0x0202_00e0;
}
