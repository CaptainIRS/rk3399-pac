#[doc = "Register `EMMCCORE_RESP3` reader"]
pub type R = crate::R<EmmccoreResp3Spec>;
#[doc = "Register `EMMCCORE_RESP3` writer"]
pub type W = crate::W<EmmccoreResp3Spec>;
#[doc = "Field `RESP` reader - Response register bit \\[127:98\\]"]
pub type RespR = crate::FieldReader<u32>;
#[doc = "Field `RESP` writer - Response register bit \\[127:98\\]"]
pub type RespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Response register bit \\[127:98\\]"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Response register bit \\[127:98\\]"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<EmmccoreResp3Spec> {
        RespW::new(self, 0)
    }
}
#[doc = "Response register bit \\[127:98\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_resp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_resp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreResp3Spec;
impl crate::RegisterSpec for EmmccoreResp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_resp3::R`](R) reader structure"]
impl crate::Readable for EmmccoreResp3Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_resp3::W`](W) writer structure"]
impl crate::Writable for EmmccoreResp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_RESP3 to value 0"]
impl crate::Resettable for EmmccoreResp3Spec {
    const RESET_VALUE: u32 = 0;
}
