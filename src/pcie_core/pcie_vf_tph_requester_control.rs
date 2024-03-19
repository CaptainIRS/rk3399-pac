#[doc = "Register `PCIE_VF_TPH_REQUESTER_CONTROL` reader"]
pub type R = crate::R<PcieVfTphRequesterControlSpec>;
#[doc = "Register `PCIE_VF_TPH_REQUESTER_CONTROL` writer"]
pub type W = crate::W<PcieVfTphRequesterControlSpec>;
#[doc = "Field `STM` reader - ST Mode \\[STM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The VF_TPH_ST_MODE output of the core reflects the setting of this register field (bits 3:0 for VF 0 and so on). This field can also be written from the local management bus."]
pub type StmR = crate::FieldReader;
#[doc = "Field `STM` writer - ST Mode \\[STM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The VF_TPH_ST_MODE output of the core reflects the setting of this register field (bits 3:0 for VF 0 and so on). This field can also be written from the local management bus."]
pub type StmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRE` reader - TPH Requester Enable \\[TRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b Reserved. 11b Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
pub type TreR = crate::FieldReader;
#[doc = "Field `TRE` writer - TPH Requester Enable \\[TRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b Reserved. 11b Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
pub type TreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R10` reader - Reserved \\[R10\\]
Reserved"]
pub type R10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - ST Mode \\[STM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The VF_TPH_ST_MODE output of the core reflects the setting of this register field (bits 3:0 for VF 0 and so on). This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - TPH Requester Enable \\[TRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b Reserved. 11b Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
    #[inline(always)]
    pub fn tre(&self) -> TreR {
        TreR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - Reserved \\[R10\\]
Reserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - ST Mode \\[STM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The VF_TPH_ST_MODE output of the core reflects the setting of this register field (bits 3:0 for VF 0 and so on). This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> StmW<PcieVfTphRequesterControlSpec> {
        StmW::new(self, 0)
    }
    #[doc = "Bits 8:9 - TPH Requester Enable \\[TRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b Reserved. 11b Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
    #[inline(always)]
    #[must_use]
    pub fn tre(&mut self) -> TreW<PcieVfTphRequesterControlSpec> {
        TreW::new(self, 8)
    }
}
#[doc = "TPH Requester Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_requester_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_tph_requester_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfTphRequesterControlSpec;
impl crate::RegisterSpec for PcieVfTphRequesterControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_tph_requester_control::R`](R) reader structure"]
impl crate::Readable for PcieVfTphRequesterControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_tph_requester_control::W`](W) writer structure"]
impl crate::Writable for PcieVfTphRequesterControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_VF_TPH_REQUESTER_CONTROL to value 0"]
impl crate::Resettable for PcieVfTphRequesterControlSpec {
    const RESET_VALUE: u32 = 0;
}
