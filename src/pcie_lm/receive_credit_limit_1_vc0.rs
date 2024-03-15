#[doc = "Register `RECEIVE_CREDIT_LIMIT_1_VC0` reader"]
pub type R = crate::R<ReceiveCreditLimit1Vc0Spec>;
#[doc = "Register `RECEIVE_CREDIT_LIMIT_1_VC0` writer"]
pub type W = crate::W<ReceiveCreditLimit1Vc0Spec>;
#[doc = "Field `NPHCL` reader - Non- Posted Header Credit Limit VC0 \\[NPHCL\\]
Non-Posted header credit limit advertised by the core for VC 0 (in number of packets)."]
pub type NphclR = crate::FieldReader;
#[doc = "Field `NPHCL` writer - Non- Posted Header Credit Limit VC0 \\[NPHCL\\]
Non-Posted header credit limit advertised by the core for VC 0 (in number of packets)."]
pub type NphclW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPC` reader - Completion Payload Credit VC0 \\[CPC\\]
Completion payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
pub type CpcR = crate::FieldReader<u16>;
#[doc = "Field `CPC` writer - Completion Payload Credit VC0 \\[CPC\\]
Completion payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
pub type CpcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `CHC` reader - Completion Header Credit VC0 \\[CHC\\]
Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
pub type ChcR = crate::FieldReader;
#[doc = "Field `CHC` writer - Completion Header Credit VC0 \\[CHC\\]
Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
pub type ChcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Non- Posted Header Credit Limit VC0 \\[NPHCL\\]
Non-Posted header credit limit advertised by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub fn nphcl(&self) -> NphclR {
        NphclR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - Completion Payload Credit VC0 \\[CPC\\]
Completion payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
    #[inline(always)]
    pub fn cpc(&self) -> CpcR {
        CpcR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Completion Header Credit VC0 \\[CHC\\]
Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub fn chc(&self) -> ChcR {
        ChcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non- Posted Header Credit Limit VC0 \\[NPHCL\\]
Non-Posted header credit limit advertised by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    #[must_use]
    pub fn nphcl(&mut self) -> NphclW<ReceiveCreditLimit1Vc0Spec> {
        NphclW::new(self, 0)
    }
    #[doc = "Bits 8:19 - Completion Payload Credit VC0 \\[CPC\\]
Completion payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
    #[inline(always)]
    #[must_use]
    pub fn cpc(&mut self) -> CpcW<ReceiveCreditLimit1Vc0Spec> {
        CpcW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Completion Header Credit VC0 \\[CHC\\]
Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    #[must_use]
    pub fn chc(&mut self) -> ChcW<ReceiveCreditLimit1Vc0Spec> {
        ChcW::new(self, 24)
    }
}
#[doc = "Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_credit_limit_1_vc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_credit_limit_1_vc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveCreditLimit1Vc0Spec;
impl crate::RegisterSpec for ReceiveCreditLimit1Vc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_credit_limit_1_vc0::R`](R) reader structure"]
impl crate::Readable for ReceiveCreditLimit1Vc0Spec {}
#[doc = "`write(|w| ..)` method takes [`receive_credit_limit_1_vc0::W`](W) writer structure"]
impl crate::Writable for ReceiveCreditLimit1Vc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_CREDIT_LIMIT_1_VC0 to value 0x20"]
impl crate::Resettable for ReceiveCreditLimit1Vc0Spec {
    const RESET_VALUE: u32 = 0x20;
}
