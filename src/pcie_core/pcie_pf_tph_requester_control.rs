#[doc = "Register `PCIE_PF_TPH_REQUESTER_CONTROL` reader"]
pub type R = crate::R<PciePfTphRequesterControlSpec>;
#[doc = "Register `PCIE_PF_TPH_REQUESTER_CONTROL` writer"]
pub type W = crate::W<PciePfTphRequesterControlSpec>;
#[doc = "Field `CSM` reader - ST Mode \\[CSM\\]\n\nThis field selects the ST mode (000\n\n= No Steering Tag Mode, 001 =\n\nInterrupt Vector Mode, 010 =\n\nDevice-Specific Mode, other values\n\nare reserved). The TPH_ST_MODE\n\noutput of the core reflects the\n\nsetting of this register field. This field\n\ncan also be written from the local\n\nmanagement bus."]
pub type CsmR = crate::FieldReader;
#[doc = "Field `CSM` writer - ST Mode \\[CSM\\]\n\nThis field selects the ST mode (000\n\n= No Steering Tag Mode, 001 =\n\nInterrupt Vector Mode, 010 =\n\nDevice-Specific Mode, other values\n\nare reserved). The TPH_ST_MODE\n\noutput of the core reflects the\n\nsetting of this register field. This field\n\ncan also be written from the local\n\nmanagement bus."]
pub type CsmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRE` reader - TPH Requester Enable \\[CRE\\]\n\nWhen set the Function is allowed to\n\ngenerate requests with Transaction\n\nProcessing Hints. Defined Encodings\n\nare: 00b - Function operating as a\n\nRequester is not permitted to issue\n\nRequests with TPH or Extended TPH.\n\n01b - Function operating as a\n\nRequester is permitted to issue\n\nRequests with TPH and is not\n\npermitted to issue Requests with\n\nExtended TPH. 10b - Reserved. 11b\n\n- Function operating as a Requester\n\nis permitted to issue Requests with\n\nTPH and Extended TPH."]
pub type CreR = crate::FieldReader;
#[doc = "Field `CRE` writer - TPH Requester Enable \\[CRE\\]\n\nWhen set the Function is allowed to\n\ngenerate requests with Transaction\n\nProcessing Hints. Defined Encodings\n\nare: 00b - Function operating as a\n\nRequester is not permitted to issue\n\nRequests with TPH or Extended TPH.\n\n01b - Function operating as a\n\nRequester is permitted to issue\n\nRequests with TPH and is not\n\npermitted to issue Requests with\n\nExtended TPH. 10b - Reserved. 11b\n\n- Function operating as a Requester\n\nis permitted to issue Requests with\n\nTPH and Extended TPH."]
pub type CreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R10` reader - Reserved \\[R10\\]\n\nReserved"]
pub type R10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - ST Mode \\[CSM\\]\n\nThis field selects the ST mode (000\n\n= No Steering Tag Mode, 001 =\n\nInterrupt Vector Mode, 010 =\n\nDevice-Specific Mode, other values\n\nare reserved). The TPH_ST_MODE\n\noutput of the core reflects the\n\nsetting of this register field. This field\n\ncan also be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - TPH Requester Enable \\[CRE\\]\n\nWhen set the Function is allowed to\n\ngenerate requests with Transaction\n\nProcessing Hints. Defined Encodings\n\nare: 00b - Function operating as a\n\nRequester is not permitted to issue\n\nRequests with TPH or Extended TPH.\n\n01b - Function operating as a\n\nRequester is permitted to issue\n\nRequests with TPH and is not\n\npermitted to issue Requests with\n\nExtended TPH. 10b - Reserved. 11b\n\n- Function operating as a Requester\n\nis permitted to issue Requests with\n\nTPH and Extended TPH."]
    #[inline(always)]
    pub fn cre(&self) -> CreR {
        CreR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - Reserved \\[R10\\]\n\nReserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - ST Mode \\[CSM\\]\n\nThis field selects the ST mode (000\n\n= No Steering Tag Mode, 001 =\n\nInterrupt Vector Mode, 010 =\n\nDevice-Specific Mode, other values\n\nare reserved). The TPH_ST_MODE\n\noutput of the core reflects the\n\nsetting of this register field. This field\n\ncan also be written from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CsmW<PciePfTphRequesterControlSpec> {
        CsmW::new(self, 0)
    }
    #[doc = "Bits 8:9 - TPH Requester Enable \\[CRE\\]\n\nWhen set the Function is allowed to\n\ngenerate requests with Transaction\n\nProcessing Hints. Defined Encodings\n\nare: 00b - Function operating as a\n\nRequester is not permitted to issue\n\nRequests with TPH or Extended TPH.\n\n01b - Function operating as a\n\nRequester is permitted to issue\n\nRequests with TPH and is not\n\npermitted to issue Requests with\n\nExtended TPH. 10b - Reserved. 11b\n\n- Function operating as a Requester\n\nis permitted to issue Requests with\n\nTPH and Extended TPH."]
    #[inline(always)]
    #[must_use]
    pub fn cre(&mut self) -> CreW<PciePfTphRequesterControlSpec> {
        CreW::new(self, 8)
    }
}
#[doc = "TPH Requester Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_requester_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_requester_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfTphRequesterControlSpec;
impl crate::RegisterSpec for PciePfTphRequesterControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_tph_requester_control::R`](R) reader structure"]
impl crate::Readable for PciePfTphRequesterControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_tph_requester_control::W`](W) writer structure"]
impl crate::Writable for PciePfTphRequesterControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_TPH_REQUESTER_CONTROL to value 0"]
impl crate::Resettable for PciePfTphRequesterControlSpec {
    const RESET_VALUE: u32 = 0;
}
