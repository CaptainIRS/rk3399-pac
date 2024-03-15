#[doc = "Register `TPH_REQUESTER_CONTROL` reader"]
pub type R = crate::R<TphRequesterControlSpec>;
#[doc = "Register `TPH_REQUESTER_CONTROL` writer"]
pub type W = crate::W<TphRequesterControlSpec>;
#[doc = "Field `CSM` reader - ST Mode \\[CSM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The TPH_ST_MODE output of the core reflects the setting of this register field. This field can also be written from the local management bus."]
pub type CsmR = crate::FieldReader;
#[doc = "Field `CSM` writer - ST Mode \\[CSM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The TPH_ST_MODE output of the core reflects the setting of this register field. This field can also be written from the local management bus."]
pub type CsmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRE` reader - TPH Requester Enable \\[CRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b - Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b - Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b - Reserved. 11b - Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
pub type CreR = crate::FieldReader;
#[doc = "Field `CRE` writer - TPH Requester Enable \\[CRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b - Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b - Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b - Reserved. 11b - Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
pub type CreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R10` reader - Reserved \\[R10\\]
Reserved"]
pub type R10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - ST Mode \\[CSM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The TPH_ST_MODE output of the core reflects the setting of this register field. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - TPH Requester Enable \\[CRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b - Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b - Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b - Reserved. 11b - Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
    #[inline(always)]
    pub fn cre(&self) -> CreR {
        CreR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - Reserved \\[R10\\]
Reserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - ST Mode \\[CSM\\]
This field selects the ST mode (000 = No Steering Tag Mode, 001 = Interrupt Vector Mode, 010 = Device-Specific Mode, other values are reserved). The TPH_ST_MODE output of the core reflects the setting of this register field. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CsmW<TphRequesterControlSpec> {
        CsmW::new(self, 0)
    }
    #[doc = "Bits 8:9 - TPH Requester Enable \\[CRE\\]
When set the Function is allowed to generate requests with Transaction Processing Hints. Defined Encodings are: 00b - Function operating as a Requester is not permitted to issue Requests with TPH or Extended TPH. 01b - Function operating as a Requester is permitted to issue Requests with TPH and is not permitted to issue Requests with Extended TPH. 10b - Reserved. 11b - Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH."]
    #[inline(always)]
    #[must_use]
    pub fn cre(&mut self) -> CreW<TphRequesterControlSpec> {
        CreW::new(self, 8)
    }
}
#[doc = "TPH Requester Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_requester_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_requester_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TphRequesterControlSpec;
impl crate::RegisterSpec for TphRequesterControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tph_requester_control::R`](R) reader structure"]
impl crate::Readable for TphRequesterControlSpec {}
#[doc = "`write(|w| ..)` method takes [`tph_requester_control::W`](W) writer structure"]
impl crate::Writable for TphRequesterControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPH_REQUESTER_CONTROL to value 0"]
impl crate::Resettable for TphRequesterControlSpec {
    const RESET_VALUE: u32 = 0;
}
