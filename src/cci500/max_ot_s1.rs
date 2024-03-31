#[doc = "Register `MAX_OT_S1` reader"]
pub type R = crate::R<MaxOtS1Spec>;
#[doc = "Register `MAX_OT_S1` writer"]
pub type W = crate::W<MaxOtS1Spec>;
#[doc = "Field `MAX_OT` reader - The maximum number of OTs for the\n\ninterface. This is a combined issuing limit. It\n\nrepresents the maximum number of\n\ntransactions that the upstream master can\n\nissue when the AR and AW channels are\n\nconsidered as one issuing source."]
pub type MaxOtR = crate::BitReader;
#[doc = "Field `MAX_OT` writer - The maximum number of OTs for the\n\ninterface. This is a combined issuing limit. It\n\nrepresents the maximum number of\n\ntransactions that the upstream master can\n\nissue when the AR and AW channels are\n\nconsidered as one issuing source."]
pub type MaxOtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The maximum number of OTs for the\n\ninterface. This is a combined issuing limit. It\n\nrepresents the maximum number of\n\ntransactions that the upstream master can\n\nissue when the AR and AW channels are\n\nconsidered as one issuing source."]
    #[inline(always)]
    pub fn max_ot(&self) -> MaxOtR {
        MaxOtR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The maximum number of OTs for the\n\ninterface. This is a combined issuing limit. It\n\nrepresents the maximum number of\n\ntransactions that the upstream master can\n\nissue when the AR and AW channels are\n\nconsidered as one issuing source."]
    #[inline(always)]
    #[must_use]
    pub fn max_ot(&mut self) -> MaxOtW<MaxOtS1Spec> {
        MaxOtW::new(self, 0)
    }
}
#[doc = "Maximum Outstanding Transactions Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_ot_s1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`max_ot_s1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxOtS1Spec;
impl crate::RegisterSpec for MaxOtS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_ot_s1::R`](R) reader structure"]
impl crate::Readable for MaxOtS1Spec {}
#[doc = "`write(|w| ..)` method takes [`max_ot_s1::W`](W) writer structure"]
impl crate::Writable for MaxOtS1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX_OT_S1 to value 0"]
impl crate::Resettable for MaxOtS1Spec {
    const RESET_VALUE: u32 = 0;
}
